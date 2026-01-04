"""
Al-Mizan ETL: SemanticHadith V2 - Remaining Entities
Extracts: Chapters, Hadith Types, Mentions
"""
import re
from pathlib import Path

INPUT_FILE = "/home/a/code/al-mizan-project/almizan-etl/data/semantichadith/SemanticHadith-V2/SemanticHadithKGV2.ttl"
OUTPUT_DIR = Path("/home/a/code/al-mizan-project/almizan-etl/output")

def escape_sql(text: str) -> str:
    if not text:
        return ""
    return text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ").strip()

def parse_remaining():
    print(f"Reading {INPUT_FILE}...")
    
    chapters = {}  # chapter_id -> {name, collection, book_no}
    hadith_types = []  # (hadith_id, type)
    mentions = []  # (hadith_id, entity)
    hadith_chapters = []  # (hadith_id, chapter_id)
    
    current_id = None
    current_block = []
    line_count = 0
    
    with open(INPUT_FILE, 'r', encoding='utf-8') as f:
        for line in f:
            line_count += 1
            if line_count % 500000 == 0:
                print(f"  Processed {line_count:,} lines | Chapters: {len(chapters)} | Types: {len(hadith_types)} | Mentions: {len(mentions)}")
            
            stripped = line.strip()
            
            # Detect chapter blocks: :SB-CH0001 rdf:type
            chapter_match = re.match(r'^:([A-Z]{2}-CH\d+)\s+rdf:type', stripped)
            # Detect hadith blocks for types
            hadith_match = re.match(r'^:([A-Z]{2}-HD\d+)\s+rdf:type', stripped)
            
            if chapter_match:
                # Process previous block
                if current_id and current_block:
                    process_block(current_id, " ".join(current_block), chapters, hadith_types, mentions, hadith_chapters)
                current_id = chapter_match.group(1)
                current_block = [stripped]
                continue
            elif hadith_match:
                if current_id and current_block:
                    process_block(current_id, " ".join(current_block), chapters, hadith_types, mentions, hadith_chapters)
                current_id = hadith_match.group(1)
                current_block = [stripped]
                continue
            
            if current_id:
                current_block.append(stripped)
                if stripped == '.':
                    process_block(current_id, " ".join(current_block), chapters, hadith_types, mentions, hadith_chapters)
                    current_id = None
                    current_block = []
    
    print(f"\n=== Parsing Complete ===")
    print(f"  Chapters: {len(chapters)}")
    print(f"  Hadith Types: {len(hadith_types)}")
    print(f"  Mentions: {len(mentions)}")
    print(f"  Hadith-Chapter Links: {len(hadith_chapters)}")
    
    return chapters, hadith_types, mentions, hadith_chapters

def process_block(entity_id, block, chapters, hadith_types, mentions, hadith_chapters):
    # Chapter entity
    if entity_id.startswith(('SB-CH', 'SM-CH', 'SN-CH', 'IM-CH', 'AD-CH', 'JT-CH')) and ':HadithChapter' in block:
        chapter = parse_chapter(entity_id, block)
        if chapter:
            chapters[entity_id] = chapter
    
    # Hadith with type/mentions/chapter
    elif '-HD' in entity_id and ':Hadith' in block:
        # Extract hadith type
        type_match = re.search(r':hasHadithType\s+:(\w+)', block)
        if type_match:
            hadith_types.append((entity_id, type_match.group(1)))
        
        # Extract mentions
        for mention_match in re.finditer(r':containsMentionOf\s+:(\S+)', block):
            entity = mention_match.group(1).rstrip(' ;.,')
            if entity and not entity.startswith(('http', 'rdf', 'owl')):
                mentions.append((entity_id, entity))
        
        # Extract chapter link
        chapter_match = re.search(r':isPartOfChapter\s+:([A-Z]{2}-CH\d+)', block)
        if chapter_match:
            hadith_chapters.append((entity_id, chapter_match.group(1)))

