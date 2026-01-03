# Al-Mizan Data Ingestion Strategy

**Purpose:** Comprehensive roadmap for populating the Tawhidic knowledge graph with existing digital datasets

**Target Schema:** v1.2 (861 lines, 46 indexes, Tawhidic Foundation complete)

---

## 📊 Data Inventory

### ✅ Existing Datasets (Ready)

| Dataset | Location | Records | Status | Priority |
|---------|----------|---------|--------|----------|
| **Quranic Corpus Morphology** | `database/quranic-corpus-morphology-0.4.txt` | ~77K lines | ✅ Ready | **P1** |
| **TanzilQuran 2.1** | Various XML files | 6,236 verses | ✅ Ready | **P1** |
| **Hadith JSONL** | `almizan-etl/hadith_nodes.jsonl` | 34,458 | ⚠️ Needs review | **P2** |
| **Juz 30 (Arabic)** | `almizan-etl/data/juz30_arabic.json` | ~617 verses | ✅ Ready | P3 |
| **40 Hadith** | `almizan-etl/data/hadith40.json` | 40 hadiths | ✅ Ready | P3 |

### 🔄 Pending Data Sources (Human Action Required)

| Source | Contact | Purpose | Timeline |
|--------|---------|---------|----------|
| **SemanticHadithKG** | <amna.kamran@nu.edu.pk> | Hadith graph + Isnad chains | Week 2-3 |
| **Sunnah.com API** | GitHub issue request | Official 6 books (Kutub al-Sittah) | Week 3-4 |
| **OntoDIN** | Research needed | Narrator/Isnad enrichment | Phase 2 |

---

## 🎯 Ingestion Priority Matrix

### Phase 1 (Weeks 1-2): Foundation + P0 Data

| Priority | Dataset | Tables Populated | ETL Script | Estimated Time |
|----------|---------|------------------|------------|----------------|
| **P1-A** | Tawhidic Seed | `allah`, `prophet`, `angel`, `ijma`, `knowledge_domain` | `database/seeds/tawhidic_foundation.surql` | 5 min |
| **P1-B** | TanzilQuran XML | `quran_verse`, `translation` | `almizan-etl/transform_tanzil.py` | 15 min |
| **P1-C** | Quranic Morphology | `root_word`, `has_root` | `almizan-etl/transform_morphology.py` | 30 min |
| **P1-D** | Divine Names (99) | `divine_name`, `manifests_as` | `almizan-etl/transform_divine_names.py` | 5 min |

**Total:** ~55 minutes for core foundation

---

### Phase 2 (Weeks 3-4): Hadith + Enrichment

| Priority | Dataset | Tables Populated | Dependencies |
|----------|---------|------------------|--------------|
| **P2-A** | SemanticHadithKG RDF | `hadith`, `scholar`, `narrated_by`, `graded` | Human: Contact team |
| **P2-B** | Sunnah.com API | `hadith` (canonical 6 books) | Human: API key |
| **P2-C** | 40 Hadith Nawawi | `hadith` (foundational) | None |

---

### Phase 3 (Weeks 5-8): Validation + Scholarly Data

| Priority | Dataset | Purpose |
|----------|---------|---------|
| **P3-A** | Graph integrity check | Fix 22 orphan verses |
| **P3-B** | Madhahib data | Populate `school` table (4 major + historical) |
| **P3-C** | Historical scholars | Populate `scholar` table (classical mujtahidun) |

---

## 🔧 ETL Scripts Status

### Existing Scripts (Verified)

