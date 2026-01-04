#!/usr/bin/env python3
"""
verify_ingestion.py
Verifies that the data in SurrealDB matches the source files.
"""
import requests
import json
import os
import xml.etree.ElementTree as ET

# Manual .env loader since pip is unavailable
def load_env_file(filepath):
    if not os.path.exists(filepath):
        return
    with open(filepath, 'r') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'):
                continue
            if '=' in line:
                key, val = line.split('=', 1)
                os.environ[key.strip()] = val.strip()

# Load env vars from project root
BASE_DIR = os.path.dirname(__file__)
PROJECT_ROOT = os.path.dirname(BASE_DIR)
load_env_file(os.path.join(PROJECT_ROOT, ".env"))

DB_URL = "http://localhost:8000/sql"
DB_NS = os.getenv("DB_NS", "idc")
DB_DB = os.getenv("DB_DB", "main")
DB_AUTH = (os.getenv("DB_USER", "root"), os.getenv("DB_PASS", "root"))

BASE_DIR = os.path.dirname(__file__)
DATA_DIR = os.path.join(BASE_DIR, "data")
HADITH_DIR = os.path.join(DATA_DIR, "hadith-api")

def query_db(sql):
    headers = {'Accept': 'application/json', 'NS': DB_NS, 'DB': DB_DB}
    response = requests.post(DB_URL, data=sql, headers=headers, auth=DB_AUTH)
    if response.status_code != 200:
        print(f"Error querying DB: {response.text}")
        return None
    return response.json()

def get_db_counts():
    print("🔍 Querying SurrealDB for counts per table...")
    sql = """
    USE NS idc;
    USE DB main;
    SELECT count() FROM quran_verse GROUP ALL;
    SELECT count() FROM hadith GROUP ALL;
    SELECT count() FROM root_word GROUP ALL;
    SELECT count(), collection FROM hadith GROUP BY collection;
    """
    results = query_db(sql)
    
    counts = {}
    if not results:
        return counts

    print(f"DEBUG: Raw DB Results: {json.dumps(results, indent=2)}")
        
    # Result 0: USE NS
    # Result 1: USE DB
    # Result 2: quran_verse
    try:
        if results[2]['status'] == 'OK':
            quran_res = results[2]['result']
            counts['quran_verse'] = quran_res[0]['count'] if quran_res else 0
        else:
            print(f"Error querying quran_verse: {results[2]}")
            counts['quran_verse'] = 0
    except (IndexError, KeyError):
        counts['quran_verse'] = 0

    # Result 3: hadith (Get breakdown)
    # Query for breakdown
    try:
        if results[3]['status'] == 'OK':
             # This was count() group all.
             hadith_res = results[3]['result']
             counts['hadith'] = hadith_res[0]['count'] if hadith_res else 0
        else:
             counts['hadith'] = 0
             
        # Breakdown query (Result 5)
        if len(results) > 5 and results[5]['status'] == 'OK':
             counts['hadith_breakdown'] = results[5]['result']
        else:
             counts['hadith_breakdown'] = []
    except (IndexError, KeyError):
        counts['hadith'] = 0
        counts['hadith_breakdown'] = []

    # Result 4: root_word
    try:
        if results[4]['status'] == 'OK':
            root_res = results[4]['result']
            counts['root_word'] = root_res[0]['count'] if root_res else 0
        else:
            print(f"Error querying root_word: {results[4]}")
            counts['root_word'] = 0
    except (IndexError, KeyError):
        counts['root_word'] = 0
        
    return counts

def get_source_quran_count():
    xml_path = os.path.join(DATA_DIR, "quran-data.xml")
    if not os.path.exists(xml_path):
        return 0
    tree = ET.parse(xml_path)
    return sum(int(s.get('ayas')) for s in tree.findall('.//sura'))

def get_source_hadith_counts():
    total = 0
    
    # Nawawi 40
    nawawi_path = os.path.join(DATA_DIR, "hadith40.json")
    if os.path.exists(nawawi_path):
        with open(nawawi_path, 'r') as f:
            data = json.load(f)
            total += len(data.get('hadiths', []))
            
    # Bukhari
    bukhari_path = os.path.join(HADITH_DIR, "bukhari.json")
    if os.path.exists(bukhari_path):
        with open(bukhari_path, 'r') as f:
            data = json.load(f)
            total += len(data.get('hadiths', []))
            
    # Muslim
    muslim_path = os.path.join(HADITH_DIR, "muslim.json")
    if os.path.exists(muslim_path):
        with open(muslim_path, 'r') as f:
            data = json.load(f)
            total += len(data.get('hadiths', []))
            
    return total

def main():
    print("========================================")
    print("   AL-MIZAN INGESTION VERIFICATION")
    print("========================================")
    
    # 1. Get DB Counts
    db_counts = get_db_counts()
    print(f"\n[Database Status]")
    print(f"Connected to: {DB_URL} ({DB_NS}/{DB_DB})")
    print(f"Quran Verses: {db_counts.get('quran_verse', 'N/A')}")
    print(f"Hadiths:      {db_counts.get('hadith', 'N/A')}")
    print(f"Root Words:   {db_counts.get('root_word', 'N/A')}")

    # 2. Get Source Counts
    quran_src = get_source_quran_count()
    hadith_src = get_source_hadith_counts()
    
    print(f"\n[Source File Analysis]")
    print(f"Quran Verses (Tanzil): {quran_src}")
    print(f"Hadiths (Total):       {hadith_src}")
    
    # 3. Discrepancy Report
    print(f"\n[Verification Report]")
    
    # Quran
    diff_q = db_counts.get('quran_verse', 0) - quran_src
    if diff_q == 0:
        print("✅ QURAN: MATCH")
    else:
        print(f"❌ QURAN: MISMATCH ({diff_q:+})")
        
    # Hadith
    diff_h = db_counts.get('hadith', 0) - hadith_src
    if diff_h == 0:
        print("✅ HADITH: MATCH")
    else:
        print(f"⚠️ HADITH: DIFFERENCE ({diff_h:+})")
        print("   Breakdown by Collection (DB):")
        for b in db_counts.get('hadith_breakdown', []):
            print(f"   - {b.get('collection', 'Unknown')}: {b.get('count', 0)}")
        print("   (Note: This might be expected if not all collections are ingested yet)")

def check_schema():
    print("\n[Schema Check]")
    sql = """
    USE NS idc;
    USE DB main;
    INFO FOR TABLE hadith;
    """
    results = query_db(sql)
    print(json.dumps(results, indent=2))

if __name__ == "__main__":
    main()
    # check_schema()
