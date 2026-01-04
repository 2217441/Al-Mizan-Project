# Quick Start Guide

> **Time to complete**: 5 minutes  
> **Audience**: Researchers, Developers  
> **Prerequisites**: None

---

## Welcome to Al-Mizan

Al-Mizan is an Islamic Knowledge Graph that maps relationships between Primary Sources (Quran, Hadith) and Secondary Rulings (Fiqh). This guide will have you querying the graph in under 5 minutes.

---

## Step 1: Access the System

### Option A: Use the Live Demo

Navigate to the landing page:
```
http://localhost:3000/landing
```

### Option B: Use the API Directly

All examples use `curl`. You can also use Postman, Insomnia, or any HTTP client.

---

## Step 2: Query Your First Verse

Let's retrieve the opening verse of the Quran (Al-Fatiha 1:1):

```bash
curl http://localhost:3000/api/v1/verse/1/1
```

**Response**:
```json
{
  "id": "quran_verse:1_1",
  "surah": 1,
  "ayah": 1,
  "text_uthmani": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
  "juz": 1,
  "place": "Makkah"
}
```

🎉 You just retrieved your first verse from the Knowledge Graph!

---

## Step 3: Include Linguistic Analysis

Add `?include_roots=true` to get Arabic root word analysis:

```bash
curl "http://localhost:3000/api/v1/verse/1/1?include_roots=true"
```

**Response** (includes roots):
```json
{
  "id": "quran_verse:1_1",
  "surah": 1,
  "ayah": 1,
  "text_uthmani": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
  "juz": 1,
  "place": "Makkah",
  "roots": ["ب س م", "ا ل ه", "ر ح م"]
}
```

The roots show the trilateral Arabic roots present in the verse.

---

## Step 4: Query a Hadith

Retrieve the first hadith from Sahih al-Bukhari:

```bash
curl http://localhost:3000/api/v1/hadith/bukhari/1
```

**Response**:
```json
{
  "id": "hadith:bukhari_1_1",
  "collection": "bukhari",
  "book_number": 1,
  "hadith_number": 1,
  "text": "Actions are judged by intentions...",
  "grade": "Sahih"
}
```

---

## Step 5: Explore the Knowledge Graph

The real power of Al-Mizan is in its graph structure. Retrieve the Tawhidic Knowledge Graph:

```bash
curl http://localhost:3000/api/v1/graph
```

This returns nodes and edges showing:
- **Allah** as the ontological root
- **Prophets** chosen by Allah
- **Quran Verses** narrated by prophets
- **Hadith** transmitted by narrators

You can visualize this at:
```
http://localhost:3000/graph
```

---

## Step 6: Check Scholarly Consensus

Use the Synthesis Engine to check ruling status on a topic:

```bash
curl -X POST http://localhost:3000/api/v1/synthesis \
  -H "Content-Type: application/json" \
  -d '{"topic": "gold"}'
```

**Response**:
```json
{
  "@context": "http://schema.org",
  "@type": "FinancialProduct",
  "status": "Green",
  "ruling_status": "http://schema.org/Approved",
  "consensus_score": 1.0,
  "summary": "Permissible (Majority). Gold is the standard of value in Islamic Finance.",
  "primary_scholar": "Imam Malik"
}
```

| Status | Meaning |
|--------|---------|
| 🟢 Green | Permissible (Halal) |
| 🟡 Yellow | Disputed/Pending |
| 🔴 Red | Prohibited (Haram) |

---

## Step 7: Explore Divine Names

Retrieve the 99 Names of Allah:

```bash
curl http://localhost:3000/api/v1/names
```

Or get a specific name:

```bash
curl http://localhost:3000/api/v1/names/1
```

---

## What's Next?

### For Researchers

- **[API Reference](./API_REFERENCE.md)** - Complete endpoint documentation
- **[Schema Reference](./SCHEMA_REFERENCE.md)** - Understand the data model *(coming soon)*
- **[Query Tutorial](./QUERY_TUTORIAL.md)** - Advanced SurQL queries *(coming soon)*

### For Developers

- **[Developer Setup](../../CONTRIBUTING.md)** - Local development environment
- **[Architecture Overview](../ARCHITECTURE.md)** - System design
- **[ETL Pipeline](../data_ingestion_strategy.md)** - Data ingestion

### Interactive Tools

| Tool | URL | Description |
|------|-----|-------------|
| Landing Page | `/landing` | Project overview |
| Graph Viewer | `/graph` | Interactive knowledge graph |
| API Playground | `/playground` | Test API calls |
| Governance Console | `/governance` | Ruling management |

---

## Common Issues

### Connection Refused

If you get `connection refused`, ensure the server is running:

```bash
cd /home/a/code/al-mizan-project/almizan-core
cargo run
```

### Database Not Connected

Ensure SurrealDB is running:

```bash
docker-compose up -d surreal
```

### Empty Responses

The database may not have data loaded. Run the ETL pipeline:

```bash
cd /home/a/code/al-mizan-project/almizan-etl
python run_etl.py
```

---

## Getting Help

- **GitHub Issues**: Report bugs or request features
- **Documentation**: Check the `/docs` folder
- **API Playground**: Test endpoints at `/playground`

---

*You're now ready to explore the Islamic Knowledge Graph! 🎉*

---

*Last updated: 2026-01-04*
