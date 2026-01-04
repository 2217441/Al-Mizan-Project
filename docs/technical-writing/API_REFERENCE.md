# Al-Mizan API Reference

> **Version**: v1.0  
> **Base URL**: `http://localhost:3000`  
> **Author**: Technical Writing Agent  
> **Last Updated**: 2026-01-04

---

## Overview

The Al-Mizan API provides programmatic access to the Islamic Knowledge Graph, enabling researchers and developers to query Quranic verses, hadith collections, scholarly rulings, and semantic relationships between primary sources.

### Authentication

Most endpoints are public (read-only). Protected endpoints require JWT authentication obtained via the `/auth/signin` endpoint.

```bash
# Include token in requests
curl -H "Authorization: Bearer <token>" https://api.example.com/api/v1/...
```

### Response Format

All responses are JSON with consistent error handling:

**Success Response**:
```json
{
  "field": "value",
  ...
}
```

**Error Response**:
```json
{
  "error": "Error description"
}
```

---

## Table of Contents

1. [Authentication](#1-authentication)
2. [Quran Verses](#2-quran-verses)
3. [Hadith Collections](#3-hadith-collections)
4. [Knowledge Graph](#4-knowledge-graph)
5. [Synthesis Engine](#5-synthesis-engine)
6. [Divine Names](#6-divine-names)
7. [Evidence Retrieval](#7-evidence-retrieval)
8. [Dashboard](#8-dashboard)
9. [Enterprise](#9-enterprise)
10. [Identity (DID)](#10-identity-did)
11. [Network Operations](#11-network-operations)

---

## 1. Authentication

### `POST /auth/signup`

Create a new user account.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | string | Yes | Valid email address |
| `password` | string | Yes | User password |

**Example**:
```bash
curl -X POST http://localhost:3000/auth/signup \
  -H "Content-Type: application/json" \
  -d '{"email": "researcher@example.com", "password": "secure123"}'
```

**Response**:
- `201 Created`: Account created successfully
- `500 Internal Server Error`: Database or hashing error

---

### `POST /auth/signin`

Authenticate and receive a JWT token.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `email` | string | Yes | Registered email |
| `password` | string | Yes | Account password |

**Example**:
```bash
curl -X POST http://localhost:3000/auth/signin \
  -H "Content-Type: application/json" \
  -d '{"email": "researcher@example.com", "password": "secure123"}'
```

**Success Response** (`200 OK`):
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Error Response** (`401 Unauthorized`):
- Invalid credentials

> [!NOTE]
> Tokens expire after 24 hours. Re-authenticate to obtain a new token.

---

## 2. Quran Verses

### `GET /api/v1/verse/{surah}/{ayah}`

Retrieve a specific verse by surah and ayah number.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `surah` | integer | Surah number (1-114) |
| `ayah` | integer | Ayah number within surah |

**Query Parameters**:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `include_roots` | boolean | `false` | Include Arabic root words |

**Example**:
```bash
# Get Al-Fatiha verse 1
curl http://localhost:3000/api/v1/verse/1/1

# Include root word analysis
curl "http://localhost:3000/api/v1/verse/1/1?include_roots=true"
```

**Success Response** (`200 OK`):
```json
{
  "id": "quran_verse:1_1",
  "surah": 1,
  "ayah": 1,
  "text_uthmani": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
  "juz": 1,
  "place": "Makkah",
  "roots": ["ب س م", "ا ل ل ه", "ر ح م"]
}
```

**Error Responses**:
- `404 Not Found`: Verse does not exist
- `500 Internal Server Error`: Database query failed

---

### `GET /api/v1/verse/{surah}`

Retrieve all verses in a surah.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `surah` | integer | Surah number (1-114) |

**Example**:
```bash
# Get all verses of Surah Al-Fatiha
curl http://localhost:3000/api/v1/verse/1
```

**Success Response** (`200 OK`):
```json
{
  "surah": 1,
  "count": 7,
  "verses": [
    {
      "id": "quran_verse:1_1",
      "surah": 1,
      "ayah": 1,
      "text_uthmani": "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
      "juz": 1,
      "place": "Makkah"
    },
    ...
  ]
}
```

---

## 3. Hadith Collections

### `GET /api/v1/hadith/{collection}/{number}`

Retrieve a specific hadith by collection and number.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `collection` | string | Collection name (see table below) |
| `number` | float | Hadith number |

**Supported Collections**:

| Collection | Arabic | Description |
|------------|--------|-------------|
| `bukhari` | بخاري | Sahih al-Bukhari |
| `muslim` | مسلم | Sahih Muslim |
| `tirmidhi` | ترمذي | Jami' at-Tirmidhi |
| `abu_dawud` | أبو داود | Sunan Abu Dawud |
| `nasa'i` | نسائي | Sunan an-Nasa'i |
| `ibn_majah` | ابن ماجه | Sunan Ibn Majah |

**Example**:
```bash
# Get Bukhari hadith #1
curl http://localhost:3000/api/v1/hadith/bukhari/1
```

**Success Response** (`200 OK`):
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

**Error Responses**:
- `404 Not Found`: Hadith not found
- `500 Internal Server Error`: Database error

---

### `GET /api/v1/hadith/{collection}`

List hadiths from a collection (paginated).

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `collection` | string | Collection name |

**Example**:
```bash
curl http://localhost:3000/api/v1/hadith/bukhari
```

**Success Response** (`200 OK`):
```json
{
  "collection": "bukhari",
  "count": 50,
  "hadiths": [
    {
      "id": "hadith:bukhari_1_1",
      "collection": "bukhari",
      "book_number": 1,
      "hadith_number": 1,
      "text": "...",
      "grade": "Sahih"
    },
    ...
  ]
}
```

> [!NOTE]
> Results are limited to 50 hadiths per request. Pagination support coming in v1.1.

---

## 4. Knowledge Graph

### `GET /api/v1/graph`

Returns the Tawhidic Knowledge Graph in Cytoscape.js format.

The graph shows epistemological chains:
- **Allah** → Prophets (chosen_by)
- **Prophets** → Quran Verses (narrated)
- **Narrators** → Hadiths (narrated)

**Example**:
```bash
curl http://localhost:3000/api/v1/graph
```

**Success Response** (`200 OK`):
```json
{
  "nodes": [
    {
      "data": {
        "id": "allah:tawhid",
        "label": "الله",
        "type": "allah"
      }
    },
    {
      "data": {
        "id": "prophet:muhammad",
        "label": "محمد",
        "type": "prophet"
      }
    },
    ...
  ],
  "edges": [
    {
      "data": {
        "id": "chosen_prophet:muhammad",
        "source": "allah:tawhid",
        "target": "prophet:muhammad",
        "label": "chose"
      }
    },
    ...
  ]
}
```

**Node Types**:

| Type | Description | Example Label |
|------|-------------|---------------|
| `allah` | The ontological root | الله |
| `prophet` | Prophets (Anbiya) | موسى، عيسى، محمد |
| `verse` | Quran verses | 1:1, 2:255 |
| `hadith` | Hadith records | بخاري 1 |
| `narrator` | Hadith narrators | راوي (ط1) |

**Edge Types**:

| Type | Direction | Description |
|------|-----------|-------------|
| `chose` | Allah → Prophet | Divine selection |
| `narrated` | Prophet/Narrator → Verse/Hadith | Transmission |

---

## 5. Synthesis Engine

### `POST /api/v1/synthesis`

Synthesize scholarly consensus on a topic. Returns a ruling status with attribution.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `topic` | string | Yes | Topic to analyze (e.g., "bitcoin", "riba", "gold") |
| `strictness` | string | No | `"strict"` (default) or `"loose"` |
| `strictness_level` | string | No | `"basic"`, `"standard"`, `"high"`, `"extreme"` |

**Example**:
```bash
curl -X POST http://localhost:3000/api/v1/synthesis \
  -H "Content-Type: application/json" \
  -d '{"topic": "bitcoin", "strictness": "strict"}'
```

**Success Response** (`200 OK`):
```json
{
  "@context": "http://schema.org",
  "@type": "FinancialProduct",
  "status": "Yellow",
  "ruling_status": "http://schema.org/Pending",
  "consensus_score": 0.4,
  "summary": "Disputed (Strict Default). Significant scholarly disagreement regarding Gharar...",
  "primary_scholar": "Imam Al-Ghazali (Derived)",
  "scholar_avatar": "https://api.dicebear.com/7.x/shapes/svg?seed=Ghazali"
}
```

**Status Values**:

| Status | Color | Meaning |
|--------|-------|---------|
| `Green` | 🟢 | Permissible (Halal) |
| `Yellow` | 🟡 | Disputed/Pending |
| `Red` | 🔴 | Prohibited (Haram) |

**Response Headers**:
- `X-Disclaimer`: "Advisory only. Consult local state Mufti for binding rulings."

> [!IMPORTANT]
> This API provides advisory information only. All rulings should be verified with qualified scholars in your jurisdiction.

---

## 6. Divine Names

### `GET /api/v1/names`

Retrieve all 99 Names of Allah (Asma ul Husna).

**Example**:
```bash
curl http://localhost:3000/api/v1/names
```

**Success Response** (`200 OK`):
```json
{
  "count": 99,
  "names": [
    {
      "id": 1,
      "arabic": "الرَّحْمَنُ",
      "transliteration": "Ar-Rahman",
      "meaning": "The Most Gracious"
    },
    {
      "id": 2,
      "arabic": "الرَّحِيمُ",
      "transliteration": "Ar-Raheem",
      "meaning": "The Most Merciful"
    },
    ...
  ]
}
```

---

### `GET /api/v1/names/{id}`

Retrieve a specific divine name by ID.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Name ID (1-99) |

**Example**:
```bash
curl http://localhost:3000/api/v1/names/1
```

**Success Response** (`200 OK`):
```json
{
  "id": 1,
  "arabic": "الرَّحْمَنُ",
  "transliteration": "Ar-Rahman",
  "meaning": "The Most Gracious"
}
```

---

## 7. Evidence Retrieval

### `GET /api/v1/evidence/{id}`

Retrieve primary source evidence for a ruling.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | string | Ruling ID (e.g., `fiqh_ruling:123`) |

**Example**:
```bash
curl http://localhost:3000/api/v1/evidence/fiqh_ruling:123
```

**Success Response** (`200 OK`):
```json
{
  "ruling_id": "fiqh_ruling:123",
  "evidence": [
    {
      "type": "quran_verse",
      "id": "quran_verse:2_275",
      "text": "...أَحَلَّ اللَّهُ الْبَيْعَ وَحَرَّمَ الرِّبَا..."
    },
    {
      "type": "hadith",
      "id": "hadith:bukhari_2084",
      "text": "..."
    }
  ]
}
```

---

## 8. Dashboard

### `POST /api/v1/dashboard`

Get role-specific dashboard content.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `auth_token` | string | Yes | Role token (see below) |

**Token Roles**:

| Token | Role | Dashboard Type |
|-------|------|----------------|
| `MUJTAHID_KEY_786` | Mujtahid (Admin) | Theological Depth |
| Any other value | Public | Research Dashboard |

**Example**:
```bash
curl -X POST http://localhost:3000/api/v1/dashboard \
  -H "Content-Type: application/json" \
  -d '{"auth_token": "public_token"}'
```

**Success Response** (`200 OK`):
```json
{
  "dashboard_title": "Public Research Dashboard - Trust: 92%",
  "modules": [
    "Dataset Statistics",
    "Ontology Coverage Map",
    "Recent Ingestions"
  ]
}
```

---

## 9. Enterprise

### `GET /api/v1/enterprise/metrics`

Get trust metrics for a scholar.

**Query Parameters**:

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `scholar_id` | string | Yes | Scholar identifier |

**Example**:
```bash
curl "http://localhost:3000/api/v1/enterprise/metrics?scholar_id=scholar_sys_001"
```

**Success Response** (`200 OK`):
```json
{
  "scholar_id": "scholar_sys_001",
  "reliability_score": 0.92,
  "citation_count": 150,
  "peer_validations": 45
}
```

---

### `POST /api/v1/enterprise/audit`

Generate a compliance report for a document.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `document_id` | string | Yes | Document to audit |

**Example**:
```bash
curl -X POST http://localhost:3000/api/v1/enterprise/audit \
  -H "Content-Type: application/json" \
  -d '{"document_id": "contract_001"}'
```

---

### `POST /api/v1/enterprise/analyze_contract`

Analyze a contract for Shariah compliance.

**Request Body**:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `contract_type` | string | Yes | Type of contract |
| `terms` | array | Yes | Contract terms |
| ... | ... | ... | Additional fields |

---

### `POST /api/v1/enterprise/certify`

Certify a contract and issue a Verifiable Credential.

**Request Body**: Same as `/analyze_contract`

**Success Response** (`200 OK`):
```json
{
  "certified": true,
  "standard": "AAOIFI",
  "badge": "Shariah Compliant",
  "verifiable_credential": {
    "type": ["VerifiableCredential", "ShariahCertification"],
    "issuer": "did:key:z6Mk...",
    "credentialSubject": {
      "id": "did:key:z6Mk...",
      "standard": "AAOIFI",
      "badge": "Shariah Compliant",
      "status": "Active"
    }
  }
}
```

---

## 10. Identity (DID)

### `GET /api/v1/identity/resolve/{did}`

Resolve a Decentralized Identifier (DID) to its document.

**Path Parameters**:

| Parameter | Type | Description |
|-----------|------|-------------|
| `did` | string | DID to resolve (e.g., `did:key:z6Mk...`) |

**Example**:
```bash
curl http://localhost:3000/api/v1/identity/resolve/did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK
```

**Success Response** (`200 OK`):
```json
{
  "@context": ["https://www.w3.org/ns/did/v1"],
  "id": "did:key:z6Mk...",
  "authentication": [
    {
      "type": "Ed25519VerificationKey2018",
      "publicKeyBase58": "..."
    }
  ]
}
```

---

### `POST /api/v1/identity/verify`

Verify a Verifiable Credential.

**Request Body**: Full Verifiable Credential JSON

**Success Response** (`200 OK`):
```json
true
```

---

## 11. Network Operations

### `GET /api/v1/network/export`

Export a snapshot of the knowledge graph.

**Example**:
```bash
curl http://localhost:3000/api/v1/network/export
```

**Success Response** (`200 OK`):
```json
{
  "id": "snapshot_v1_mock",
  "timestamp": "2025-12-03T21:00:00Z",
  "node_count": 150,
  "signature": "sha256:mock_snapshot_signature"
}
```

---

### `POST /api/v1/network/ingest`

Ingest a graph snapshot (federation support).

**Request Body**: Snapshot JSON

**Success Response** (`200 OK`):
```
"Snapshot Ingested Successfully (Mock)"
```

---

## Error Codes

| Status Code | Description |
|-------------|-------------|
| `200 OK` | Request successful |
| `201 Created` | Resource created |
| `400 Bad Request` | Invalid request parameters |
| `401 Unauthorized` | Authentication required or failed |
| `404 Not Found` | Resource not found |
| `500 Internal Server Error` | Server-side error |

---

## Rate Limiting

Currently no rate limiting is enforced. For production deployments, consider:
- 100 requests/minute for authenticated users
- 20 requests/minute for anonymous users

---

## Changelog

| Version | Date | Changes |
|---------|------|---------|
| v1.0 | 2026-01-04 | Initial API documentation |

---

*This document is maintained by the Technical Writing Agent. Last updated: 2026-01-04T20:57:03+08:00*
