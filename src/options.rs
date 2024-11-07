use tokio::fs::File;
use std::path::Path;
use log::{warn, error};

pub struct LibFile;
pub struct PswdFile;

pub trait CheckItNow {
    async fn check(name: &str) -> Option<File>;
}

impl CheckItNow for LibFile {
    async fn check(name: &str) -> Option<File> {
        // truncating check 
        if Path::new(name).exists() {
            warn!("found local lib file. truncating...");
        }
        // opening file  
        let mut checker = File::create(name).await;
        if let Err(er) = checker {
            error!("cannot work with lib file. {}", er);
            return None;
        }
        Some(checker.unwrap())
    }
}

impl CheckItNow for PswdFile {
    async fn check(name: &str) -> Option<File> {
        let mut checker = File::open(name).await;
        // if let Err(er) = checker
        //     .read(true)
        //     .open(name).await {
        //         error!("cannot work with pswd file. {}", er);
        //         return None;
        // }
        if let Err(er) = checker {
            error!("cannot work with pswd file. {}", er);
            return None;
        }
        Some(checker.unwrap())
    }
}