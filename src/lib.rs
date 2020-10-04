
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use aes_ctr::stream_cipher::StreamCipher;
use aes_ctr::Aes128Ctr;
extern crate hex;

use aes_ctr::stream_cipher::generic_array::{typenum::U16, GenericArray};
use aes_ctr::stream_cipher::{
    NewStreamCipher, SyncStreamCipher, SyncStreamCipherSeek
};
#[wasm_bindgen]
pub fn fib(i: u32) -> u32 {
    match i {
        0 => 0,
        1 => 1,
        _ => fib(i-1) + fib(i-2)
    }
}
#[wasm_bindgen]
pub fn encrypt(data:  &mut [u8],keyStr:String) -> String {
    let key = GenericArray::from_slice(keyStr.as_bytes()
    );
    // 生成随机数
    let u8arraynonce: [u8;16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5];
    let nonce: &GenericArray<u8, U16> = GenericArray::from_slice(&u8arraynonce);
    // 创建cipher实例
    let mut cipher = Aes128Ctr::new(&key, &nonce);
    // 加密
    cipher.apply_keystream( data);
    // bytes转hex
    let encrypt_hex = hex::encode(&data);
    return encrypt_hex
}
#[wasm_bindgen]
pub fn decrypt(encrypt_hex: String,keyStr:String) -> Vec<u8> {
    // hex转bytes
    let mut encrypt_data = hex::decode(&encrypt_hex).expect("Decoding failed");
    let key = GenericArray::from_slice(keyStr.as_bytes()
    );
    let u8arraynonce: [u8;16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5];
    let nonce: &GenericArray<u8, U16> = GenericArray::from_slice(&u8arraynonce);
    let mut cipher = Aes128Ctr::new(&key, &nonce);

    // 解密
    cipher.seek(0);
    cipher.apply_keystream(&mut encrypt_data);
    let b: Vec<u8> = encrypt_data.iter().cloned().collect();
    return b
    // // u8数组转字符串
    // let ori_message = str::from_utf8(&encrypt_data).expect("Found invalid UTF-8");
    // println!("Decrypt: {:?}", &ori_message);
}

#[cfg(test)]
mod tests {
    use crate::{fib, encrypt, decrypt};

    #[test]
    fn fi_test() {
       let fi=fib(10);
        println!("fi:{:?}",fi)
    }
    #[test]
    fn encrypt_test() {
        let mut arr:[u8;6]=[1,2,3,4,5,6];
        let ss=String::from("1234567890123456");
        let enctyptString=encrypt(&mut arr, ss);
        println!("encryString:{:?}",enctyptString)
    }
    #[test]
    fn decrypt_test() {
        let encryptString=String::from("3a327381ea33");
        let ss=String::from("1234567890123456");
        let decryptVec=decrypt(encryptString, ss);
        println!("encryString:{:?}",decryptVec)
    }

}