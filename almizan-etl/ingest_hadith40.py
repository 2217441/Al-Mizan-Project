"""
Al-Mizan ETL: Ingest 40 Nawawi Hadiths into SurrealDB
This script reads the hadith40.json file and generates SurrealQL statements.
"""
import json

INPUT_FILE = "almizan-etl/data/hadith40.json"
OUTPUT_FILE = "almizan-etl/output/hadith40.surql"

def main():
    print(f"Reading {INPUT_FILE}...")
    
    with open(INPUT_FILE, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    hadiths = data.get("hadiths", [])
    print(f"Found {len(hadiths)} hadiths")
    
    with open(OUTPUT_FILE, 'w', encoding='utf-8') as out:
        out.write("-- 40 Nawawi Hadiths Import\n")
        out.write("-- Source: sunnah.com / hadith40.json\n\n")
        
        for h in hadiths:
            # Correct structure: { "id": ..., "idInBook": ..., "arabic": "...", "english": { "narrator": "...", "text": "..." } }
            hadith_id = h.get("idInBook", h.get("id", 0))
            
            # Arabic text is at top level
            arabic_text = h.get("arabic", "")
            
            # English is a nested dict
            english_data = h.get("english", {})
            narrator = english_data.get("narrator", "")
            text_en = english_data.get("text", "")
            
            # Truncate for graph display (first 150 chars)
            display_text = text_en[:150]
            if len(text_en) > 150:
                display_text += "..."
            
            # Use json.dumps for safe string serialization
            out.write(f"CREATE hadith_nawawi:{hadith_id} SET ")
            out.write(f"ref_no = {hadith_id}, ")
            out.write(f'body_ar = {json.dumps(arabic_text)}, ')
            out.write(f'narrator = {json.dumps(narrator)}, ')
            out.write(f'body_en = {json.dumps(text_en)}, ')
            out.write(f'display_text = {json.dumps(display_text)}, ')
            out.write(f"collection = 'nawawi40', ")
            out.write(f"source = 'sunnah.com';\n")
        
        out.write("\n-- Create edges: Prophet Muhammad -> Hadith (narrated)\n")
        for h in hadiths:
            hadith_id = h.get("idInBook", h.get("id", 0))
            out.write(f"RELATE prophet:muhammad->narrated->hadith_nawawi:{hadith_id};\n")
    
    print(f"Done. Generated {len(hadiths)} hadith records.")
    print(f"Output: {OUTPUT_FILE}")

if __name__ == "__main__":
    main()
