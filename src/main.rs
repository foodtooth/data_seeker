mod settings;

use lazy_static::lazy_static;
use settings::Settings;
use std::sync::RwLock;

lazy_static! {
    static ref CONFIGS: RwLock<Settings> =
        RwLock::new(Settings::new().expect("configs should be ready when starting app"));
}

fn main() {
    // 1. start http server (axum)
    // 2. database connection
    let settings = CONFIGS.read().unwrap();
    println!("{:?}", settings.server.data_seeker.host);
    todo!();
}
