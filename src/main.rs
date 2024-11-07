// len  : 5
// alph : 0123456789 
// hash : md5 | sha256

mod cli;
mod logger;
mod options;
mod gen;

use clap::Parser;
use logger::init_logger;
use cli::Cli;
use log::info;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    if let Err(er) = init_logger().await {
        eprintln!("PasswordCracker: [ERROR] - cannot init logger due to {}", er);
        return;
    }
    let _ = Cli::parse().exec().await;
    info!("end of main job");
}
