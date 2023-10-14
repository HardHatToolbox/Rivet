// use aes::Aes128;
// use block_modes::{BlockMode, Cbc};
// use block_modes::block_padding::Pkcs7;
// use rand::Rng;
// use std::convert::TryInto;
//
// // Create an alias for convenience
// type Aes128Cbc = Cbc<Aes128, Pkcs7>;
//
// pub struct Encryption {
//     pub taskKey: [u8; 16],
//     pub metadataKey: [u8; 16],
//     pub c2MessageKey: [u8; 16],
//     pub iv: [u8; 16],
// }
//
// impl Encryption {
//     pub fn new() -> Self {
//         let mut rng = rand::thread_rng();
//         let taskKey: [u8; 16] = "{{REPLCE_TASK_ENCRYPTION_KEY}}"[..].try_into().unwrap();
//         let metadataKey: [u8; 16] = "{{REPLCE_METADATA_ENCRYPTION_KEY}}"[..].try_into().unwrap();
//         let c2MessageKey: [u8; 16] = "{{REPLCE_C2_MESSAGE_ENCRYPTION_KEY}}"[..].try_into().unwrap();
//         let iv: [u8; 16] = rng.gen();
//         Self
//         {
//             taskKey,
//             metadataKey,
//             c2MessageKey,
//             iv,
//         }
//     }
//
//     pub fn encrypt(&self, data: &[u8]) -> Vec<u8>
//     {
//         let cipher = Aes128Cbc::new_from_slices(&self.key, &self.iv).unwrap();
//         return cipher.encrypt_vec(data);
//     }
//
//     pub fn decrypt(&self, encrypted: &[u8]) -> Vec<u8>
//     {
//         let cipher = Aes128Cbc::new_var(&self.key, &self.iv).unwrap();
//         return cipher.decrypt_vec(encrypted).unwrap();
//     }
// }