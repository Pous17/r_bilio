-- Drop the foreign key constraint for `borrows` table
ALTER TABLE borrows DROP CONSTRAINT IF EXISTS borrows_user_id_fkey;
ALTER TABLE borrows DROP CONSTRAINT IF EXISTS borrows_book_id_fkey;

-- Drop the tables in the reverse order
DROP TABLE IF EXISTS borrows;
DROP TABLE IF EXISTS books;
DROP TABLE IF EXISTS authors;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS employees;