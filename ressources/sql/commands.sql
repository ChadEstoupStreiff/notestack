INSERT INTO Notes (id, note, date) VALUES (:note_id, :note, NOW())
SELECT id, note, date FROM Notes WHERE id=:note_id