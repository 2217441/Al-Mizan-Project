import requests
import json
import sys
import os

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

# Load env vars
BASE_DIR = os.path.dirname(__file__)
PROJECT_ROOT = os.path.dirname(BASE_DIR)
load_env_file(os.path.join(PROJECT_ROOT, ".env"))

# Configuration
DB_URL = "http://localhost:8000/sql"
HEADERS = {"Accept": "application/json", "NS": os.getenv("DB_NS", "idc"), "DB": os.getenv("DB_DB", "main")}
AUTH = (os.getenv("DB_USER", "root"), os.getenv("DB_PASS", "root"))

def run_query(sql):
    try:
        # Explicitly adding USE NS and USE DB to ensure context
        full_sql = f"USE NS idc; USE DB main; {sql}"
        response = requests.post(DB_URL, data=full_sql, auth=AUTH, headers={"Accept": "application/json"})
        response.raise_for_status()
        return response.json()
    except Exception as e:
        print(f"❌ Error running query: {e}")
        return None

def check_integrity():
    print("🕸️  Starting Graph Integrity Check...")
    print("=" * 40)

    # 1. Check for Orphan Verses (Verses with NO roots)
    # We count verses where the outgoing edges 'has_root' is empty.
    # Note: efficient way in Surreal is just referencing the edge.
    print("🔍 Checking for Orphan Verses (Verses with 0 roots)...")
    q_orphan_verses = "SELECT count() FROM quran_verse WHERE array::len(->has_root) = 0 GROUP ALL;"
    res_orphan = run_query(q_orphan_verses)
    
    orphan_count = 0
    if res_orphan:
        # Result index 2 because index 0 and 1 are USE NS/DB responses
        result_data = res_orphan[2].get('result', [])
        if result_data:
            orphan_count = result_data[0].get('count', 0)
    
    if orphan_count == 0:
        print(f"   ✅ All verses have roots! (0 orphans)")
    else:
        print(f"   ⚠️  Found {orphan_count} verses with NO roots linked.")

    # 2. Check for Orphan Roots (Roots that are never used)
    print("\n🔍 Checking for Orphan Roots (Roots with 0 verses)...")
    q_orphan_roots = "SELECT count() FROM root_word WHERE array::len(<-has_root) = 0 GROUP ALL;"
    res_roots = run_query(q_orphan_roots)

    orphan_root_count = 0
    if res_roots:
        result_data = res_roots[2].get('result', [])
        if result_data:
            orphan_root_count = result_data[0].get('count', 0)

    if orphan_root_count == 0:
        print(f"   ✅ All roots are connected to verses! (0 orphans)")
    else:
        print(f"   ⚠️  Found {orphan_root_count} roots that are not linked to any verse.")

    # 3. Graph Density
    print("\n📊 Calculating Graph Density...")
    # Get total verses
    q_stats = "SELECT count() FROM quran_verse GROUP ALL; SELECT count() FROM has_root GROUP ALL;"
    res_stats = run_query(q_stats)
    print(f"DEBUG REF: {json.dumps(res_stats, indent=2)}")
    
    total_verses = 0
    total_edges = 0
    
    if res_stats:
        total_verses = res_stats[2]['result'][0]['count']
        total_edges = res_stats[3]['result'][0]['count']
    
    if total_verses > 0:
        avg = total_edges / total_verses
        print(f"   Total Edges (Verse->Root): {total_edges}")
        print(f"   Total Verses: {total_verses}")
        print(f"   🔗 Connection Density: {avg:.2f} roots per verse (Avg)")
    else:
        print("   ❌ No verses found to calculate density.")

if __name__ == "__main__":
    check_integrity()
