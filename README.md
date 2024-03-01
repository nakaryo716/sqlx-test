# Rustのクレート"SQLX"を試すリポジトリ
## 試したこと
### 複合型
スキーマ  
```sql
CREATE TYPE EXDATE AS (
    year   INT,
    month  INT,
    day    INT
);

CREATE TABLE IF NOT EXISTS item (
    id                 SERIAL PRIMARY KEY,
    name               TEXT NOT NULL,
    expiration_date    EXDATE NOT NULL,  
    used                BOOLEAN NOT NULL DEFAULT FALSE
);

```
insertする時は```row()```を使用する  
```sql
INSERT INTO item (name, expiration_date) VALUES ('カレー', row((2024, 5, 1)));
```

