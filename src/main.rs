use aes_ctr::Aes128Ctr;
extern crate hex;

use aes_ctr::stream_cipher::generic_array::{typenum::U16, GenericArray};
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
};
use std::str;
fn main() {
    let mut message = String::from("hello world!");
    println!("Input: {:?}", &message);

    unsafe {
        // let mut data = [1, 2, 3, 4, 5, 6, 7, 8, 8, 8];
        let mut data = &mut message.as_bytes_mut();

        // 由字符串生成key也可以自己指定
        let key = GenericArray::from_slice(b"very secret key.");

        // 生成随机数
        let u8arraynonce: [u8;16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5];
        let nonce: &GenericArray<u8, U16> = GenericArray::from_slice(&u8arraynonce);
        // 创建cipher实例
        let mut cipher = Aes128Ctr::new(&key, &nonce);
        // 加密
        cipher.apply_keystream(&mut data);
        // bytes转hex
        let encrypt_hex = hex::encode(&data);
        println!("Encrypt: {:?}", &encrypt_hex);
        // hex转bytes
        let mut encrypt_data = hex::decode(&encrypt_hex).expect("Decoding failed");
        // 解密
        cipher.seek(0);
        cipher.apply_keystream(&mut encrypt_data);
        // u8数组转字符串
        let ori_message = str::from_utf8(&encrypt_data).expect("Found invalid UTF-8");
        println!("Decrypt: {:?}", &ori_message);
    }
}
