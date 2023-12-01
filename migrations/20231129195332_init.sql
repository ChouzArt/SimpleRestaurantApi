-- Add migration script here
CREATE TABLE orders (
  id UUID PRIMARY KEY,
  table_number INTEGER NOT NULL,
  menu_item_id INTEGER NOT NULL,
  created_at TIMESTAMPTZ NOT NULL
);

CREATE TABLE menu_items (
  id INTEGER PRIMARY KEY,
  item_name TEXT NOT NULL,
  cooking_time INTEGER NOT NULL
);

ALTER TABLE orders ADD FOREIGN KEY (menu_item_id) REFERENCES menu_items (id);
SET timezone = 'Asia/Tokyo';