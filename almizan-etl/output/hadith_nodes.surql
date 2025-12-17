-- HADITH INGESTION START
BEGIN TRANSACTION;
CREATE hadith:bukhari_1 SET 
    collection = 'bukhari', 
    hadith_number = 1, 
    matn_ar = 'إِنَّمَا الأَعْمَالُ بِالنِّيَّاتِ', 
    matn_en = 'Actions are but by intentions.', 
    grading = 'Sahih', 
    mutability = 'CONSTANT';
CREATE hadith:mawdu_collection_999 SET 
    collection = 'mawdu_collection', 
    hadith_number = 999, 
    matn_ar = 'FAKE TEXT', 
    matn_en = 'This is a fabricated hadith for testing the Dajjal Filter.', 
    grading = 'Mawdu', 
    mutability = 'CONSTANT';
COMMIT TRANSACTION;
-- HADITH INGESTION END