CREATE TABLE team_member
(
    id                 SERIAL PRIMARY KEY,
    name               VARCHAR          NOT NULL,
    work_hours_per_day DOUBLE PRECISION NOT NULL DEFAULT 7.5 CHECK (work_hours_per_day BETWEEN 1 AND 12)
);

CREATE TABLE work_day
(
    id          SERIAL PRIMARY KEY,
    member_id   INTEGER NOT NULL REFERENCES team_member (id) ON DELETE CASCADE,
    date        DATE    NOT NULL,
    utilisation INTEGER NOT NULL DEFAULT 100
);