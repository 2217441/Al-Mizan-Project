import json
import os

def extract_hadith(output_path):
    print("Extracting Hadith Data...")
    
    # Mock Data: 40 Hadith Nawawi style
    mock_hadith = [
        {
            "collection": "bukhari",
            "number": 1,
            "matn_ar": "إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ",
            "matn_en": "Actions are but by intentions.",
            "grading": "Sahih"
        },
        {
            "collection": "mawdu_collection",
            "number": 999,
            "matn_ar": "FAKE TEXT",
            "matn_en": "This is a fabricated hadith for testing the Dajjal Filter.",
            "grading": "Mawdu"
        }
    ]

    surql_statements = []
    surql_statements.append("-- HADITH INGESTION START")
    surql_statements.append("BEGIN TRANSACTION;")

    for h in mock_hadith:
        hadith_id = f"hadith:{h['collection']}_{h['number']}"
        matn_ar = h['matn_ar'].replace("'", "\\'")
        matn_en = h['matn_en'].replace("'", "\\'")
        
        statement = f"CREATE {hadith_id} SET \n" \
                    f"    collection = '{h['collection']}', \n" \
                    f"    hadith_number = {h['number']}, \n" \
                    f"    matn_ar = '{matn_ar}', \n" \
                    f"    matn_en = '{matn_en}', \n" \
                    f"    grading = '{h['grading']}', \n" \
                    f"    mutability = 'CONSTANT';"
        
        surql_statements.append(statement)

    surql_statements.append("COMMIT TRANSACTION;")
    surql_statements.append("-- HADITH INGESTION END")

    with open(output_path, 'w') as f:
        f.write("\n".join(surql_statements))

    print(f"Generated {output_path} with {len(mock_hadith)} narrations.")

if __name__ == "__main__":
    os.makedirs("output", exist_ok=True)
    extract_hadith("output/hadith_nodes.surql")
