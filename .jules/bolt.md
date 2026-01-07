# Bolt's Journal

## 2024-10-24 - [Initial Setup]
**Learning:** Initialized Bolt's journal.
**Action:** Record critical performance learnings here.

## 2024-10-24 - [N+1 Query in Verse Fetching]
**Learning:** Found a loop iterating 1..300 executing individual DB selects for each verse in `get_surah`.
**Action:** Replaced with `SELECT * FROM quran_verse WHERE surah_number = $surah ORDER BY ayah_number ASC`. Reduced N queries to 1.
