extern crate chrono;
extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha2::Sha256;
use crypto::sha3::Sha3;

use std::str;
//use crypto::bcrypt;
//use rustc_hex::{ToHex,FromHex};
//extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;

use chrono::prelude::*;
use std::collections::HashMap;

#[test]
fn rust_crypt() {
    // create a SHA3-256 object
    let mut hasher = Sha3::sha3_256();

    // write input message
    hasher.input_str("helloworld");
    // read hash digest
    //    let res = hex.from_hex().unwrap();
    let hex = hasher.result_str();

    println!("hex:{}", hex);

    let mut sha256 = Sha256::new();
    sha256.input_str("hello world");
    let hex2 = sha256.result_str();
    println!("hex2:{}", hex2);

    let mut sh = Md5::new();
    sh.input_str("123456");
    let md5_str = sh.result_str();
    println!("md5_str:{}", md5_str);

    //    let tmp = vec![0x55u8, 0x2Au8, 0x55u8, 0x00u8];
    let password = String::from("123456");
    let cost = 5u32;
    let salt = String::from("0123456789ABCDEF");
    let mut output = vec![0x00u8; 24];
    crypto::bcrypt::bcrypt(cost, &salt.as_bytes(), &password.as_bytes(), &mut output);

    println!("{:?}", output);

    let output2 = output.as_slice();
    println!("{:?}", output2.to_hex());
    println!("{:?}", output2.to_base64(STANDARD));
    //    rustc_serialize::hex::

    //    let s = match str::from_utf8(output.as_slice()) {
    //        Ok(v) => v,
    //        Err(e) => panic!("Invaild UTF-8 sequence:{}",e),
    //    };
    //
    //    println!("bcrypt result : {}",s);
}

#[test]
fn alipay_sign() {
    let secret_key = "abcdefgh".to_string();
    use chrono::Utc;
    let mut params_map = HashMap::<String, String>::new();

    params_map.insert("service".to_string(), "api-demo".to_string());
    params_map.insert("partner".to_string(), "2088101568338364".to_string());

    params_map.insert(
        "timestamp".to_string(),
        Utc::now().timestamp_millis().to_string(),
    );

    params_map.insert("sign_type".to_string(), "MD5".to_string());

    params_map.get("timestamp").expect("required timestamp");
    params_map.get("sign_type").expect("required sign_type");

    let mut keys = vec![];
    for key in params_map.keys() {
        if key != "sign" && key != "sign_type" {
            keys.push(key);
        }
    }

    keys.sort();

    let mut params_str = "".to_string();
    for key in keys {
        if let Some(value) = params_map.get(key) {
            params_str = format!("{}{}={}&", params_str, key, value);
        }
    }

    params_str += &secret_key;

    println!("params_str=>{}", params_str);

    let mut md = Md5::new();
    md.input_str(&params_str);
    let sign = md.result_str();

    println!("sign=>{}", sign);
    params_map.insert("sign".to_string(), sign);

    params_map.get("sign").expect("required sign");
}
