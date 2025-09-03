-- sqlfluff:dialect:sqlite
-- sqlfluff:indentation:tab_space_size:2

CREATE TABLE IF NOT EXISTS contractors (
  id INTEGER PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  vat_status VARCHAR(50) NOT NULL,
  nip VARCHAR(10) NOT NULL,
  regon VARCHAR(14),
  krs VARCHAR(10),
  residence_address TEXT,
  working_address TEXT
);

CREATE TABLE IF NOT EXISTS account_numbers (
  id INTEGER PRIMARY KEY,
  account_number VARCHAR(34) NOT NULL,
  contractor_id INTEGER NOT NULL,
  FOREIGN KEY (contractor_id) REFERENCES contractors (id)
);
