use crate::{MESSAGE_SIZE, HASH_SIZE};
use sha2::{Digest, Sha256};
use rand::prelude::ThreadRng;
use rand::RngCore;

pub fn validate_message(msg: &[u8]) -> bool {
    let hash = &msg[MESSAGE_SIZE-HASH_SIZE..];
    let data = &msg[..MESSAGE_SIZE-HASH_SIZE];

    let mut digest = Sha256::new();
    digest.update(&data);
    let hash_result = digest.finalize();

    &hash_result[..] != hash
}

pub fn build_message(random: &mut ThreadRng) -> Vec<u8> {
    let mut data = [0u8; MESSAGE_SIZE-HASH_SIZE];
    random.fill_bytes(&mut data);

    let mut digest = Sha256::new();
    digest.update(&data);
    let result = digest.finalize();

    let buf: Vec<u8> = data.iter()
        .chain(&result)
        .cloned()
        .collect();
    debug_assert_eq!(buf.len(), MESSAGE_SIZE);
    buf
}
