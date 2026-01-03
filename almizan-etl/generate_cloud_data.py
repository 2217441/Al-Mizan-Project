#!/usr/bin/env python3
"""
generate_cloud_test_data.py

Generates partial SQL files for Surrealist Cloud testing (Free Tier).
- Juz 30 (Surahs 78-114)
- 40 Hadith Nawawi

Usage: python3 generate_cloud_test_data.py
"""
import os
import json
import xml.etree.ElementTree as ET

BASE_DIR = os.path.dirname(__file__)
DATA_DIR = os.path.join(BASE_DIR, "data")
OUTPUT_DIR = os.path.join(BASE_DIR, "output")

def escape_surql(text: str) -> str:
    if text is None: return ""
    return text.replace("\\", "\\\\").replace("'", "\\'")

def get_juz_number(surah_num):
    # Simplified Juz 30 detection (Surah 78+)
    if surah_num >= 78:
        return 30
    return 0 # We won't strictly calculate others for this test script

def remove_diacritics(text):
    import re
    # 1. Remove Tashkeel (diacritics)
    text = re.sub(r'[\u064B-\u065F\u0670\u06D6-\u06DC\u06DF-\u06E8\u06EA-\u06ED]', '', text)
    # 2. Normalize Alifs (Wasla, Hamza, Madda -> Bare Alif)
    # \u0622(آ) \u0623(أ) \u0625(إ) \u0671(ٱ) -> \u0627(ا)
    text = re.sub(r'[\u0622\u0623\u0625\u0671]', '\u0627', text)
    return text

def generate_juz30():
    print("Generating Juz 30 SQL...")
    
    # 1. Parse Metadata
    tree = ET.parse(os.path.join(DATA_DIR, "quran-data.xml"))
    surahs = {}
    for s in tree.getroot().findall('.//sura'):
        idx = int(s.get('index'))
        surahs[idx] = {
            'ayas': int(s.get('ayas')),
            'place': 'Makkah' if s.get('type') == 'Meccan' else 'Madinah'
        }

    # 2. Read Text
    with open(os.path.join(DATA_DIR, "quran-uthmani.xml"), 'r') as f:
        lines = [l.strip() for l in f if l.strip()]

    statements = []
    statements.append("-- JUZ 30 DATA (Surahs 78-114)")
    statements.append("BEGIN TRANSACTION;")
    
    line_idx = 0
    # Tanzil file order is 1..114
    for surah_num in range(1, 115):
        meta = surahs.get(surah_num)
        count = meta['ayas']
        
        for ayah_num in range(1, count + 1):
            if line_idx < len(lines):
                text = lines[line_idx]
                line_idx += 1
                
                # Filter for Juz 30 only
                if surah_num >= 78:
                    text_ar = escape_surql(text)
                    text_simple = escape_surql(remove_diacritics(text))
                    verse_id = f"quran_verse:{surah_num}_{ayah_num}"
                    
                    stmt = f"""UPSERT {verse_id} SET
    text_uthmani = '{text_ar}',
    text_simple = '{text_simple}',
    surah_number = {surah_num},
    ayah_number = {ayah_num},
    juz_number = 30,
    revelation_place = '{meta['place']}',
    mutability = 'CONSTANT';"""
                    statements.append(stmt)
                    
                    # Add narrated_by relationship (Allah -> Gabriel -> Muhammad -> Verse)
                    # We assume Prophet Muhammad narrated all of them
                    stmt_rel = f"RELATE prophet:muhammad->narrated_quran->{verse_id};"
                    statements.append(stmt_rel)
                    
                    # SAMPLE: Link Surah 112 (Ikhlas) to Hadith 1 for testing graph traversal
                    if surah_num == 112 and ayah_num == 1:
                         # Link to Hadith 1 (Nawawi) just for testing "explains" edge
                         statements.append(f"RELATE hadith:nawawi_1->explains->{verse_id};")
    
    statements.append("COMMIT TRANSACTION;")
    
    out_path = os.path.join(OUTPUT_DIR, "cloud_juz30.surql")
    with open(out_path, 'w') as f:
        f.write("\n".join(statements))
    print(f"✅ Saved {out_path} ({len(statements)} lines)")

def generate_hadith40():
    print("Generating Hadith 40 SQL...")
    
    with open(os.path.join(DATA_DIR, "hadith40.json"), 'r') as f:
        data = json.load(f)
        
    statements = []
    statements.append("-- 40 HADITH NAWAWI")
    statements.append("BEGIN TRANSACTION;")
    
    for h in data.get('hadiths', []):
        id_val = h.get('idInBook', h.get('id'))
        matn_ar = escape_surql(h.get('arabic'))
        
        # Simple English extraction
        eng = h.get('english', {})
        if isinstance(eng, dict):
            matn_en = escape_surql(eng.get('text', ''))
        else:
            matn_en = escape_surql(str(eng))
            
        hid = f"hadith:nawawi_{id_val}"
        
        stmt = f"""UPSERT {hid} SET
    collection = 'Nawawi 40',
    hadith_number = {id_val},
    matn_ar = '{matn_ar}',
    matn_en = '{matn_en}',
    hadith_type = 'nabawi';"""
        statements.append(stmt)
        
    statements.append("COMMIT TRANSACTION;")

    out_path = os.path.join(OUTPUT_DIR, "cloud_hadith40.surql")
    with open(out_path, 'w') as f:
        f.write("\n".join(statements))
    print(f"✅ Saved {out_path}")

if __name__ == "__main__":
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    generate_juz30()
    generate_hadith40()
