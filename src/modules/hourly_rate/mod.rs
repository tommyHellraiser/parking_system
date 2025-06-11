use mysql_async::prelude::FromRow;

use crate::get_from_row;

pub type HourlyRateIdType = u64;

#[derive(Debug)]
pub struct HourlyRate {
    pub id: HourlyRateIdType,
    pub name: String,
    pub rate: f64,
}

impl FromRow for HourlyRate {
    fn from_row(row: mysql_async::Row) -> Self
    where
        Self: Sized,
    {
        let table_name = "hourly_rates";
        Self {
            id: get_from_row!(row, HourlyRateIdType, "ID", table_name),
            name: get_from_row!(row, String, "name", table_name),
            rate: get_from_row!(row, f64, "rate", table_name),
        }
    }

    fn from_row_opt(_row: mysql_async::Row) -> Result<Self, mysql_async::FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
