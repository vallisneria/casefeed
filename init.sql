DROP TABLE IF EXISTS bulletin_case;
DROP TABLE IF EXISTS case_summary;
DROP TABLE IF EXISTS court_case;

CREATE TABLE IF NOT EXISTS court_case (
    collected_time  INTEGER default (unixepoch('now')) NOT NULL,    -- 이 판례를 처음으로 발견한 시간
    glaw_id         INTEGER NOT NULL,    -- 법원 종합법률정보 api 상의 일련번호
    case_title      TEXT NOT NULL,
    case_subtitle   TEXT,
    case_code       TEXT NOT NULL,
    court_name      TEXT NOT NULL,
    case_type       TEXT NOT NULL,
    decision_date   TEXT NOT NULL,
    en_bank         INTEGER NOT NULL,    -- BOOLEAN
    decision_type   TEXT NOT NULL,
    PRIMARY key (court_name, case_code)
);

CREATE TABLE IF NOT EXISTS bulletin_case (
    collected_time  INTEGER default (unixepoch('now')) NOT NULL ,    -- 해당 판례가 공보에 포함된다는 것을 처음으로 발견한 시간
    court_name      TEXT NOT NULL,
    case_code       TEXT NOT NULL,
    bulletin_code   TEXT NOT NULL,
    UNIQUE (court_name, case_code),
    FOREIGN KEY (court_name, case_code)
    REFERENCES court_case(court_name, case_code) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS case_summary (
    court_name      TEXT NOT NULL,
    case_code       TEXT NOT NULL,
    main_issue      TEXT NOT NULL,       -- 각 문단이 하나의 item인 JSON Array 형태로 업로드
    summary_of_decision TEXT NOT NULL,   -- 각 문단이 하나의 item인 JSON Array 형태로 업로드
    UNIQUE (court_name, case_code),
    FOREIGN KEY (court_name, case_code)
    REFERENCES court_case(court_name, case_code) ON DELETE CASCADE
);
