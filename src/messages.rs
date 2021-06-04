use sha2::{Digest, Sha256};
use rand::prelude::ThreadRng;
use rand::RngCore;

pub const MESSAGE_SIZE: usize = 256;
pub const HASH_SIZE: usize = 32;

pub fn validate_message(msg: &[u8]) -> bool {
    let hash = &msg[MESSAGE_SIZE-HASH_SIZE..];
    let payload = &msg[..MESSAGE_SIZE-HASH_SIZE];

    let mut digest = Sha256::new();
    digest.update(&payload);
    let hash_result = digest.finalize();

    &hash_result[..] == hash
}

pub fn build_message(random: &mut ThreadRng) -> Vec<u8> {
    let mut payload = [0u8; MESSAGE_SIZE-HASH_SIZE];
    random.fill_bytes(&mut payload);

    let mut digest = Sha256::new();
    digest.update(&payload);
    let result = digest.finalize();

    let buf: Vec<u8> = payload.iter()
        .chain(&result)
        .cloned()
        .collect();
    debug_assert_eq!(buf.len(), MESSAGE_SIZE);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_generated_message_is_valid() {
        let msg = build_message(&mut thread_rng());
        assert!(validate_message(&msg));
    }

    #[test]
    fn test_corrupted_message_is_invalid() {
        let mut msg = build_message(&mut thread_rng());
        msg[20] = 42;
        assert!(!validate_message(&msg));
    }
}