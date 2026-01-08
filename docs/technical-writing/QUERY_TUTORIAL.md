# SurQL Query Tutorial

> **Audience**: Researchers, Developers  
> **Prerequisites**: Basic SQL knowledge  
> **Author**: Technical Writing Agent  
> **Last Updated**: 2026-01-08

---

## Overview

SurQL (SurrealDB Query Language) is the query language for the Al-Mizan Knowledge Graph. It combines SQL-like syntax with graph traversal capabilities, making it ideal for exploring Islamic textual relationships.

This tutorial covers essential query patterns for researching the knowledge graph.

---

## Table of Contents

1. [Getting Started](#1-getting-started)
2. [Basic Queries](#2-basic-queries)
3. [Graph Traversal](#3-graph-traversal)
4. [Filtering & Conditions](#4-filtering--conditions)
5. [Aggregations](#5-aggregations)
6. [Advanced Patterns](#6-advanced-patterns)
7. [Research Examples](#7-research-examples)

---

## 1. Getting Started

### Connecting to the Database

You can run SurQL queries through:

1. **API Endpoint** (coming soon): `POST /api/v1/query`
2. **SurrealDB CLI**: Direct database connection
3. **Playground**: Interactive web interface at `/playground`

### Basic Syntax

```surql
-- Single-line comment
/* Multi-line
   comment */

-- Basic SELECT
SELECT * FROM table_name;

-- With conditions
SELECT * FROM table_name WHERE field = 'value';
```

---

## 2. Basic Queries

### Selecting Records

```surql
-- Get all verses (limit for performance)
SELECT * FROM quran_verse LIMIT 10;

-- Get specific verse by ID
SELECT * FROM quran_verse:1_1;

-- Get specific fields
SELECT surah_number, ayah_number, text_uthmani 
FROM quran_verse 
LIMIT 10;
```

### Counting Records

```surql
-- Count all verses
SELECT count() FROM quran_verse;

-- Count by category
SELECT revelation_place, count() 
FROM quran_verse 
GROUP BY revelation_place;
```

### Ordering Results

```surql
-- Order by surah and ayah
SELECT * FROM quran_verse 
ORDER BY surah_number, ayah_number 
LIMIT 20;

-- Descending order
SELECT * FROM hadith 
ORDER BY hadith_number DESC 
LIMIT 10;
```

---

## 3. Graph Traversal

This is where SurQL shines for the Knowledge Graph!

### Following Edges (Outbound)

Use `->` to follow outbound relationships:

```surql
-- Get roots of a verse
SELECT ->has_root->root_word 
FROM quran_verse:1_1;

-- Get all concepts discussed by a verse
SELECT ->discusses->concept 
FROM quran_verse:2_255;

-- Get divine names invoked in a verse
SELECT ->invokes->divine_name 
FROM quran_verse:1_1;
```

### Reverse Traversal (Inbound)

Use `<-` to follow inbound relationships:

```surql
-- Get all verses that have a specific root
SELECT <-has_root<-quran_verse 
FROM root_word:رحم;

-- Get all rulings that cite a specific verse
SELECT <-derived_from<-fiqh_ruling 
FROM quran_verse:2_275;
```

### Multi-hop Traversal

Chain traversals for deep queries:

```surql
-- From Allah through prophets to verses
SELECT ->chosen_by->prophet->narrated_quran->quran_verse 
FROM allah:tawhid;

-- From ruling to its primary sources and their translations
SELECT ->derived_from->(quran_verse, hadith)->translated_as->translation 
FROM fiqh_ruling:riba_prohibition;
```

### Extracting Edge Properties

```surql
-- Get hadith gradings with the grading details
SELECT 
    id,
    collection,
    hadith_number,
    <-graded.rank AS grades,
    <-graded<-scholar.name_en AS graders
FROM hadith 
WHERE collection = 'bukhari' 
LIMIT 5;
```

---

## 4. Filtering & Conditions

### Basic Conditions

```surql
-- Exact match
SELECT * FROM quran_verse WHERE surah_number = 1;

-- Multiple conditions (AND)
SELECT * FROM hadith 
WHERE collection = 'bukhari' 
AND hadith_type = 'qudsi';

-- OR conditions
SELECT * FROM scholar 
WHERE status = 'active' 
OR reputation > 8.0;
```

### Pattern Matching

```surql
-- Contains (case-sensitive)
SELECT * FROM concept WHERE name_en CONTAINS 'prayer';

-- Starts with
SELECT * FROM prophet WHERE name_en ~ /^M/;
```

### INSIDE Operator

```surql
-- Check if value is in a list
SELECT * FROM fiqh_ruling 
WHERE hukm INSIDE ['Haram', 'Makruh'];

-- Check revelation place
SELECT * FROM quran_verse 
WHERE revelation_place INSIDE ['Makkah'];
```

### NULL Checks

```surql
-- Find verses without translations
SELECT * FROM quran_verse 
WHERE ->translated_as->translation IS NONE
LIMIT 10;

-- Find scholars without death date
SELECT * FROM scholar 
WHERE death_year_ah IS NONE;
```

### Range Queries

```surql
-- Verses in Juz 30
SELECT * FROM quran_verse 
WHERE juz_number = 30;

-- Scholars with reputation between 5 and 10
SELECT * FROM scholar 
WHERE reputation >= 5.0 AND reputation <= 10.0;
```

---

## 5. Aggregations

### Grouping

```surql
-- Count verses per surah
SELECT surah_number, count() AS verse_count 
FROM quran_verse 
GROUP BY surah_number
ORDER BY surah_number;

-- Count hadiths per collection
SELECT collection, count() AS total 
FROM hadith 
GROUP BY collection;
```

### Math Functions

```surql
-- Average scholar reputation
SELECT math::mean(reputation) AS avg_reputation 
FROM scholar 
WHERE status = 'active';

-- Min/Max hadith numbers
SELECT 
    math::min(hadith_number) AS first,
    math::max(hadith_number) AS last 
FROM hadith 
WHERE collection = 'bukhari';
```

### Array Functions

```surql
-- Count edges
SELECT 
    id,
    array::len(->has_root->root_word) AS root_count 
FROM quran_verse 
LIMIT 10;
```

---

## 6. Advanced Patterns

### Subqueries

```surql
-- Find verses that discuss concepts related to prayer
SELECT * FROM quran_verse 
WHERE ->discusses->concept.name_en CONTAINS 'prayer';

-- Find scholars who graded weak hadiths
SELECT DISTINCT <-graded<-scholar 
FROM hadith 
WHERE <-graded.rank = 'Daif';
```

### Transactions

```surql
BEGIN TRANSACTION;

-- Create a ruling with edge
LET $ruling = CREATE fiqh_ruling SET
    text = 'Digital currency trading requires caution',
    hukm = 'Makruh',
    issued_by = scholar:contemporary_scholar;

-- Link to evidence
RELATE $ruling->derived_from->quran_verse:2_275;

COMMIT TRANSACTION;
```

### Conditional Selection

```surql
-- Different labels based on grade
SELECT 
    id,
    collection,
    IF grade = 'Sahih' THEN 'Authentic' 
    ELSE IF grade = 'Hasan' THEN 'Good'
    ELSE 'Needs Review' 
    END AS status
FROM hadith 
LIMIT 10;
```

### Records as Arrays

```surql
-- Get all verse IDs as array
SELECT array::group(id) AS verse_ids 
FROM quran_verse 
WHERE surah_number = 1;
```

---

## 7. Research Examples

### Example 1: Find All Evidence for Riba Prohibition

**Question**: What primary sources support the prohibition of Riba?

```surql
-- Get all rulings about Riba with their evidence
SELECT 
    text,
    hukm,
    issued_by.name_en AS scholar,
    ->derived_from->(quran_verse, hadith) AS evidence
FROM fiqh_ruling 
WHERE text CONTAINS 'riba' 
OR text CONTAINS 'usury'
OR text CONTAINS 'interest';
```

### Example 2: Trace Isnad Chain

**Question**: Who narrated a specific hadith?

```surql
-- Get narration chain for a hadith
SELECT 
    collection,
    hadith_number,
    ->narrated_by->scholar.name_ar AS narrators,
    ->narrated_by.narration_order AS chain_order
FROM hadith:bukhari_1;
```

### Example 3: Find Abrogated Verses

**Question**: Which verses have been abrogated?

```surql
-- Find abrogated verses with their abrogator
SELECT 
    out AS abrogated_verse,
    in AS abrogating_verse,
    attributed_to.name_en AS attributed_scholar,
    consensus
FROM abrogates;
```

### Example 4: Semantic Analysis of a Surah

**Question**: What concepts and roots appear in Surah Al-Fatiha?

```surql
-- Get thematic and linguistic analysis
SELECT 
    id,
    text_uthmani,
    ->has_root->root_word.root_ar AS roots,
    ->discusses->concept.name_en AS themes,
    ->invokes->divine_name.arabic AS divine_names
FROM quran_verse 
WHERE surah_number = 1
ORDER BY ayah_number;
```

### Example 5: Scholar Reputation Analysis

**Question**: Who are the most trusted scholars with canonical rulings?

```surql
-- Top scholars by reputation with ruling counts
SELECT 
    name_ar,
    name_en,
    reputation,
    status,
    (SELECT count() FROM fiqh_ruling WHERE issued_by = $parent.id AND status = 'canonical') AS canonical_rulings
FROM scholar 
WHERE status = 'active' 
ORDER BY reputation DESC 
LIMIT 10;
```

### Example 6: Cross-Reference Quran to Hadith

**Question**: Which hadiths explain a specific verse?

```surql
-- Find hadith that explain Ayat al-Kursi
SELECT 
    <-explains<-hadith.collection AS collection,
    <-explains<-hadith.hadith_number AS number,
    <-explains<-hadith.matn_en AS text
FROM quran_verse:2_255;
```

### Example 7: Knowledge Domain Exploration

**Question**: What types of knowledge are considered Fard Ayn?

```surql
-- Get individual obligations with their rulings
SELECT 
    name_en,
    description_en,
    obligation_type,
    <-categorized_as<-fiqh_ruling AS related_rulings
FROM knowledge_domain 
WHERE obligation_type = 'fard_ayn';
```

---

## Quick Reference

### Operators

| Operator | Meaning | Example |
|----------|---------|---------|
| `=` | Equals | `WHERE field = 'value'` |
| `!=` | Not equals | `WHERE field != 'value'` |
| `>`, `<` | Comparison | `WHERE num > 5` |
| `>=`, `<=` | Inclusive comparison | `WHERE num >= 5` |
| `CONTAINS` | String contains | `WHERE text CONTAINS 'word'` |
| `INSIDE` | In list | `WHERE val INSIDE ['a','b']` |
| `IS NONE` | Is null | `WHERE field IS NONE` |
| `IS NOT NONE` | Is not null | `WHERE field IS NOT NONE` |

### Graph Operators

| Operator | Direction | Example |
|----------|-----------|---------|
| `->` | Outbound | `->has_root->root_word` |
| `<-` | Inbound | `<-derived_from<-fiqh_ruling` |
| `<->` | Both directions | `<->relates<->` |

### Useful Functions

| Function | Purpose | Example |
|----------|---------|---------|
| `count()` | Count records | `SELECT count() FROM table` |
| `array::len()` | Array length | `array::len(->edges)` |
| `math::mean()` | Average | `math::mean(field)` |
| `time::now()` | Current time | `SET created_at = time::now()` |
| `string::lowercase()` | Lowercase | `string::lowercase(field)` |

---

## Next Steps

- **[Schema Reference](./SCHEMA_REFERENCE.md)** - Understand the data model
- **[API Reference](./API_REFERENCE.md)** - Query via REST API
- **[Glossary](./GLOSSARY.md)** - Terminology definitions

---

*This tutorial is maintained by the Technical Writing Agent. Last updated: 2026-01-08*
