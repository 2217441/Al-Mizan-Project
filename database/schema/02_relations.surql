-- 02_relations.surql
-- Defines the Edge Types (Relationships) and logic constraints.

-- 1. EXPLAINS (Tafsir Logic)
-- Hadith -> Verse
DEFINE TABLE EXPLAINS SCHEMAFULL;

-- 2. NARRATED_BY (Isnad Logic)
-- Hadith -> Scholar
DEFINE TABLE NARRATED_BY SCHEMAFULL;
DEFINE FIELD narration_order ON TABLE NARRATED_BY TYPE int; -- 1 = First narrator

-- 3. DERIVED_FROM (Istinbat Logic)
-- Ruling -> Source (Verse/Hadith)
DEFINE TABLE DERIVED_FROM SCHEMAFULL;

-- The 'IN' side must be a Fiqh Ruling
DEFINE FIELD in ON TABLE DERIVED_FROM TYPE record<fiqh_ruling>;

-- The 'OUT' side must be a Source Text (Verse or Hadith)
DEFINE FIELD out ON TABLE DERIVED_FROM TYPE record<quran_verse | hadith>;

-- Metadata fields
DEFINE FIELD strength ON TABLE DERIVED_FROM TYPE float; -- Confidence score (0.0 - 1.0)
DEFINE FIELD method ON TABLE DERIVED_FROM TYPE string; -- e.g. "Qiyas", "Ijma"

-- THE "DAJJAL FILTER" (Logic Gate)
-- If someone tries to link a ruling to a WEAK/FABRICATED Hadith, block it.
DEFINE EVENT check_grading ON TABLE DERIVED_FROM WHEN $event = "CREATE" THEN {
    -- Fetch the Hadith being linked to
    LET $source_node = (SELECT * FROM $after.out);
    
    -- If it is a Hadith and it is Mawdu (Fabricated)...
    -- Note: .grading accesses the field on the remote record
    IF ($source_node.grading = 'Mawdu') {
        -- REJECT THE WRITE
        THROW "Error: Cannot base a ruling on Fabricated Evidence.";
    };
};

-- 4. ABROGATES (Naskh Logic)
-- Text -> Text
DEFINE TABLE ABROGATES SCHEMAFULL;
DEFINE FIELD consensus ON TABLE ABROGATES TYPE bool;

-- 5. CONDITION_FOR (Dependency Logic)
-- Ruling -> Ruling
DEFINE TABLE CONDITION_FOR SCHEMAFULL;
