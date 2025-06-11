use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};

use crate::get_from_row;

pub type ParkingLotIdType = u64;

#[derive(Debug)]
pub struct ParkingLot {
    pub id: ParkingLotIdType,
    pub lot_name: String,
    pub address: String,
}

impl FromRow for ParkingLot {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table_name = "parking_lots";
        Self {
            id: get_from_row!(row, ParkingLotIdType, "ID", table_name),
            lot_name: get_from_row!(row, String, "lot_name", table_name),
            address: get_from_row!(row, String, "address", table_name),
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}