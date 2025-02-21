DROP TABLE IF EXISTS court_bulletin;

DROP TABLE IF EXISTS constitutional_court_bulletin;

DROP TABLE IF EXISTS court_case;

DROP TABLE IF EXISTS constitutional_court_case;

CREATE TABLE IF NOT EXISTS court_case (
    collected_time TIMESTAMPTZ NOT NULL default now (), -- 이 판례를 처음으로 발견한 시간
    case_title TEXT NOT NULL,
    case_subtitle TEXT,
    case_code TEXT NOT NULL,
    court_name TEXT NOT NULL,
    case_type TEXT NOT NULL,
    decision_date DATE NOT NULL,
    en_bank BOOLEAN NOT NULL decision_type TEXT NOT NULL,
    summary JSON NOT NULL, -- 판시사항
    PRIMARY key (court_name, case_code)
);

CREATE TABLE IF NOT EXISTS constitutional_case (
    collected_time TIMESTAMPTZ NOT NULL default now (), -- 이 판례를 처음으로 발견한 시간
    case_title TEXT NOT NULL,
    case_subtitle TEXT,
    case_code TEXT NOT NULL,
    case_type TEXT NOT NULL,
    decision_date DATE NOT NULL, -- YYYY-MM-DD 형태의 문자열
    summary TEXT NOT NULL, -- 판시사항.
    PRIMARY key (case_code)
);

CREATE TABLE IF NOT EXISTS court_bulletin (
    collected_time TIMESTAMPTZ NOT NULL default now (), -- 해당 판례가 공보에 포함된다는 것을 처음으로 발견한 시간
    court_name TEXT NOT NULL,
    case_code TEXT NOT NULL,
    bulletin_code TEXT NOT NULL PRIMARY KEY,
    UNIQUE (court_name, case_code),
    FOREIGN KEY (court_name, case_code) REFERENCES court_case (court_name, case_code) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS constitutional_bulletin (
    collected_time TIMESTAMPTZ NOT NULL default now (), -- 해당 판례가 공보에 포함된다는 것을 처음으로 발견한 시간
    case_code TEXT NOT NULL,
    bulletin_code TEXT NOT NULL PRIMARY KEY,
    FOREIGN KEY (case_code) REFERENCES constitutional_court_case (case_code) ON DELETE CASCADE
);
