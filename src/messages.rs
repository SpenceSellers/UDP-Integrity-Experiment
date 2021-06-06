use sha2::{Digest, Sha256};
use rand::prelude::ThreadRng;
use rand::RngCore;
use std::mem;
use std::convert::TryInto;

pub const MESSAGE_SIZE: usize = 256;
pub const HASH_SIZE: usize = 32;

pub struct DecodedMessage {
    pub sequence_number: u32
}

pub fn validate_message(msg: &[u8]) -> Option<DecodedMessage> {
    let hash = &msg[MESSAGE_SIZE-HASH_SIZE..];
    let payload = &msg[..MESSAGE_SIZE-HASH_SIZE];

    let mut digest = Sha256::new();
    digest.update(&payload);
    let hash_result = digest.finalize();

    if &hash_result[..] == hash {
        return Some(DecodedMessage {
            sequence_number: u32::from_be_bytes(payload[..4].try_into().unwrap())
        });
    }

    return None;
}

pub fn build_message(sequence_number: u32, random: &mut ThreadRng) -> Vec<u8> {
    let mut rand_payload = [0u8; MESSAGE_SIZE-HASH_SIZE-mem::size_of::<u32>()];
    random.fill_bytes(&mut rand_payload);

    let mut payload = [0u8; MESSAGE_SIZE-HASH_SIZE];
    payload[..4].copy_from_slice(&u32::to_be_bytes(sequence_number));
    payload[4..].copy_from_slice(&rand_payload);

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
        let msg = build_message(1057, &mut thread_rng());

        if let Some(decoded) = validate_message(&msg) {
            assert_eq!(decoded.sequence_number, 1057);
        } else {
            panic!("Expected result to be valid");
        }
    }

    #[test]
    fn test_corrupted_message_is_invalid() {
        let mut msg = build_message(1057, &mut thread_rng());
        msg[20] = 42;
        assert!(validate_message(&msg).is_none());
    }
}