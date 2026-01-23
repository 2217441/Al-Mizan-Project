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

## 2024-10-26 - [Database-side Logic for Text Selection]
**Learning:** Moving conditional text selection (English vs Arabic fallback) to SurrealQL using `IF` statements significantly reduces network payload and eliminates duplicate string allocations in Rust structs.
**Action:** When handling multilingual fields, check if the fallback logic can be executed in the database query instead of the application layer.

## 2024-10-27 - [SurrealDB ID Parsing Overhead]
**Learning:** Found an anti-pattern where `surrealdb::sql::Thing` IDs were being converted to Strings and then parsed back to integers (e.g., `id.to_string().parse()`). This causes unnecessary allocation for every row.
**Action:** Access the `surrealdb::sql::Id` enum variants directly (e.g., `match id { Id::Number(n) => n, ... }`) to avoid intermediate string allocations.
