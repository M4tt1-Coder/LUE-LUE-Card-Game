DROP TABLE chats;

CREATE TABLE chats (
  number_of_messages integer NOT NULL default 0,
  id text PRIMARY KEY,
  game_id TEXT NOT NULL,
  FOREIGN KEY(game_id) REFERENCES games(id)
);
