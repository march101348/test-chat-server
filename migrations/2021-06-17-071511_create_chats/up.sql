CREATE TABLE chats (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  room_id INTEGER NOT NULL,
  content VARCHAR NOT NULL
);