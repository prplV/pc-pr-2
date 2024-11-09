use std::sync::Arc;
use tokio::join;
use anyhow::Ok;
use log::info;
use md5;
use sha2::{Sha256, Digest};
use tokio::sync::mpsc::{Receiver, Sender};

type ChannelSender = Arc<Sender<[u8; 1]>>;
type ChannelReceiver = Arc<Receiver<[u8; 1]>>;
type Timer = Arc<tokio::time::Instant>;

pub async fn bf_loop(target: Arc<String>, step: u64, thread_id: u64, sender: ChannelSender, receiver : ChannelReceiver, timer: Timer, buffer : Arc<Vec<u8>>) -> anyhow::Result<()> {
    let mut position = thread_id;
    loop {
        if position + step >= (buffer.len() as u64) || receiver.len() > 0 {
            break;
        }

        let pswd_bytes = &buffer[position as usize..(position + 5) as usize];
        let pswd = String::from_utf8_lossy(pswd_bytes);

        let result = join!(md5_verify(&pswd , target.as_str()), sha256_verify(&pswd , target.as_str()));
        match result {
            (true, _) => {
                let mil_stop = timer.elapsed().as_millis();
                let sec_stop = timer.elapsed().as_secs();
                info!("found password : {}, hash: MD5, time : {}.{:03}s, thread : {}", pswd, sec_stop, mil_stop, thread_id);
                let _ = sender.send(*b"1").await;
                break;
            },
            (_, true) => {
                let mil_stop = timer.elapsed().as_millis();
                let sec_stop = timer.elapsed().as_secs();
                info!("found password : {}, hash: SHA256, time : {}.{:03}s, thread : {}", pswd, sec_stop, mil_stop, thread_id);
                let _ = sender.send(*b"1").await;
                break;
            },
            _ => {}, 
        }
        position+=step;
        tokio::task::yield_now().await;
    }
    Ok(())
}

pub async fn md5_verify(pswd : &str, target: &str) -> bool {
    let hashed_pswd = md5::compute(pswd);
    // println!("{:x} -- {}", hashed_pswd, target);
    format!("{:x}", hashed_pswd) == target
}

pub async fn sha256_verify(pswd : &str, target: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(pswd);
    let result_hash = hasher.finalize();
    // println!("{:x} -- {}", result_hash, target);
    format!("{:x}", result_hash) == target
}