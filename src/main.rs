#[macro_use]
extern crate diesel;

extern crate log;

mod error;

mod endpoints;
mod models;
mod domain;
pub mod schema;
mod driver;

fn main() -> Result<(), &'static str> {


    println!("Starting restaurant server");
    std::env::set_var("RUST_BACKTRACE", "1");

    if std::env::var("RUST_LOG").ok().is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    log::info!("Restaurant server log");


    let sys = actix::System::new("restaurant-api");
    if let Err(e) = endpoints::build() {
        log::error!("ERROR: {:?}!", e);
    }
    let _ = sys.run();

    Ok(())
}