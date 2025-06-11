use std::sync::OnceLock;

use mysql_async::Pool;

const DB_ADDR: &str = "mysql://root@localhost:3306/parking_system";

pub static POOL: OnceLock<Pool> = OnceLock::new();

pub fn init_db_pool() {
    let pool = mysql_async::Pool::new(DB_ADDR);
    let _ = POOL.get_or_init(|| pool);
}

#[macro_export]
macro_rules! get_from_row {
    ($row: expr, $type: ty, $col_name: literal, $table_name: expr) => {
        match $row.get::<$type, _>($col_name) {
            Some(field) => field,
            None => {
                let msg = format!(
                    "Couldn't get field {} from table {}",
                    $col_name, $table_name
                );
                std::panic::panic_any(msg);
            }
        }
    };
}
