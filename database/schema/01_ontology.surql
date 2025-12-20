-- 01_ontology.surql
-- Defines the Node Types (Vertices) and strict typing rules.

-- 1. QURAN VERSE (Thabit / Immutable)
-- Source of Truth. Strictly the Arabic text.
DEFINE TABLE quran_verse SCHEMAFULL;
DEFINE FIELD text_uthmani ON TABLE quran_verse TYPE string;
DEFINE FIELD surah_number ON TABLE quran_verse TYPE int;
DEFINE FIELD ayah_number ON TABLE quran_verse TYPE int;
DEFINE FIELD juz_number ON TABLE quran_verse TYPE int;
DEFINE FIELD revelation_place ON TABLE quran_verse TYPE string ASSERT $value INSIDE ['Makkah', 'Madinah'];
DEFINE FIELD mutability ON TABLE quran_verse TYPE string ASSERT $value = 'CONSTANT';

-- 1a. ROOT WORD (Morphology)
-- Linguistic roots connecting words across the corpus.
DEFINE TABLE root_word SCHEMAFULL;
DEFINE FIELD root_ar ON TABLE root_word TYPE string; -- e.g. "كتب"
DEFINE FIELD definition_en ON TABLE root_word TYPE string;

-- 1b. CONCEPT (Thematic Ontology)
-- Abstract themes (Tawhid, Adab, etc).
DEFINE TABLE concept SCHEMAFULL;
DEFINE FIELD name_en ON TABLE concept TYPE string;
DEFINE FIELD name_ar ON TABLE concept TYPE string;

-- 1c. DIVINE NAME (Asma ul Husna)
-- The 99 Names of Allah.
DEFINE TABLE divine_name SCHEMAFULL;
DEFINE FIELD transliteration ON TABLE divine_name TYPE string; -- e.g. "Ar-Rahman"
DEFINE FIELD arabic ON TABLE divine_name TYPE string;
DEFINE FIELD meaning_en ON TABLE divine_name TYPE string;

-- 2. HADITH (Thabit / Immutable)
-- Source of Truth.
DEFINE TABLE hadith SCHEMAFULL;
DEFINE FIELD collection ON TABLE hadith TYPE string; -- e.g. "Sahih Bukhari"
DEFINE FIELD hadith_number ON TABLE hadith TYPE int;
DEFINE FIELD matn_ar ON TABLE hadith TYPE string;
-- Note: 'grading' removed from node. It is an opinion, not a fact.
DEFINE FIELD mutability ON TABLE hadith TYPE string ASSERT $value = 'CONSTANT';

-- 3. TRANSLATION (Tafsir/Zanni)
-- Human effort to understand the Divine.
DEFINE TABLE translation SCHEMAFULL;
DEFINE FIELD text_en ON TABLE translation TYPE string;
DEFINE FIELD translator ON TABLE translation TYPE string; -- e.g. "Sahih International"
DEFINE FIELD language ON TABLE translation TYPE string DEFAULT 'en';

-- 4. SCHOLAR (Context)
-- Biographical data.
DEFINE TABLE scholar SCHEMAFULL;
DEFINE FIELD name_ar ON TABLE scholar TYPE string;
DEFINE FIELD name_en ON TABLE scholar TYPE string;
DEFINE FIELD death_year_ah ON TABLE scholar TYPE int;

-- 5. FIQH RULING (Zanni / Interpretive)
-- Scholar's derivation. Mutable but must be signed.
DEFINE TABLE fiqh_ruling SCHEMAFULL;
DEFINE FIELD text ON TABLE fiqh_ruling TYPE string;
DEFINE FIELD hukm ON TABLE fiqh_ruling TYPE string; -- e.g. "Wajib", "Haram"
DEFINE FIELD madhab ON TABLE fiqh_ruling TYPE string;
-- STRICT ATTRIBUTION: A ruling without an author is invalid.
DEFINE FIELD issued_by ON TABLE fiqh_ruling TYPE record<scholar> ASSERT $value != NONE;
DEFINE FIELD mutability ON TABLE fiqh_ruling TYPE string ASSERT $value = 'VARIABLE';

-- 6. EDGES (Relationships)
-- Explicitly define the edges for graph traversal.

-- Evidence: Ruling -> derived_from -> Source
DEFINE TABLE derived_from SCHEMAFULL TYPE RELATION FROM fiqh_ruling TO quran_verse|hadith;

-- Translation: Source -> translated_as -> Translation
DEFINE TABLE translated_as SCHEMAFULL TYPE RELATION FROM quran_verse|hadith TO translation;

-- Grading: Scholar -> graded -> Hadith (with metadata)
DEFINE TABLE graded SCHEMAFULL TYPE RELATION FROM scholar TO hadith;
DEFINE FIELD rank ON TABLE graded TYPE string; -- e.g. "Sahih", "Daif"

-- Root: Word -> has_root -> RootWord
DEFINE TABLE has_root SCHEMAFULL TYPE RELATION FROM quran_verse TO root_word;

-- Concept: Verse -> discusses -> Concept
DEFINE TABLE discusses SCHEMAFULL TYPE RELATION FROM quran_verse TO concept;

-- Attribute: Verse -> invokes -> DivineName
DEFINE TABLE invokes SCHEMAFULL TYPE RELATION FROM quran_verse TO divine_name;
