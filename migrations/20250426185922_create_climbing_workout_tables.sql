-- migrations/0001_create_tables.sql

CREATE TABLE climbing_sessions (
  id UUID PRIMARY KEY,
  date DATE NOT NULL,
  location TEXT NOT NULL,
  style TEXT NOT NULL,
  notes TEXT
);

CREATE TABLE climb_entries (
  id UUID PRIMARY KEY,
  session_id UUID REFERENCES climbing_sessions(id) ON DELETE CASCADE,
  name TEXT,
  grade TEXT NOT NULL,
  attempts SMALLINT NOT NULL,
  sent BOOLEAN NOT NULL,
  reached_top BOOLEAN NOT NULL,
  lead BOOLEAN NOT NULL,
  rests SMALLINT
);

CREATE TABLE workout_sessions (
    id UUID PRIMARY KEY,
    date DATE NOT NULL,
    notes TEXT
);

CREATE TABLE exercise_entries (
    id UUID PRIMARY KEY,
    workout_session_id UUID REFERENCES workout_sessions(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    sets SMALLINT NOT NULL,
    reps SMALLINT NOT NULL,
    weight_lb INT NOT NULL,
    rpe SMALLINT,
    is_main_lift BOOLEAN NOT NULL
);

CREATE TABLE climbing_metrics (
    id UUID PRIMARY KEY,
    date DATE NOT NULL,
    finger_strength_percent_bw REAL,
    max_pullup_percent_bw REAL,
    notes TEXT
);