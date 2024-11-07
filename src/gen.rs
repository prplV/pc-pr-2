use log::info;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Mutex;
use std::sync::Arc;

pub type Writer = Arc<Mutex<File>>;
const ALPHABET: &[u8; 26] = b"abcdefghijklmnopqrstuvwxyz";

pub struct Generator;
impl Generator {
    pub async fn gen_and_write(consts : usize, writer: Writer) {
        let mut buffer: Vec<u8> = Vec::new();

        for i in 0..26_usize.pow(4) {
            let mut index = i;
            // let mut word = [b'a'; 4];
            buffer.push(ALPHABET[consts as usize]);

            for _ in (0..4).rev() {
                buffer.push(ALPHABET[index % 26]);
                // word[j] = ALPHABET[index % 26];
                index /= 26;
            }
            // let body = format!("{}{}", ALPHABET[consts as usize] as char, String::from_utf8_lossy(&word));
            // let mut writer = writer.lock().await;
            // let _ = writer.write_all(body.as_bytes()).await;
            // let _ = writer.flush().await;
            // buffer.push(ALPHABET[consts as usize]);
            // println!("{:?}", buffer.capacity() / 5);
        }
        let mut writer = writer.lock().await;
        let _ = writer.write_all(&buffer).await;
        let _ = writer.flush().await;
        info!("end of generation in {} subjob. buffer size = {} Kb", consts+1, (buffer.capacity() / 1024));
    }
}