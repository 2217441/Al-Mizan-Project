import xml.etree.ElementTree as ET
import os

def extract_quran(xml_path, output_path):
    print(f"Extracting Quran from {xml_path}...")
    
    if not os.path.exists(xml_path):
        print(f"ERROR: XML Source {xml_path} not found. Please place 'quran-uthmani.xml' in database/ folder.")
        # Fallback to empty to prevent crash, allowing user to see the error message.
        return

    tree = ET.parse(xml_path)
    root = tree.getroot()

    surql_statements = []
    surql_statements.append("-- QURAN INGESTION START")
    surql_statements.append("BEGIN TRANSACTION;")

    # quran -> suras -> sura -> ayas -> aya
    for surah in root.findall('sura'):
        surah_num = int(surah.get('index'))
        # name_ar = surah.get('name') 
        
        for ayah in surah.findall('aya'):
            ayah_num = int(ayah.get('index'))
            text_uthmani = ayah.get('text')
            
            # Simple Juz Logic (Approximation for brevity, ideally mapped accurately)
            # Juz 1 starts at 1:1, Juz 2 at 2:142, etc. 
            # For this script we will default to 0 if not calculated, or use a lookup.
            # Using placeholder 0 for now as XML doesn't strictly have Juz info usually.
            juz_num = 0 
            
            revelation_place = "Makkah" # Default, needs lookup map

            verse_id = f"verse:{surah_num}_{ayah_num}"
            
            # Escape single quotes for SQL
            clean_text = text_uthmani.replace("'", "\\'")

            statement = f"CREATE {verse_id} SET \n" \
                        f"    text_uthmani = '{clean_text}', \n" \
                        f"    surah_number = {surah_num}, \n" \
                        f"    ayah_number = {ayah_num}, \n" \
                        f"    juz_number = {juz_num}, \n" \
                        f"    revelation_place = '{revelation_place}', \n" \
                        f"    mutability = 'CONSTANT';"
            
            surql_statements.append(statement)

    surql_statements.append("COMMIT TRANSACTION;")
    surql_statements.append("-- QURAN INGESTION END")

    with open(output_path, 'w', encoding='utf-8') as f:
        f.write("\n".join(surql_statements))
    
    print(f"Generated {output_path} successfully.")

if __name__ == "__main__":
    os.makedirs("output", exist_ok=True)
    # Ideally checking for the file in the parent 'database' folder
    INPUT_XML = "../database/quran-uthmani.xml"
    OUTPUT_SURQL = "output/quran_nodes.surql"
    
    extract_quran(INPUT_XML, OUTPUT_SURQL)
