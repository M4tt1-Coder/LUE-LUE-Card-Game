-- Migration number: 0001 	 2025-07-09T23:35:32.836Z

PRAGMA foreign_keys = ON;

CREATE TABLE games (
  id TEXT PRIMARY KEY,
  which_player_turn TEXT NOT NUll UNIQUE,
  state INTEGER NOT NULL DEFAULT 0,
  started_at TIMESTAMP NOT null DEFAULT CURRENT_TIMESTAMP,
  round_number integer not null default 0,
  card_to_play integer not null
); 

CREATE TABLE players (
  id text primary key,
  name text not null,
  score integer default 0,
  joined_at timestamp not null DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE cards (
  card_type integer not null default 0,
  player_id text,
  claim_id text,
  id text PRIMARY KEY,
  FOREIGN KEY(player_id) REFERENCES players(id),
  FOREIGN KEY(claim_id) REFERENCES claims(id)
);

CREATE TABLE chats (
  number_of_messages integer NOT NULL default 0,
  id text PRIMARY KEY
);

CREATE TABLE chat_messages (
  id text PRIMARY KEY,
  player_id text Not null,
  content text Not null,
  sent_at timestamp not null DEFAULT CURRENT_TIMESTAMP,
  chat_id text not null,
  FOREIGN KEY(player_id) REFERENCES players(id),
  FOREIGN KEY(chat_id) REFERENCES chats(id)
);

CREATE TABLE claims (
  created_by text NOT null,
  number_of_cards integer not null default 0,
  id text PRIMARY KEY,
  FOREIGN KEY(created_by) REFERENCES players(id)
);

