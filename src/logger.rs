extern crate dotenvy;

use dotenvy::dotenv;
use fast_log::config::Config;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;

pub fn new_logger() {
    dotenv().ok();
    fast_log::init(Config::new()
        .console()
        .file_split("logs/",
                    LogSize::MB(1),
                    RollingType::All,
                    LogPacker {})
    ).unwrap();
}