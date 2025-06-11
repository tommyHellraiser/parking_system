use error_mapper::TheResult;

mod api;
mod modules;
mod database;

fn main() {
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(error) => std::panic::panic_any(error.to_string())
    };

    if let Err(error) = runtime.block_on(exec()) {
        std::panic::panic_any(error.to_string())
    }
}

async fn exec() -> TheResult<()> {
    database::init_db_pool();

    Ok(())
}
