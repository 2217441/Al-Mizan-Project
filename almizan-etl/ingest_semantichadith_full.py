"""
Al-Mizan ETL: Full SemanticHadith V2 Integration
Extracts ALL entity types: Hadiths, Narrators, Chains, Topics, Chapters, Similarities
Expected runtime: 10-15 minutes for 3M lines
"""
import re
import json
import sys
from collections import defaultdict
from pathlib import Path

INPUT_FILE = "/home/a/code/al-mizan-project/almizan-etl/data/semantichadith/SemanticHadith-V2/SemanticHadithKGV2.ttl"
OUTPUT_DIR = Path("/home/a/code/al-mizan-project/almizan-etl/output")

# Collection prefixes
COLLECTION_MAP = {
    "SB-HD": "bukhari",
    "SM-HD": "muslim",
    "SN-HD": "nasa'i",
    "IM-HD": "ibn_majah",
    "AD-HD": "abu_dawud",
    "JT-HD": "tirmidhi",
}

def extract_collection(hadith_id: str) -> str:
    for prefix, collection in COLLECTION_MAP.items():
        if hadith_id.startswith(prefix):
            return collection
    return "unknown"

def extract_ref_no(hadith_id: str) -> int:
    match = re.search(r'HD(\d+)', hadith_id)
    return int(match.group(1)) if match else 0

def extract_lang_text(content: str) -> dict:
    result = {}
    pattern = r'"([^"]+)@(\w+)"\^\^xsd:string'
    for match in re.finditer(pattern, content):
        text, lang = match.groups()
        if text != "0":
            result[lang] = text.strip()
    return result

def parse_ttl_file():
    """Parse the full SemanticHadith TTL file"""
    print(f"Reading {INPUT_FILE}...")
    print("Extracting: Hadiths, Narrators, Chains, Topics, Chapters")
    
    # Storage
    hadiths = {}
    narrators = {}
    chain_segments = {}
    topics = set()
    chapters = set()
    hadith_topics = []  # (hadith_id, topic)
    hadith_similarities = []  # (hadith1, hadith2)
    
    current_id = None
    current_type = None
    current_block = []
    line_count = 0
    
    with open(INPUT_FILE, 'r', encoding='utf-8') as f:
        for line in f:
            line_count += 1
            if line_count % 500000 == 0:
                print(f"  Processed {line_count:,} lines | Hadiths: {len(hadiths)} | Narrators: {len(narrators)} | Chains: {len(chain_segments)}")
            
            stripped = line.strip()
            
            # Detect entity blocks
            # Hadith: :SB-HD0136 rdf:type owl:NamedIndividual
            hadith_match = re.match(r'^:([A-Z]{2}-HD\d+)\s+rdf:type', stripped)
            # Narrator: :HN00001 rdf:type owl:NamedIndividual
            narrator_match = re.match(r'^:(HN\d+)\s+rdf:type', stripped)
            # Chain Segment: :SB-HD0136-ChainSeg-1 rdf:type
            chain_match = re.match(r'^:([A-Z]{2}-HD\d+-ChainSeg-\d+)\s+rdf:type', stripped)
            # Topic: :Salah rdf:type owl:NamedIndividual, :Topic
            topic_match = re.match(r'^:([A-Za-z]+)\s+rdf:type.*:Topic', stripped)
            # Chapter: :SN-CH2537 rdf:type
            chapter_match = re.match(r'^:([A-Z]{2}-CH\d+)\s+rdf:type', stripped)
            
            if hadith_match or narrator_match or chain_match or topic_match or chapter_match:
                # Process previous block
                if current_id and current_block:
                    full_block = " ".join(current_block)
                    process_block(current_id, current_type, full_block, 
                                hadiths, narrators, chain_segments, topics, chapters,
                                hadith_topics, hadith_similarities)
                
                # Start new block
                if hadith_match:
                    current_id = hadith_match.group(1)
                    current_type = "hadith"
                elif narrator_match:
                    current_id = narrator_match.group(1)
                    current_type = "narrator"
                elif chain_match:
                    current_id = chain_match.group(1)
                    current_type = "chain"
                elif topic_match:
                    current_id = topic_match.group(1)
                    current_type = "topic"
                elif chapter_match:
                    current_id = chapter_match.group(1)
                    current_type = "chapter"
                
                current_block = [stripped]
                continue
            
            # Accumulate block lines
            if current_id:
                current_block.append(stripped)
                if stripped == '.':
                    full_block = " ".join(current_block)
                    process_block(current_id, current_type, full_block,
                                hadiths, narrators, chain_segments, topics, chapters,
                                hadith_topics, hadith_similarities)
                    current_id = None
                    current_type = None
                    current_block = []
    
    print(f"\n=== Parsing Complete ===")
    print(f"  Hadiths: {len(hadiths)}")
    print(f"  Narrators: {len(narrators)}")
    print(f"  Chain Segments: {len(chain_segments)}")
    print(f"  Topics: {len(topics)}")
    print(f"  Hadith-Topic links: {len(hadith_topics)}")
    print(f"  Similar Hadiths: {len(hadith_similarities)}")
    
    return hadiths, narrators, chain_segments, topics, hadith_topics, hadith_similarities

