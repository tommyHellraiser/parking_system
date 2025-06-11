use mysql_async::{prelude::FromRow, FromRowError, Row};

use crate::get_from_row;

pub type CarDetailsIdType = u64;

#[derive(Debug)]
pub struct CarDetail {
    pub id: CarDetailsIdType,
    pub license_plate: String,
    pub car_color: String,
    pub make: String,
    pub model: String,
    pub year: u16,
}

impl FromRow for CarDetail {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table_name = "car_details";
        Self {
            id: get_from_row!(row, CarDetailsIdType, "ID", table_name),
            license_plate: get_from_row!(row, String, "license_plate", table_name),
            car_color: get_from_row!(row, String, "car_color", table_name),
            make: get_from_row!(row, String, "make", table_name),
            model: get_from_row!(row, String, "model", table_name),
            year: get_from_row!(row, u16, "year", table_name),
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}