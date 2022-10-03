use log::debug;
use log::error;
use log::info;
use log::warn;
fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    debug!("hello world!");
    error!("{}", "hello world!");
    info!("{:?}", "hello world!");
    warn!("{:#?}", "hello world!");
}
