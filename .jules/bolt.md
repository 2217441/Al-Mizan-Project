# Bolt's Journal

## 2024-10-24 - [Initial Setup]
**Learning:** Initialized Bolt's journal.
**Action:** Record critical performance learnings here.

## 2024-10-24 - [N+1 Query in Verse Fetching]
**Learning:** Found a loop iterating 1..300 executing individual DB selects for each verse in `get_surah`.
**Action:** Replaced with `SELECT * FROM quran_verse WHERE surah_number = $surah ORDER BY ayah_number ASC`. Reduced N queries to 1.

## 2024-10-25 - [Graph Response Allocations]
**Learning:** Discovered significant memory allocation overhead in `get_graph` due to repeated `.to_string()` calls for static string literals in struct fields and unnecessary `sanitize_id` string replacements.
**Action:** Switched `GraphData` structs to use `Cow<'a, str>` to allow `Cow::Borrowed` for static literals, and optimized `sanitize_id` to check for brackets before allocating. Reduced unnecessary allocations by ~260 per request.

## 2024-10-25 - [DB-Side Conditional Logic]
**Learning:** SurrealDB queries can handle conditional logic (IF/ELSE) to reduce payload size and allocation in Rust. Specifically for Hadith text fallback.
**Action:** Identify other places where Rust logic filters fields that could be done in SQL.
