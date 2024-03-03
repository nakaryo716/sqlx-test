use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};



#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub expiration_date: ExDate,
    pub used: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExDate {
    pub year: i32,
    pub month: i32,
    pub day: i32,
}

pub struct CreateItem {
    pub name: String,
    pub expiration_date: ExDate,
}

impl FromRow<'_, PgRow> for Item {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self { 
            id: row.try_get(0)?,
            name: row.try_get(1)?,
            expiration_date: ExDate {
                year: row.try_get(2)?,
                month: row.try_get(3)?,
                day: row.try_get(4)?,
            },
            used: row.try_get(5)?
        })
    }
}




// impl FromRow<'_, PgRow> for ExDate {
//     fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
//         Ok(Self { 
//             year: row.try_get("year")?,
//             month: row.try_get("month")?,
//             day: row.try_get("day")?
//         })
//     }
// }


// impl<'r> Decode<'r, Postgres> for ExDate {
//     fn decode(value: <Postgres as HasValueRef<'r>>::ValueRef,
//     ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
//         let (year, month, day) = <(i32, i32, i32) as Decode<Postgres>>::decode(value)?;
//         Ok(ExDate { year, month, day })
//     }
// }

// use sqlx::FromRow;


// #[derive(Debug, FromRow)]
// pub struct Item {
//     pub id: i32,
//     pub  name: String,
//     pub year: i32,
//     pub month: i32,
//     pub day: i32,
//     pub used: bool,
// }

// #[derive(Debug)]
// pub struct CreateItem {
//     pub name: String,
//     pub year: i32,
//     pub month: i32,
//     pub day: i32,
// }