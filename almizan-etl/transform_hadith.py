#!/usr/bin/env python3
"""
transform_hadith.py

Stage 2 of the Al-Mizan ETL Pipeline.

Responsibility:
1. Parse massive Hadith JSON collections (Bukhari, Muslim).
2. Sanitize Hadith numbers (handling decimals like 402.2).
3. Generate 'hadith_collections.surql'.

Note:
- Uses 'hadith_number' (float) to maintain strict ordering.
- Grade information is extracted where available.

Inputs:
- data/hadith-api/bukhari.json
- data/hadith-api/muslim.json

Outputs:
- output/hadith_collections.surql
"""
import json
import os
import re

DATA_DIR = os.path.join(os.path.dirname(__file__), "data", "hadith-api")
OUTPUT_DIR = os.path.join(os.path.dirname(__file__), "output")

def transform_collection(collection_name: str, json_path: str) -> list:
    """Transform a hadith collection JSON to SurQL statements"""
    with open(json_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    statements = []
    hadiths = data.get('hadiths', [])
    
    print(f"📖 Processing {collection_name}: {len(hadiths)} hadiths")
    
    for h in hadiths:
        hadith_num = h.get('hadithnumber', 0)
        text = h.get('text', '')
        book = h.get('reference', {}).get('book', 0)
        grades = h.get('grades', [])
        
        # Determine grade if available
        grade = None
        if grades:
            grade = grades[0].get('grade', None)
        
        # Sanitize hadith number (some have decimals like 402.2)
        hadith_num_str = str(hadith_num).replace('.', '_')
        
        # Create unique ID: collection_booknumber_hadithnumber
        record_id = f"{collection_name}_{book}_{hadith_num_str}"
        
        # Use json.dumps for safe string serialization
        stmt = f"CREATE hadith:{record_id} CONTENT {{"
        stmt += f"collection: '{collection_name}', "
        stmt += f"hadith_number: {hadith_num}, "
        stmt += f"book_number: {book}, "
        stmt += f"matn_en: {json.dumps(text, ensure_ascii=False)}"
        if grade:
            stmt += f", grade: {json.dumps(grade, ensure_ascii=False)}"
        stmt += "};"
        
        statements.append(stmt)
    
    return statements

def main():
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    
    all_statements = []
    
    # Process available collections
    collections = [
        ('bukhari', os.path.join(DATA_DIR, 'bukhari.json')),
        ('muslim', os.path.join(DATA_DIR, 'muslim.json')),
    ]
    
    for name, path in collections:
        if os.path.exists(path):
            stmts = transform_collection(name, path)
            all_statements.extend(stmts)
    
    # Write to output - no transactions
    output_path = os.path.join(OUTPUT_DIR, "hadith_collections.surql")
    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("-- Hadith Collections (Bukhari, Muslim)\n")
        f.write("-- Generated from fawazahmed0/hadith-api\n\n")
        
        for stmt in all_statements:
            f.write(stmt + "\n")
    
    print(f"\n✅ Generated: {output_path}")
    print(f"   Total hadiths: {len(all_statements)}")

if __name__ == "__main__":
    main()
