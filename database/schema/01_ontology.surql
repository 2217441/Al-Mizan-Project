-- 01_ontology.surql
-- Defines the Node Types (Vertices) and strict typing rules.

-- 1. QURAN VERSE (Thabit / Immutable)
-- Source of Truth. Read-Only for public.
DEFINE TABLE quran_verse SCHEMAFULL;
DEFINE FIELD text_uthmani ON TABLE quran_verse TYPE string;
DEFINE FIELD text_en ON TABLE quran_verse TYPE string;
DEFINE FIELD surah_number ON TABLE quran_verse TYPE int;
DEFINE FIELD ayah_number ON TABLE quran_verse TYPE int;
DEFINE FIELD mutability ON TABLE quran_verse TYPE string ASSERT $value = 'CONSTANT';

-- 2. HADITH (Thabit / Immutable)
-- Source of Truth. Read-Only for public.
DEFINE TABLE hadith SCHEMAFULL;
DEFINE FIELD collection ON TABLE hadith TYPE string; -- e.g. "Sahih Bukhari"
DEFINE FIELD hadith_number ON TABLE hadith TYPE int;
DEFINE FIELD matn_ar ON TABLE hadith TYPE string;
DEFINE FIELD matn_en ON TABLE hadith TYPE string;
DEFINE FIELD grading ON TABLE hadith TYPE string; -- e.g. "Sahih", "Hasan"
DEFINE FIELD mutability ON TABLE hadith TYPE string ASSERT $value = 'CONSTANT';

-- 3. FIQH RULING (Zanni / Interpretive)
-- Scholar's derivation. Mutable but must be signed.
DEFINE TABLE fiqh_ruling SCHEMAFULL;
DEFINE FIELD text ON TABLE fiqh_ruling TYPE string;
DEFINE FIELD hukm ON TABLE fiqh_ruling TYPE string; -- e.g. "Wajib", "Haram"
DEFINE FIELD madhab ON TABLE fiqh_ruling TYPE string;
DEFINE FIELD mutability ON TABLE fiqh_ruling TYPE string ASSERT $value = 'VARIABLE';

-- 4. SCHOLAR (Context)
-- Biographical data.
DEFINE TABLE scholar SCHEMAFULL;
DEFINE FIELD name_ar ON TABLE scholar TYPE string;
DEFINE FIELD name_en ON TABLE scholar TYPE string;
DEFINE FIELD death_year_ah ON TABLE scholar TYPE int;
