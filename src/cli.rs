use clap::Parser;
use anyhow::{Result, Ok};
use log::{info};
use tokio::join;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::options::{LibFile, PswdFile, CheckItNow};
use crate::gen::Generator as Worker;

const PASSWORD_LIB: &str = "pswd.lib";

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(short, long, default_value=None)]
    threads : Option<isize>,
    #[arg(short, long, conflicts_with = "password", required_unless_present = "password")]
    file : Option<String>,
    #[arg(short, long, conflicts_with = "file", required_unless_present = "file")]
    password : Option<String>,
}


impl Cli {
    pub async fn exec(&self) -> Result<()>{
        // PRELOAD
        if self.file.is_some() {
            let preload = join!(LibFile::check(PASSWORD_LIB), PswdFile::check(self.file.as_ref().unwrap()));
        }
        else {
            let preload = LibFile::check(PASSWORD_LIB).await;
            let mut subjobs: Vec<tokio::task::JoinHandle<()>> = vec![];
            if let Some(wr) = preload {
                let temp = Arc::new(Mutex::new(wr));
                info!("creating password lib. starting 26 subjobs...");
                for i in 0..26 {
                    let writer = temp.clone();
                    let subjob = tokio::spawn(async move {
                        let _ = Worker::gen_and_write(i, writer).await;
                    });
                    subjobs.push(subjob);
                }
                let timer = tokio::time::Instant::now();
                for event in subjobs {
                    let _ = event.await;
                }
                let mil_stop = timer.elapsed().as_millis();
                let sec_stop = timer.elapsed().as_secs();
                info!("subjobs are done in {}.{}s, lib was formed", sec_stop, (mil_stop as u64 - sec_stop * 60) );
            }
        }
        Ok(())
    }
}