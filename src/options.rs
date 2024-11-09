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
        let checker = File::create(name).await;
        if let Err(er) = checker {
            error!("cannot work with lib file. {}", er);
            return None;
        }
        Some(checker.unwrap())
    }
}
impl LibFile {
    pub async fn get_lib(name: &str) -> Option<File> {
        let checker = File::open(name).await;
        if let Err(er) = checker {
            error!("cannot open lib file. {}", er);
            return None;
        }
        Some(checker.unwrap())
    }
}

impl CheckItNow for PswdFile {
    async fn check(name: &str) -> Option<File> {
        let checker = File::open(name).await;
        if let Err(er) = checker {
            error!("cannot work with pswd file. {}", er);
            return None;
        }
        Some(checker.unwrap())
    }
}


pub async fn chunk_array<T>(arr: &[T], num_chunks: usize) -> Vec<Vec<T>> 
where T: Clone {
    let chunk_size = (arr.len() / num_chunks) / 5 * 5;
    let mut chunks: Vec<Vec<T>> = Vec::with_capacity(num_chunks);
    let mut start = 0;

    for _ in 0..num_chunks {
        let end = (start + chunk_size).min(arr.len()); 
        chunks.push(arr[start..end].to_vec());
        start = end;
    }
    chunks
}

// pub async fn verify_threads(threads: &mut u64, len : &mut u64) {
//     println!("{}", threads);
//     let mut num_threads = num_cpus::get() as u64 * 1000;
//     if threads > &mut num_threads {
//         *threads = num_threads;
//     }
//     while *len % *threads != 0 {
//         *threads-=1;
//     }
//     println!("{}", threads);
// }