```python
# 1. Tanzil Quran Importer
almizan-etl/transform_tanzil.py
Status: ✅ Production-ready
Input: quran-uthmani.xml
Output: quran_verse + translation tables
Runtime: ~15 min

# 2. Morphology Transformer  
almizan-etl/transform_morphology.py
Status: ✅ Production-ready
Input: quranic-corpus-morphology-0.4.txt
Output: root_word + has_root tables
Runtime: ~30 min

# 3. Divine Names
almizan-etl/transform_divine_names.py
Status: ✅ Production-ready  
Output: divine_name table (99 names)
Runtime: <5 min

# 4. SemanticHadith Ingestion
almizan-etl/ingest_semantichadith.py
Status: ⚠️ Needs update for new schema
Input: SemanticHadith RDF dump
Output: hadith + scholar + narrated_by
Action: Update to match v1.2 schema
```

---

## 📋 Week-by-Week Execution Plan

### **Week 1 (Current):** Schema Deployment + Tawhidic Foundation

```bash
# 1. Deploy schema v1.2
surreal import --conn http://localhost:8000 \
    --user root --pass root \
    --ns idc --db main \
    database/schema/schema.surql

# 2. Import Tawhidic foundation
surreal import --conn http://localhost:8000 \
    --user root --pass root \
    --ns idc --db main \
    database/seeds/tawhidic_foundation.surql

# Verification
surreal sql "SELECT count() FROM prophet" # Should return 25
surreal sql "SELECT * FROM allah:tawhid"  # Should return singleton
```

**Deliverable:** Allah + 25 prophets + 4 angels + revelation chains

---

### **Week 2:** Core Quranic Data

```bash
# 1. Import Quran verses (Tanzil)
cd almizan-etl
python3 transform_tanzil.py --input data/quran-uthmani.xml

# 2. Import morphology
python3 transform_morphology.py

# 3. Import divine names
python3 transform_divine_names.py

# 4. Link prophet Muhammad to Quran narration
surreal sql "
FOR \$verse IN (SELECT id FROM quran_verse) {
    RELATE prophet:muhammad->narrated_quran->\$verse.id;
};
"
```

**Deliverable:** 6,236 verses + morphology + 99 divine names

---

### **Week 3:** Human Actions + Hadith Prep

**Human Tasks:**

1. ✉️ Email SemanticHadith team for RDF dump access
2. 🔑 Request Sunnah.com API key via GitHub
3. 🔍 Research OntoDIN contact information

**Technical Tasks:**

```bash
# Update ETL scripts for v1.2 schema
git checkout -b feat/hadith-etl-v1.2

# Modify ingest_semantichadith.py:
# - Add hadith_type field (qudsi vs nabawi detection)
# - Map to new scholar schema (status, reputation fields)
# - Generate narration chains (narrated_by edges)
```

**Deliverable:** Updated ETL scripts + pending data requests

---

### **Week 4:** Hadith Import (Conditional)

**If SemanticHadith data received:**

```python
# Import SemanticHadith RDF
python3 almizan-etl/ingest_semantichadith.py \
    --rdf semantichadith.ttl \
    --output hadith_import.surql

surreal import hadith_import.surql
```

**Fallback (if pending):**

```bash
# Import 40 Hadith Nawawi as placeholder
python3 almizan-etl/transform_hadith.py \
    --input data/hadith40.json
```

**Deliverable:** Minimum 40 canonical hadiths in graph

---

## 🔍 Data Quality Checklist

### Post-Import Validation

```surql
-- 1. Verify record counts
SELECT count() FROM quran_verse;      -- Should be 6,236
SELECT count() FROM prophet;          -- Should be 25
SELECT count() FROM angel;            -- Should be 4
SELECT count() FROM divine_name;      -- Should be 99
SELECT count() FROM root_word;        -- Should be ~2,000+

-- 2. Check Tawhidic traceability
-- Every prophet should be chosen by Allah
SELECT count() FROM chosen_by;        -- Should be 25

-- 3. Verify revelation chain
SELECT * FROM allah:tawhid->revealed_quran->prophet:muhammad;

-- 4. Check for orphaned verses (CRITICAL)
SELECT * FROM quran_verse WHERE count(->has_root) = 0 LIMIT 25;

-- 5. Verify indexes exist
SELECT name FROM sys::index WHERE name LIKE '%_idx';
```

