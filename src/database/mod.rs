use std::sync::OnceLock;

use mysql_async::Pool;

const DB_ADDR: &str = "mysql://root@localhost:3306/parking_system";

pub static POOL: OnceLock<Pool> = OnceLock::new();

pub fn init_db_pool() {
    let pool = mysql_async::Pool::new(DB_ADDR);
    let _ = POOL.get_or_init(|| pool);
}
