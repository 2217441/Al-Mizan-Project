import json
import os

import json
import os

def extract_hadith(json_path, output_path):
    print(f"Extracting Hadith Data from {json_path}...")
    
    if not os.path.exists(json_path):
        print(f"ERROR: JSON Source {json_path} not found. Please place 'hadith_collection.json' in database/ folder.")
        return

    with open(json_path, 'r') as f:
        hadith_collection = json.load(f)

    surql_statements = []
    surql_statements.append("-- HADITH INGESTION START")
    surql_statements.append("BEGIN TRANSACTION;")

    for h in hadith_collection:
        # 1. Create Hadith Node (The Text/Matn)
        hadith_id = f"hadith:{h['collection']}_{h['number']}"
        matn_ar = h['matn_ar'].replace("'", "\\'")
        
        # Note: 'grading' and 'matn_en' removed from Hadith node as per New Ontology.
        statement_hadith = f"CREATE {hadith_id} SET \n" \
                           f"    collection = '{h['collection']}', \n" \
                           f"    hadith_number = {h['number']}, \n" \
                           f"    matn_ar = '{matn_ar}', \n" \
                           f"    mutability = 'CONSTANT';"
        surql_statements.append(statement_hadith)

        # 2. Create Translation Node (If exists)
        if 'matn_en' in h and h['matn_en']:
            matn_en = h['matn_en'].replace("'", "\\'")
            trans_id = f"translation:{h['collection']}_{h['number']}_en"
            
            statement_trans = f"CREATE {trans_id} SET \n" \
                              f"    text_en = '{matn_en}', \n" \
                              f"    language = 'en', \n" \
                              f"    translator = 'Standard Version';" 
            surql_statements.append(statement_trans)
            
            # 3. Link Hadith -> Translation
            surql_statements.append(f"RELATE {hadith_id}->translated_as->{trans_id};")

    surql_statements.append("COMMIT TRANSACTION;")
    surql_statements.append("-- HADITH INGESTION END")

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("\n".join(surql_statements))

    print(f"Generated {output_path} with {len(hadith_collection)} narrations.")

if __name__ == "__main__":
    os.makedirs("output", exist_ok=True)
    INPUT_JSON = "../database/hadith_collection.json"
    OUTPUT_SURQL = "output/hadith_nodes.surql"
    extract_hadith(INPUT_JSON, OUTPUT_SURQL)
