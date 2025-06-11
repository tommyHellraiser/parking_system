use mysql_async::prelude::FromRow;
use mysql_async::{FromRowError, Row};
use chrono::NaiveDateTime;

use crate::get_from_row;
use crate::modules::car_details::CarDetailsIdType;
use crate::modules::hourly_rate::HourlyRateIdType;
use crate::modules::parking_lots::ParkingLotIdType;

pub type ParkingShiftIdType = u64;

#[derive(Debug)]
pub struct ParkingShift {
    pub id: ParkingShiftIdType,
    pub car_id: CarDetailsIdType,
    pub lot_id: ParkingLotIdType,
    pub rate_id: HourlyRateIdType,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
}

impl FromRow for ParkingShift {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table_name = "parking_shifts";
        Self {
            id: get_from_row!(row, ParkingShiftIdType, "ID", table_name),
            car_id: get_from_row!(row, CarDetailsIdType, "car_ID", table_name),
            lot_id: get_from_row!(row, ParkingLotIdType, "lot_ID", table_name),
            rate_id: get_from_row!(row, HourlyRateIdType, "rate_ID", table_name),
            start_time: get_from_row!(row, NaiveDateTime, "start_time", table_name),
            end_time: get_from_row!(row, Option<NaiveDateTime>, "end_time", table_name),
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}