---

## 🐛 Known Issues & Resolutions

### Issue #1: Hadith JSONL Placeholder Text

**Problem:** `hadith_nodes.jsonl` contains `"body": "0@en"` placeholder text

**Root Cause:** Incomplete SemanticHadith extraction

**Resolution:**

1. **Short-term:** Skip this file, use 40 Hadith Nawawi instead
2. **Long-term:** Re-fetch from SemanticHadith official RDF dump

---

### Issue #2: 22 Orphan Verses

**Problem:** Some verses lack morphological root links

**Detection:**

```bash
cd almizan-etl  
python3 verify_graph_integrity.py
```

**Resolution:**

1. Generate report of affected verses
2. Cross-reference with Quranic Corpus website
3. Manual enrichment OR wait for complete morphology dataset

---

### Issue #3: Missing Arabic Text

**Problem:** Some datasets only have English translations

**Resolution:**

- Prioritize Tanzil XML (has both Uthmani + Simple Arabic)
- Use `text_uthmani` as canonical, `text_simple` for search
- Mark `matn_en` as optional in hadith table

---

## 📦 Deployment Workflow

### Development → Staging → Production

```bash
# 1. Development (local)
docker-compose up -d
surreal import schema.surql
surreal import seeds/*.surql
python3 almizan-etl/transform_*.py

# 2. Staging
# Export from dev
surreal export --conn http://localhost:8000 \
    --user root --pass root \
    --ns idc --db main \
    staging_backup_$(date +%Y%m%d).surql

# Import to staging server
surreal import staging_backup_20260103.surql

# 3. Production (after validation)
# Same process with production credentials
```

---

## 🎯 Success Metrics

### Week 1 Targets

- [x] Schema v1.2 deployed
- [x] Tawhidic foundation seeded (Allah + 25 prophets + 4 angels)
- [ ] 6,236 Quran verses imported
- [ ] 99 divine names imported
- [ ] Zero orphaned divine references

### Week 2 Targets

- [ ] ~2,000 Arabic root words imported
- [ ] ~10K morphology edges created
- [ ] 100% verse → root_word coverage (or document exceptions)

### Week 3-4 Targets

- [ ] SemanticHadith data received
- [ ] Sunnah.com API key acquired
- [ ] Minimum 1,000 hadiths imported with full isnad chains

---

## 📞 Data Sources Contact Log

| Date | Contact | Purpose | Status |
|------|---------|---------|--------|
| - | <amna.kamran@nu.edu.pk> | SemanticHadith RDF access | ⏳ Pending |
| - | GitHub: sunnah.com | API key request | ⏳ Pending |
| - | TBD | OntoDIN research | 🔍 Researching |

---

## 🔮 Future Enhancements (Phase 2+)

1. **OCR Pipeline** (User mentioned)
   - Scan classical fiqh texts (PDF → Tesseract → NLP)
   - Extract rulings + citations
   - Auto-populate `fiqh_ruling` table

2. **API Integrations**
   - Quran.com API (recitations, tafsir)
   - IslamQA.info (contemporary fatwas)
   - Hadith.com (additional collections)

3. **Crowdsourcing Platform**
   - Scholar dashboard for manual enrichment
   - Validation workflows
   - Ijma consensus building

---

## 📄 Related Documents

- [Implementation Plan](file:///home/a/.gemini/antigravity/brain/a2a9601e-ffb3-48b2-95bc-6ae84fbeac80/implementation_plan.md) - 8-month roadmap
- [TODO_HUMAN_ACTION.md](file:///home/a/code/al-mizan-project/docs/archive/TODO_HUMAN_ACTION.md) - Your pending actions
- [Week 1 Progress](file:///home/a/.gemini/antigravity/brain/a2a9601e-ffb3-48b2-95bc-6ae84fbeac80/week1_progress.md) - P0 fixes completed

**Status:** Ready for execution after schema deployment
