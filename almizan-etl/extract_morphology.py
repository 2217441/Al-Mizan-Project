import os
import re

def extract_morphology(input_path, output_path):
    print(f"Extracting Morphology from {input_path}...")
    
    # Store unique connections to avoid duplicates: (root, surah, ayah)
    root_occurrences = set()
    unique_roots = set()

    with open(input_path, 'r', encoding='utf-8') as f:
        for line in f:
            if line.startswith('#') or not line.strip():
                continue
            
            # Format: LOCATION FORM TAG FEATURES
            # Example: (1:1:1:2) somi N STEM|POS:N|LEM:{som|ROOT:smw|M|GEN
            parts = line.split('\t')
            if len(parts) < 4:
                continue
            
            location = parts[0] # (1:1:1:2)
            features = parts[3] # STEM|POS:N|LEM:{som|ROOT:smw|M|GEN
            
            # Extract Root
            root_match = re.search(r'ROOT:([^|]+)', features)
            if root_match:
                root = root_match.group(1)
                
                # Extract Surah/Ayah from Location (S:A:W:Seg)
                loc_match = re.search(r'\((\d+):(\d+):', location)
                if loc_match:
                    surah = loc_match.group(1)
                    ayah = loc_match.group(2)
                    
                    root_occurrences.add((root, surah, ayah))
                    unique_roots.add(root)

    print(f"Found {len(unique_roots)} unique roots.")
    print(f"Found {len(root_occurrences)} root-verse connections.")

    # Generate SurQL
    statements = []
    statements.append("-- MORPHOLOGY INGESTION START")
    statements.append("BEGIN TRANSACTION;")

    # 1. Create Root Nodes
    for root in sorted(unique_roots):
        # Escape special chars if any
        clean_root = root.replace("'", "") 
        statements.append(f"CREATE root:`{clean_root}` SET label = '{clean_root}', types = ['root'];")

    # 2. Create Relations (Root -> Verse)
    # Note: We assume verse nodes (verse:1_1) are created by extract_quran.py
    for root, surah, ayah in sorted(root_occurrences):
        clean_root = root.replace("'", "")
        # Edge: root:smw -> appears_in -> verse:1_1
        # Wrap root ID in backticks to handle special chars like $
        statements.append(f"RELATE root:`{clean_root}`->appears_in->verse:{surah}_{ayah};")

    statements.append("COMMIT TRANSACTION;")
    statements.append("-- MORPHOLOGY INGESTION END")

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("\n".join(statements))
    
    print(f"Written to {output_path}")

if __name__ == "__main__":
    INPUT_FILE = "../database/quranic-corpus-morphology-0.4.txt"
    OUTPUT_FILE = "output/morphology.surql"
    os.makedirs("output", exist_ok=True)
    
    if os.path.exists(INPUT_FILE):
        extract_morphology(INPUT_FILE, OUTPUT_FILE)
    else:
        print(f"Error: {INPUT_FILE} not found.")