def process_block(entity_id, entity_type, block, hadiths, narrators, chain_segments, 
                  topics, chapters, hadith_topics, hadith_similarities):
    """Process a single entity block"""
    
    if entity_type == "hadith" and ":Hadith" in block:
        hadith = parse_hadith(entity_id, block)
        if hadith:
            hadiths[entity_id] = hadith
            
            # Extract topic relationships
            topic_match = re.search(r':discussesTopic\s+:(\w+)', block)
            if topic_match:
                hadith_topics.append((entity_id, topic_match.group(1)))
            
            # Extract similar hadiths
            for sim_match in re.finditer(r':isSimilar\s+:([A-Z]{2}-HD\d+)', block):
                hadith_similarities.append((entity_id, sim_match.group(1)))
    
    elif entity_type == "narrator" and ":HadithNarrator" in block:
        narrator = parse_narrator(entity_id, block)
        if narrator:
            narrators[entity_id] = narrator
    
    elif entity_type == "chain" and ":NarratorChainSegment" in block:
        segment = parse_chain_segment(entity_id, block)
        if segment:
            chain_segments[entity_id] = segment
    
    elif entity_type == "topic" and ":Topic" in block:
        topics.add(entity_id)

def parse_hadith(hadith_id, block):
    """Parse hadith block"""
    if "owl:Class" in block:
        return None
    
    hadith = {
        "ref_id": hadith_id,
        "collection": extract_collection(hadith_id),
        "ref_no": extract_ref_no(hadith_id),
    }
    
    text_match = re.search(r':fullHadithText\s+(.*?)(?:;|$)', block, re.DOTALL)
    if text_match:
        lang_texts = extract_lang_text(text_match.group(1))
        if 'ar' in lang_texts:
            hadith['body_ar'] = lang_texts['ar']
        if 'ur' in lang_texts:
            hadith['body_ur'] = lang_texts['ur']
    
    chain_match = re.search(r':hasNarratorChain\s+:(\S+)', block)
    if chain_match:
        hadith['chain_id'] = chain_match.group(1).rstrip(' ;.')
    
    chapter_match = re.search(r':isPartOfChapter\s+:(\S+)', block)
    if chapter_match:
        hadith['chapter_id'] = chapter_match.group(1).rstrip(' ;.')
    
    topic_match = re.search(r':discussesTopic\s+:(\S+)', block)
    if topic_match:
        hadith['topic'] = topic_match.group(1).rstrip(' ;.')
    
    if 'body_ar' not in hadith:
        return None
    
    return hadith

def parse_narrator(narrator_id, block):
    """Parse narrator block"""
    narrator = {"narrator_id": narrator_id}
    
    # Extract Arabic name
    name_match = re.search(r':name\s+"([^"]+)@ar"', block)
    if name_match:
        narrator['name_ar'] = name_match.group(1)
    
    # Extract popular name
    pop_match = re.search(r':popularName\s+"([^"]+)"', block)
    if pop_match:
        narrator['popular_name'] = pop_match.group(1)
    
    # Extract generation
    gen_match = re.search(r':generation\s+"(\d+)"', block)
    if gen_match:
        narrator['generation'] = int(gen_match.group(1))
    
    # Extract death year
    death_match = re.search(r':deathYear\s+"(\d+)"', block)
    if death_match:
        narrator['death_year'] = int(death_match.group(1))
    
    return narrator if 'name_ar' in narrator or 'popular_name' in narrator else None

def parse_chain_segment(segment_id, block):
    """Parse chain segment block"""
    segment = {"segment_id": segment_id}
    
    # Extract hadith ID from segment ID (e.g., SB-HD0136-ChainSeg-1 -> SB-HD0136)
    hadith_match = re.match(r'([A-Z]{2}-HD\d+)-ChainSeg', segment_id)
    if hadith_match:
        segment['hadith_id'] = hadith_match.group(1)
    
    # Extract narrator reference
    narrator_match = re.search(r':refersToNarrator\s+:(HN\d+)', block)
    if narrator_match:
        segment['narrator_id'] = narrator_match.group(1)
    
    # Extract follows relationship
    follows_match = re.search(r':follows\s+:(\S+)', block)
    if follows_match:
        segment['follows'] = follows_match.group(1).rstrip(' ;.')
    
    # Check if root narrator
    segment['is_root'] = ':RootNarratorChainSegment' in block
    
    return segment if 'narrator_id' in segment else None

