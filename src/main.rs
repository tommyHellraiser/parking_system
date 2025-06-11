use error_mapper::TheResult;

use crate::api::init_api;

mod api;
mod database;
mod modules;

fn main() {
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => std::panic::panic_any(error.to_string()),
    };

    if let Err(error) = runtime.block_on(exec()) {
        std::panic::panic_any(error.to_string())
    }
}

async fn exec() -> TheResult<()> {
    database::init_db_pool();

    init_api().await?;

    Ok(())
}
