-- BISMILLAH: QURAN INGESTION START
BEGIN TRANSACTION;
CREATE verse:1_1 SET 
    text_uthmani = 'بِسْمِ ٱللَّهِ ٱلرَّحْمَـٰنِ ٱلرَّحِيمِ', 
    text_en = 'In the name of Allah, the Entirely Merciful, the Especially Merciful.', 
    surah_number = 1, 
    ayah_number = 1, 
    mutability = 'CONSTANT';
CREATE verse:2_255 SET 
    text_uthmani = 'ٱللَّهُ لَآ إِلَـٰهَ إِلَّا هُوَ ٱلْحَىُّ ٱلْقَيُّومُ...', 
    text_en = 'Allah - there is no deity except Him, the Ever-Living, the Sustainer with self-subsisting existence...', 
    surah_number = 2, 
    ayah_number = 255, 
    mutability = 'CONSTANT';
COMMIT TRANSACTION;
-- QURAN INGESTION END