def write_output(hadiths, narrators, chain_segments, topics, hadith_topics, hadith_similarities):
    """Write all entities to SurrealQL files"""
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    
    # Hadiths
    print(f"\nWriting {len(hadiths)} hadiths...")
    with open(OUTPUT_DIR / "semantic_hadith_full.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Hadiths\n\n")
        for ref_id, h in hadiths.items():
            safe_id = ref_id.replace("-", "_")
            body_ar = h.get('body_ar', '')
            body_ur = h.get('body_ur', '')
            display = body_ar[:100] + "..." if len(body_ar) > 100 else body_ar
            
            f.write(f'CREATE semantic_hadith:{safe_id} SET ')
            f.write(f'ref_id = "{ref_id}", ')
            f.write(f'collection = "{h["collection"]}", ')
            f.write(f'ref_no = {h["ref_no"]}, ')
            f.write(f'body_ar = {json.dumps(body_ar)}, ')
            if body_ur:
                f.write(f'body_ur = {json.dumps(body_ur)}, ')
            if h.get('chain_id'):
                f.write(f'chain_id = "{h["chain_id"]}", ')
            if h.get('topic'):
                f.write(f'topic = "{h["topic"]}", ')
            f.write(f'display_text = {json.dumps(display)};\n')
    
    # Narrators
    print(f"Writing {len(narrators)} narrators...")
    with open(OUTPUT_DIR / "narrators.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Narrators\n\n")
        for nid, n in narrators.items():
            name_ar = n.get('name_ar', n.get('popular_name', ''))
            popular = n.get('popular_name', '')
            gen = n.get('generation', 0)
            death = n.get('death_year', 0)
            
            f.write(f'CREATE narrator:{nid} SET ')
            f.write(f'narrator_id = "{nid}", ')
            f.write(f'name_ar = {json.dumps(name_ar)}, ')
            if popular:
                f.write(f'popular_name = {json.dumps(popular)}, ')
            f.write(f'generation = {gen}, ')
            f.write(f'death_year = {death};\n')
    
    # Chain edges: Narrator -> Hadith (narrated)
    print(f"Writing {len(chain_segments)} chain edges...")
    with open(OUTPUT_DIR / "chain_edges.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Narrator Chain Edges\n\n")
        seen_edges = set()
        for seg_id, seg in chain_segments.items():
            hadith_id = seg.get('hadith_id')
            narrator_id = seg.get('narrator_id')
            if hadith_id and narrator_id:
                edge_key = f"{narrator_id}_{hadith_id}"
                if edge_key not in seen_edges:
                    seen_edges.add(edge_key)
                    safe_hadith = hadith_id.replace("-", "_")
                    f.write(f'RELATE narrator:{narrator_id}->narrated->semantic_hadith:{safe_hadith};\n')
    
    # Topics
    print(f"Writing {len(topics)} topics...")
    with open(OUTPUT_DIR / "topics.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Topics\n\n")
        for topic in topics:
            safe_topic = topic.replace("-", "_")
            f.write(f'CREATE topic:{safe_topic} SET name = "{topic}";\n')
    
    # Hadith-Topic relationships
    print(f"Writing {len(hadith_topics)} hadith-topic links...")
    with open(OUTPUT_DIR / "hadith_topics.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Hadith-Topic Links\n\n")
        for hadith_id, topic in hadith_topics:
            safe_hadith = hadith_id.replace("-", "_")
            safe_topic = topic.replace("-", "_")
            f.write(f'RELATE semantic_hadith:{safe_hadith}->discusses->topic:{safe_topic};\n')
    
    # Similar hadiths
    print(f"Writing {len(hadith_similarities)} similarity links...")
    with open(OUTPUT_DIR / "similarities.surql", 'w', encoding='utf-8') as f:
        f.write("-- SemanticHadith V2 - Similar Hadiths\n\n")
        seen_sims = set()
        for h1, h2 in hadith_similarities:
            key = tuple(sorted([h1, h2]))
            if key not in seen_sims:
                seen_sims.add(key)
                safe_h1 = h1.replace("-", "_")
                safe_h2 = h2.replace("-", "_")
                f.write(f'RELATE semantic_hadith:{safe_h1}->similar_to->semantic_hadith:{safe_h2};\n')
    
    print(f"\n=== Output Complete ===")
    print(f"Files written to: {OUTPUT_DIR}")

def main():
    hadiths, narrators, chain_segments, topics, hadith_topics, hadith_similarities = parse_ttl_file()
    write_output(hadiths, narrators, chain_segments, topics, hadith_topics, hadith_similarities)
    
    print("\n=== Summary by Collection ===")
    by_collection = defaultdict(int)
    for h in hadiths.values():
        by_collection[h['collection']] += 1
    for collection, count in sorted(by_collection.items(), key=lambda x: -x[1]):
        print(f"  {collection}: {count:,}")

if __name__ == "__main__":
    main()
