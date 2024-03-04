# Rustのクレート"SQLX"を試すリポジトリ
## 試したこと
### 複合型
**これはうまくいかなかった**  

#### スキーマ  
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
#### うまくいかなかった原因
FromRowを手動で実装する必要があったが、```try_get()```の引数にindexやカラム名を入れる必要があるが、複合型の場合うまくパースしてくれず、NotFoundになってしまった．
カラムのインデックスを取得すると  
0: id, 1: name, 2: expiration_date, 3: used  
になっており、それ以降の階層を取得できない
```(expiration_date).year```も使用不可であった．
```rust
impl FromRow<'_, PgRow> for Item {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            expiration_date: ExDate {
                year: row.try_get(2)?, //ここが問題
                month: row.try_get(3)?,　//ここが問題
                day: row.try_get(4)?,　//ここが問題
            },
            used: row.try_get(5)?,
        })
    }
}
```

更に分割することを試したが、Decodeトレイトを手動で実装できたものの、Typeトレイトは手動実装できなかったため、断念
```rust
impl FromRow<'_, PgRow> for ExDate {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            year: row.try_get("year")?,
            month: row.try_get("month")?,
            day: row.try_get("day")?
        })
    }
}

impl<'r> Decode<'r, Postgres> for ExDate {
    fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let (year, month, day) = <(i32, i32, i32) as Decode<Postgres>>::decode(value)?;
        Ok(ExDate { year, month, day })
    }
}
```

#### 解決策
1. スキーマの再設計  
データベースでは複合型は使わず、それぞれのカラムを作成
```sql
CREATE TABLE IF NOT EXISTS item (
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    year            INTEGER NOT NULL,
    month           INTEGER NOT NULL,
    day             INTEGER NOT NULL,
    used            BOOLEAN NOT NULL DEFAULT FALSE
);
```

2. Rustコード内でFromRowトレイトの手動実装  
データベース内ではそれぞれのカラムになっているが，Rustコード内では構造体にまとめるようにトレイトを実装  
これによって```SerdeJson```でシリアライズした時にJsonが扱いやすくなる
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub expiration_date: ExDate,
    pub used: bool,
}

pub struct CreateItem {
    pub name: String,
    pub expiration_date: ExDate,
}

impl FromRow<'_, PgRow> for Item {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            expiration_date: ExDate {
                year: row.try_get("year")?, // それぞれのindexを取得
                month: row.try_get("month")?,
                day: row.try_get("day")?,
            },
            used: row.try_get("used")?,
        })
    }
}
```

出力は以下のようになる(example)
```
Item {
    id: 5,
    name: "potato",
    expiration_date: ExDate {
        year: 2024,
        month: 5,
        day: 25,
    },
    used: false,
}
```