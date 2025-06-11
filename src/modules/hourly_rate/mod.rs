use mysql_async::prelude::FromRow;

pub type HourlyRateIdType = u64;

#[derive(Debug)]
pub struct HourlyRate {
    pub id: HourlyRateIdType,
    pub name: String,
    pub rate: f32
}

macro_rules! get_from_row {
    ($row: expr, $type: ty, $col_name: literal, $table_name: expr) => {
        match $row.get::<$type, _>($col_name) {
            Some(field) => field,
            None => {
                let msg = format!("Couldn't get field {} from table {}", $col_name, $table_name);
                std::panic::panic_any(msg);
            }
        }
    };
}

impl FromRow for HourlyRate {
    fn from_row(row: mysql_async::Row) -> Self
        where
            Self: Sized, {
        let table_name = "hourly_rates";
        Self {
            id: get_from_row!(row, HourlyRateIdType, "ID", table_name),
            name: get_from_row!(row, String, "name", table_name),
            rate: get_from_row!(row, f32, "rate", table_name)
        }
    }

    fn from_row_opt(_row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
        where
            Self: Sized {
        unimplemented!()
    }
}
