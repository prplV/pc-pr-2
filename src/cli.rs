use clap::Parser;
use anyhow::{Result, Ok};
use log::{info, error};
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::options::{LibFile, PswdFile, CheckItNow, chunk_array};
use crate::gen::Generator as Worker;
use crate::bruteforce::bf_loop;
use tokio::sync::mpsc;

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
        let mut subjobs: Vec<tokio::task::JoinHandle<()>> = vec![];
        // let lib_file = LibFile::check(PASSWORD_LIB).await;
        if let Some(wr) = LibFile::check(PASSWORD_LIB).await {
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
            info!("subjobs are done in {}.{:03}s, lib was formed", sec_stop, (mil_stop as u64 - sec_stop * 60) );
        } else {
            error!("cannot create pswd lib. returning...");
            return Err(anyhow::anyhow!("cannot create pswd lib. returning..."));
        }
        // let threads = self.threads.unwrap();
        let threads = match self.threads {
            Some(t) => t as u64,
            None => 1_u64,
        };
        let libfile= LibFile::get_lib(PASSWORD_LIB).await.unwrap();
        let file_size: u64 = LibFile::get_lib(PASSWORD_LIB).await.unwrap().metadata().await.unwrap().len();
        let mut f = libfile.try_clone().await.unwrap();
        let mut buffer_lib = vec![0u8; file_size as usize];
        let _ = f.read_exact(&mut buffer_lib).await;
        info!("buffer was loaded");
        // let arc_buffer: Arc<Vec<u8>> = Arc::new(buffer_lib);
        let mut tevents: Vec<tokio::task::JoinHandle<()>> = vec![];
        let (tx, rx) = mpsc::channel::<[u8; 1]>(1);
        let (sender, receiver) = (Arc::new(tx), Arc::new(rx));
        let timer = tokio::time::Instant::now();
        // verify_threads(&mut(threads as u64), &mut file_size).await;
        let chunks = Arc::new(chunk_array(&buffer_lib, threads as usize).await);

        // delay
        info!("WAITING 3 SECS UNTIL STARTING...");
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        
        // processing
        if self.file.is_some() {
            if let Some(mut rd) = PswdFile::check(self.file.as_ref().unwrap()).await {
                info!("starting work with passwords in file in {} thread(s)", &threads);
                let mut pswds: Vec<u8> = vec![]; 
                let _ = rd.read(&mut pswds);
                println!("{:?}", pswds);
            }
        }
        else {
            let hash_pswd = self.password.clone().unwrap();
            let arc_pswd = Arc::new(hash_pswd);
            let arc_timer = Arc::new(timer);

            info!("starting work with password hash line in {} thread(s)", &threads);
            for i in 0..threads {
                let arc_pswd = arc_pswd.clone();
                let arc_timer = arc_timer.clone();
                let chunks = chunks.clone(); 
                
                let arc_chunk = Arc::new(chunks[i as usize].clone());
                let (sender, receiver) = (sender.clone(), receiver.clone());

                let event = tokio::spawn(async move {
                    let _ = bf_loop(arc_pswd.clone(), 5 as u64, i as u64, sender, receiver, arc_timer, arc_chunk.clone()).await;
                });
                
                tevents.push(event);
            }
            for i in tevents {
                let _ = i.await;
            }
        }
        Ok(())
    }
}