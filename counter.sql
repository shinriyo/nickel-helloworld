-- スキーマと初期データ
CREATE TABLE counter (
  id SERIAL,
  counter SMALLINT NOT NULL DEFAULT 0
);

INSERT INTO counter (id, counter) VALUES (0, 1);