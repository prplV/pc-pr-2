use anyhow::{Result, Ok};
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

pub async fn init_logger() -> Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "PasswordCracker: [{}] - {}",
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .try_init()?;
    Ok(())
}