def parse_chapter(chapter_id, block):
    chapter = {"chapter_id": chapter_id}
    
    # Extract collection from ID
    prefix = chapter_id[:2]
    collection_map = {'SB': 'bukhari', 'SM': 'muslim', 'SN': "nasa'i", 'IM': 'ibn_majah', 'AD': 'abu_dawud', 'JT': 'tirmidhi'}
    chapter['collection'] = collection_map.get(prefix, 'unknown')
    
    # Extract chapter number
    num_match = re.search(r'CH(\d+)', chapter_id)
    if num_match:
        chapter['chapter_no'] = int(num_match.group(1))
    
    # Extract Arabic name
    name_match = re.search(r':chapterName\s+"([^"]+)@ar"', block)
    if name_match:
        chapter['name_ar'] = name_match.group(1)
    
    # Extract English name
    name_en_match = re.search(r':chapterName\s+"([^"]+)@en"', block)
    if name_en_match:
        chapter['name_en'] = name_en_match.group(1)
    
    return chapter if 'name_ar' in chapter or 'name_en' in chapter else None

def write_output(chapters, hadith_types, mentions, hadith_chapters):
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    
    # Chapters
    print(f"\nWriting {len(chapters)} chapters...")
    with open(OUTPUT_DIR / "chapters.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Chapters\n\n")
        for ch_id, ch in chapters.items():
            safe_id = ch_id.replace("-", "_")
            name_ar = escape_sql(ch.get('name_ar', ''))
            name_en = escape_sql(ch.get('name_en', ''))
            
            f.write(f'CREATE chapter:{safe_id} SET ')
            f.write(f'chapter_id = "{ch_id}", ')
            f.write(f'collection = "{ch["collection"]}", ')
            f.write(f'chapter_no = {ch.get("chapter_no", 0)}, ')
            if name_ar:
                f.write(f'name_ar = "{name_ar}", ')
            if name_en:
                f.write(f'name_en = "{name_en}", ')
            f.write(f'display_name = "{name_ar or name_en}";\n')
    
    # Hadith Types (as edge: hadith -> has_type -> type)
    print(f"Writing {len(hadith_types)} hadith type links...")
    type_entities = set()
    with open(OUTPUT_DIR / "hadith_types.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Hadith Types\n\n")
        # Create type entities first
        for _, htype in hadith_types:
            type_entities.add(htype)
        for t in sorted(type_entities):
            f.write(f'CREATE hadith_type:{t} SET name = "{t}";\n')
        f.write("\n")
        # Create edges
        for hadith_id, htype in hadith_types:
            safe_hadith = hadith_id.replace("-", "_")
            f.write(f'RELATE semantic_hadith:{safe_hadith}->has_type->hadith_type:{htype};\n')
    
    # Hadith-Chapter links
    print(f"Writing {len(hadith_chapters)} hadith-chapter links...")
    with open(OUTPUT_DIR / "hadith_chapters.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Hadith-Chapter Links\n\n")
        for hadith_id, chapter_id in hadith_chapters:
            safe_hadith = hadith_id.replace("-", "_")
            safe_chapter = chapter_id.replace("-", "_")
            f.write(f'RELATE semantic_hadith:{safe_hadith}->in_chapter->chapter:{safe_chapter};\n')
    
    # Mentions (person/location references)
    print(f"Writing {len(mentions)} mention links...")
    mentioned_entities = set()
    with open(OUTPUT_DIR / "mentions.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Mentions\n\n")
        # Create entity nodes first
        for _, entity in mentions:
            mentioned_entities.add(entity)
        for e in sorted(mentioned_entities):
            safe_e = re.sub(r'[^a-zA-Z0-9_]', '_', e)
            f.write(f'CREATE mentioned_entity:{safe_e} SET name = "{escape_sql(e)}";\n')
        f.write("\n")
        # Create edges
        for hadith_id, entity in mentions:
            safe_hadith = hadith_id.replace("-", "_")
            safe_entity = re.sub(r'[^a-zA-Z0-9_]', '_', entity)
            f.write(f'RELATE semantic_hadith:{safe_hadith}->mentions->mentioned_entity:{safe_entity};\n')
    
    print(f"\n=== Output Complete ===")
    print(f"Files: chapters.surql, hadith_types.surql, hadith_chapters.surql, mentions.surql")

if __name__ == "__main__":
    chapters, hadith_types, mentions, hadith_chapters = parse_remaining()
    write_output(chapters, hadith_types, mentions, hadith_chapters)
