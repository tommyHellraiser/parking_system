use std::str::FromStr;

use error_mapper::{create_new_error, TheError};
use mysql_async::prelude::{FromRow, FromValue};
use mysql_async::{FromRowError, FromValueError, Row, Value};
use chrono::NaiveDateTime;

use crate::get_from_row;
use crate::modules::parking_shifts::ParkingShiftIdType;

pub type PaymentIdType = u64;

#[derive(Debug)]
pub enum PaymentMethod {
    Cash,
    Card,
    Digital,
}

#[derive(Debug)]
pub struct Payment {
    pub id: PaymentIdType,
    pub shift_id: ParkingShiftIdType,
    pub amount: f64,
    pub payment_method: PaymentMethod,
    pub payment_time: NaiveDateTime,
}

impl FromRow for Payment {
    fn from_row(row: Row) -> Self
    where
        Self: Sized,
    {
        let table_name = "payments";

        let method = row.get::<PaymentMethod, _>("method").unwrap();

        Self {
            id: get_from_row!(row, PaymentIdType, "ID", table_name),
            shift_id: get_from_row!(row, ParkingShiftIdType, "shift_ID", table_name),
            amount: get_from_row!(row, f64, "amount", table_name),
            payment_method: get_from_row!(row, String, "method", table_name)
                .as_str()
                .parse::<PaymentMethod>()
                .unwrap_or(PaymentMethod::Cash),
            payment_time: get_from_row!(row, NaiveDateTime, "payment_time", table_name),
        }
    }

    fn from_row_opt(_row: Row) -> Result<Self, FromRowError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

impl FromValue for PaymentMethod {
    type Intermediate = PaymentMethod;
}

impl TryFrom<Value> for PaymentMethod {
    type Error = FromValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match &value {
            Value::Bytes(content) => {
                let content_str = String::from_utf8_lossy(content).to_string();
                PaymentMethod::from_str(&content_str).map_err(|_| FromValueError(value.clone()))
            },
            _ => Err(FromValueError(value))
        }
    }
}

impl std::str::FromStr for PaymentMethod {
    type Err = TheError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Cash" => Ok(PaymentMethod::Cash),
            "Card" => Ok(PaymentMethod::Card),
            "Digital" => Ok(PaymentMethod::Digital),
            _ => Err(create_new_error!("Attempted to read invalid variant")),
        }
    }
}