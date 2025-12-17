# Al-Mizan Domain-Specific Ontology (DSO) v1.0

## 1. Overview

This document defines the schema constraints and relationship logic for the Al-Mizan Knowledge Graph. It formalizes the "Stratification of Truth" by mapping Islamic Epistemological concepts (Thawabit/Mutaghayyirat) into database constraints using SurrealDB.

## 2. Core Vertices (Nodes)

### 2.1 Primary Sources (The Immutable Layer)

Nodes in this layer are Read-Only and cannot be modified by the inference engine.

#### `quran_verse`

- **id**: `verse:2_255` (Surah_Ayah)
- **text_uthmani**: String (Arabic text)
- **text_en**: String (Translation)
- **mutability**: CONSTANT (Thabit)

#### `hadith`

- **id**: `hadith:bukhari_1`
- **collection**: String (e.g., "Sahih Bukhari")
- **matn_ar**: String (Content Arabic)
- **grading**: Enum (Sahih, Hasan, Daif, Mawdu)
- **mutability**: CONSTANT (Thabit)

### 2.2 Secondary Derivations (The Interpretive Layer)

Nodes in this layer represent scholarly understanding and are subject to Ijtihad (Reasoning).

#### `fiqh_ruling`

- **id**: `ruling:shafii_wudu_niyyah`
- **text**: "Intention is Fard for Wudu"
- **hukm**: Enum (Wajib, Mandub, Mubah, Makruh, Haram)
- **madhab**: String (e.g., "Shafi'i")
- **mutability**: VARIABLE (Zanni)

#### `scholar`

- **id**: `scholar:ibn_kathir`
- **name_ar**: String
- **death_year_ah**: Integer

## 3. Core Edges (Relationships)

### 3.1 Textual Linkage

#### `EXPLAINS` (`hadith` $\rightarrow$ `quran_verse`)

- **Definition**: A Hadith provides Tafsir or context for a verse.

#### `NARRATED_BY` (`hadith` $\rightarrow$ `scholar`)

- **Definition**: Links a text to its transmitter (Isnad chain).

### 3.2 Logical Derivation (The Reasoning Engine)

#### `DERIVED_FROM` (`fiqh_ruling` $\rightarrow$ [`quran_verse` | `hadith`])

- **Definition**: The Dalil (Evidence) used to support a ruling.
- **Constraint**: A ruling cannot exist without at least one outgoing DERIVED_FROM edge (No orphan rulings).

#### `CONDITION_FOR` (`fiqh_ruling` $\rightarrow$ `fiqh_ruling`)

- **Definition**: Logical dependency (e.g., Wudu is a condition for Salah).
- **Usage**: Used by EdTech modules to generate prerequisite maps.

#### `ABROGATES` ([`verse`|`hadith`] $\rightarrow$ [`verse`|`hadith`])

- **Definition**: The active text supersedes the ruling of the target text (Nasakh).

## 4. SurrealDB Schema Implementation

```surql
-- Define Tables (Nodes)
DEFINE TABLE quran_verse SCHEMAFULL;
DEFINE FIELD text_uthmani ON TABLE quran_verse TYPE string;
DEFINE FIELD mutability ON TABLE quran_verse TYPE string ASSERT $value = 'CONSTANT';

DEFINE TABLE fiqh_ruling SCHEMAFULL;
DEFINE FIELD hukm ON TABLE fiqh_ruling TYPE string;
DEFINE FIELD mutability ON TABLE fiqh_ruling TYPE string ASSERT $value = 'VARIABLE';

-- Define Relations (Edges)
DEFINE TABLE DERIVED_FROM SCHEMAFULL;
DEFINE FIELD strength ON TABLE DERIVED_FROM TYPE float; -- 1.0 (Qati) to 0.1 (Weak Qiyas)

-- Logic Constraint: High-grade derivation
-- Ensure we only derive rulings from valid sources
DEFINE EVENT sanitize_sources ON TABLE DERIVED_FROM WHEN $event = "CREATE" THEN {
    IF (out.grading = "Mawdu") {
        THROW "Cannot derive ruling from Fabricated Hadith";
    };
};
```
