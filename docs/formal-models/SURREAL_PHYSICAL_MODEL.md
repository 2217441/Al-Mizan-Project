# Al-Mizan: SurrealDB Physical Schema Model

This document specifies the **physical byte-level schema** of the Al-Mizan vector database. It uses strict **SurrealQL** typing conventions (`record<T>`, `array<float>`, `geometry`) to ensure implementation fidelity.

---

## 1. The Physical Graph (SurrealQL)

**Top Event**: The strict `SCHEMAFULL` definition of the Knowledge Graph.

```plantuml
@startuml
!theme plain
hide circle
skinparam linetype ortho

' REPOSITORY: https://github.com/FirdausHisyam/Islamic-Digital-Citadel
' ENGINE: SurrealDB (v2.0)
' MODE: SCHEMAFULL (Strict Typing)

package "Tier 1: Thabit (Immutable Source)" {
    class "allah" as allah << TABLE >> {
        <color:blue>id</color>: record<allah>
        names: array<string>
        attributes: object
        --
        <b>DEFINE TABLE</b> allah <b>SCHEMAFULL</b>
        <b>PERMISSIONS</b> READ FULL
    }

    class "quran_verse" as quran_verse << TABLE >> {
        <color:blue>id</color>: record<quran_verse>
        surah: int
        ayah: int
        text_uthmani: string
        text_en: string
        <color:green>embedding_v1</color>: array<float, 1536>
        --
        <b>DEFINE TABLE</b> quran_verse <b>SCHEMAFULL</b>
        <b>DEFINE INDEX</b> idx_embed <b>ON</b> quran_verse <b>COLUMNS</b> embedding_v1 <b>MTREE DIMENSION</b> 1536 <b>DIST COSINE</b>
    }
    
    class "hadith" as hadith << TABLE >> {
        <color:blue>id</color>: record<hadith>
        collection: string
        number: int
        grade: string
        isnad: array<string>
        text_matn: string
        <color:green>embedding_v1</color>: array<float, 1536>
        --
        <b>DEFINE TABLE</b> hadith <b>SCHEMAFULL</b>
        <b>DEFINE INDEX</b> mt_search <b>ON</b> hadith <b>COLUMNS</b> text_matn <b>SEARCH ANALYZER</b> arabic_snowball
    }
}

package "Tier 2: Zanni (Interpretive Layer)" {
    class "scholar" as scholar << TABLE >> {
        <color:blue>id</color>: record<scholar>
        public_key_did: string
        reputation_score: float
        is_jailed: bool
        last_active: datetime
        --
        <b>DEFINE TABLE</b> scholar <b>SCHEMAFULL</b>
        <b>DEFINE INDEX</b> uniq_did <b>ON</b> scholar <b>COLUMNS</b> public_key_did <b>UNIQUE</b>
    }

    class "fiqh_ruling" as fiqh_ruling << TABLE >> {
        <color:blue>id</color>: record<fiqh_ruling>
        hukm: string
        confidence: decimal
        generated_at: datetime
        hash_merkle_root: string
        --
        <b>DEFINE TABLE</b> fiqh_ruling <b>SCHEMAFULL</b>
        <b>DEFINE EVENT</b> immutable_log <b>ON</b> fiqh_ruling <b>THEN</b> ( create audit_log set ... )
    }
    
    class "ijma_consensus" as ijma_consensus << TABLE >> {
        <color:blue>id</color>: record<ijma_consensus>
        topic_hash: string
        vote_tally: int
        status: string
        quorum_met: bool
        --
        <b>DEFINE TABLE</b> ijma_consensus <b>SCHEMAFULL</b>
    }
}

package "Relations (Graph Edges)" {
    class "REVEALS" as REVEALS << REL >> {
        <color:blue>in</color>: record<allah>
        <color:blue>out</color>: record<quran_verse> | record<hadith>
        revelation_order: int
        --
        <b>DEFINE TABLE</b> REVEALS <b>SCHEMAFULL</b>
    }

    class "DERIVES" as DERIVES << REL >> {
        <color:blue>in</color>: record<scholar>
        <color:blue>out</color>: record<fiqh_ruling>
        methodology: string
        signature: string
        --
        <b>DEFINE TABLE</b> DERIVES <b>SCHEMAFULL</b>
    }

    class "EVIDENCE" as EVIDENCE << REL >> {
        <color:blue>in</color>: record<fiqh_ruling>
        <color:blue>out</color>: record<wahy>
        relevance_score: float
        --
        <b>DEFINE TABLE</b> EVIDENCE <b>SCHEMAFULL</b>
    }
    
    class "VOTES_ON" as VOTES_ON << REL >> {
        <color:blue>in</color>: record<scholar>
        <color:blue>out</color>: record<ijma_consensus>
        vote: string
        stake_amount: decimal
        --
        <b>DEFINE TABLE</b> VOTES_ON <b>SCHEMAFULL</b>
    }
}

allah "1" ..> "*" quran_verse : REVEALS
allah "1" ..> "*" hadith : REVEALS

scholar "1" ..> "*" fiqh_ruling : DERIVES
scholar "*" ..> "*" ijma_consensus : VOTES_ON

fiqh_ruling "*" ..> "*" quran_verse : EVIDENCE
fiqh_ruling "*" ..> "*" hadith : EVIDENCE

ijma_consensus "1" ..> "*" fiqh_ruling : RATIFIES

@enduml
```

## 2. SurrealQL Implementation Specifics

### 2.1 Vector Indexing (M-Tree)
The `quran_verse` and `hadith` tables utilize strict **M-Tree** indexing for semantic search.

```sql
-- STRICT TYPING
DEFINE INDEX idx_embed ON quran_verse 
  COLUMNS embedding_v1 
  MTREE DIMENSION 1536 
  DIST COSINE;
```

### 2.2 Graph Enforcement
Graph integrity is enforced at the database layer using `TYPE record`.

```sql
-- EDGE CONSTRAINT
DEFINE FIELD in ON TABLE REVEALS TYPE record<allah>;
DEFINE FIELD out ON TABLE REVEALS TYPE record<quran_verse> | record<hadith>;
```
