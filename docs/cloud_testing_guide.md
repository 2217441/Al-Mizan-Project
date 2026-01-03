# Surrealist Cloud Testing Guide

**Purpose:** Rapid testing of the Al-Mizan Graph using SurrealDB Free Cloud Tier (1GB).

---

## 🚀 1. Setup Cloud Instance

1. Go to **[surrealdb.com/cloud](https://surrealdb.com/cloud)** and Log in / Sign up.
2. Click **Create Instance**.
3. **Settings:**
    * **Tier:** Free (Sandbox)
    * **Region:** Choose closest (e.g., Singapore/London)
    * **Name:** `al-mizan-test`
4. Wait for the instance to be "Ready".
5. Click **Connect** -> **Open in Surrealist**.

---

## 🛠️ 2. Initialize Schema (Copy & Paste)

In the Surrealist **SQL Query** view, copy the contents of `database/schema/schema.surql`.

> **Note:** Ensure you are using the latest version with the "Tawhidic Foundation" section.

**Run Query:**

```surql
-- Paste `schema.surql` content here
-- Click "Run Query" (Review results for green checkmarks)
```

**Verify:**

```surql
INFO FOR DB;
-- Should list tables: allah, prophet, quran_verse, etc.
```

---

## 🕌 3. Seed Tawhidic Foundation

Copy content from `database/seeds/tawhidic_foundation.surql`.

**Run Query:**

```surql
-- Paste `tawhidic_foundation.surql` content here
-- Click "Run Query"
```

**Verify:**

```surql
SELECT * FROM allah:tawhid;
-- Should return 1 record with essence='al-Wujud al-Mutlaq'
```

---

## 📖 4. Load Test Data (Juz 30 + 40 Hadith)

We generated simplified SQL files for quick cloud loading.

### A. Juz 30 (Surahs 78-114)

**File:** `almizan-etl/output/cloud_juz30.surql`
*(Open this file locally, copy all text, paste into Surrealist)*

**Run Query:**  (This creates ~560 records)

```surql
-- Paste content of cloud_juz30.surql
```

### B. 40 Hadith Nawawi

**File:** `almizan-etl/output/cloud_hadith40.surql`

**Run Query:** (Creates 42 records)

```surql
-- Paste content of cloud_hadith40.surql
```

---

## 🕸️ 5. Visualize the Graph

Surrealist has a built-in **Visualizer**.

1. Click the **"Explorer"** (Table icon) in the left sidebar.
2. Select the `minimized` view or specific table (e.g., `quran_verse`).
3. **OR** Run a specific Graph Query in the **SQL Query** view, then switch the Output View to **"Visualizer"** (Network Icon).

### Try this Graph Visualization Query

Copy this query to see the **Revelation Chain** visually:

```surql
SELECT * FROM 
    allah:tawhid, 
    angel:jibril, 
    prophet:muhammad, 
    (SELECT * FROM quran_verse WHERE surah_number = 112)
FETCH 
    ->revealed_quran, 
    ->chosen_by, 
    ->narrated_quran;
```

## ⚡ 6. Using GraphQL (Optional)

> **⚠️ troubleshooting:** If you get `null`, ensure **Namespace (idc)** and **Database (main)** are selected in the top-right / connection settings.

**Query:**

```graphql
query {
    quran_verse {
        id
        text_simple
        surah_number
    }
}
```

*Note: GraphQL returns a JSON "tree". For a visual "network" graph, use the **Visualizer** in Step 5.*

---

## ⚡ 6. Using GraphQL

If you prefer the **GraphQL** tab in Surrealist:

> **Note:** GraphQL in SurrealDB is best for simple fetching. Complex graph traversals (like `<-narrated_quran`) are often easier in SurrealQL.

**Try this query to fetch Verses:**

```graphql
query {
    quran_verse {
        id
        text_simple
        surah_number
        ayah_number
    }
}
```

1. Click the **"Results"** View (Play icon) to see the JSON response.
2. Use `Ctrl + Space` in the editor to see available fields (Introspection).

---

## 🧪 7. Test Queries (SurrealQL)

### 1. Unified Graph Trace

*Retrieve a verse, its hadith explanation, and the prophet who narrated it.*

```surql
SELECT 
    id, 
    surah_number, 
    ayah_number,
    <-narrated_quran<-prophet.name_en AS narrator,
    <-explains<-hadith AS explanation
FROM quran_verse 
WHERE juz_number = 30
LIMIT 5;
```

### 2. Search by Root Word (Simulated)

*(Uses the new `text_simple` field for easy Arabic search)*

```surql
SELECT * FROM quran_verse 
WHERE text_simple CONTAINS 'الرحمن';
```

### 3. Trace Revelation Chain

```surql
SELECT *, 
    ->revealed_quran->prophet->chosen_by->allah 
FROM quran_verse 
LIMIT 1;
```

---

## 🔄 Switching Back to Local

When you are ready to return to local development:

1. Run `docker-compose up -d`
2. Use the `surreal import` CLI commands as usual.
