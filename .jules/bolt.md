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

## 2024-10-28 - [SurrealDB ID Serialization Overhead]
**Learning:** `Thing::to_string()` allocates a new `String` every time. When serializing thousands of items (e.g., verses), this allocation adds up.
**Action:** Implemented `serialize_thing_id` to write `Thing` directly to the `serde` serializer without intermediate string allocation, and updated response structs to hold `Thing` instead of `String`.

## 2024-10-29 - [Cow for Static Strings in Responses]
**Learning:** API response structs with `String` fields cause unnecessary heap allocations when returning static string literals (e.g., status strings like "Green", "Pending").
**Action:** Use `std::borrow::Cow<'a, str>` in response structs and return `Cow::Borrowed("static_string")` to eliminate these allocations.

## 2024-10-30 - [Graph Response Optimization and Stability]
**Learning:** Optimizing Edge IDs by using counters destroyed ID stability, which is bad for clients. Optimizing Node IDs using `GraphId` (holding `Thing` directly) was a pure win.
**Action:** When optimizing IDs, ensure they remain deterministic if the client relies on them for caching/diffing. Use `Cow` or custom enums to hold raw DB types (`Thing`) to avoid intermediate string allocations.
