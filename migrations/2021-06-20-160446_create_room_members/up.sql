CREATE TABLE room_members (
  user_id INTEGER REFERENCES users(id) ON DELETE cascade ON UPDATE cascade,
  room_id INTEGER REFERENCES rooms(id) ON DELETE cascade ON UPDATE cascade,
  PRIMARY KEY (user_id, room_id)
);