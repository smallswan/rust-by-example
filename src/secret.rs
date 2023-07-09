extern crate chrono;
extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha2::Sha256;
use crypto::sha3::Sha3;
use futures::StreamExt;

// use std::io::prelude::BufRead;
use core::time;
use crypto::bcrypt;
use std::{fs::File, io::BufRead, io::BufReader, str};
//use rustc_hex::{ToHex,FromHex};
//extern crate rustc_serialize;
use rustc_serialize::base64::{ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;

use chrono::prelude::*;
use std::collections::HashMap;
use stopwatch::Stopwatch;

/// 支付宝-支付API， https://opendocs.alipay.com/apis/api_1/alipay.trade.pay
#[derive(new, Debug)]
struct AlipayPayParam {
    app_id: String,
    method: String,
    #[new(value="utf-8".to_string())]
    charset: String,
    sign_type: String,
    #[new(default)]
    pub sign: String,
    timestamp: String,
    #[new(value="1.0".to_string())]
    version: String,
    biz_content: String,
}

pub trait AlipaySign {}

// use std::fs::File;
use std::io::prelude::*;
// use std::io::BufReader;
use base64::encode;

#[cfg(test)]
mod tests {
    use image::EncodableLayout;

    use super::*;
    #[test]
    fn argon2() -> Result<(), argon2::password_hash::errors::Error> {
        use argon2::{
            password_hash::{
                rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
            },
            Argon2,
        };
        use passwords::{analyzer, scorer};

        let pwd = "ARG2on&@!";
        let password = pwd.as_bytes(); // Bad password; don't actually use!
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = PasswordHash::new(&password_hash)?;
        assert!(Argon2::default()
            .verify_password(password, &parsed_hash)
            .is_ok());

        println!("score:{}", scorer::score(&analyzer::analyze(pwd)));
        println!("{}", &password_hash);
        Ok(())
    }

    #[test]
    fn hex() {
        use data_encoding::HEXLOWER;
        let nonce = b"unique nonce";
        println!("{}", HEXLOWER.encode(nonce));
        let plaintext = b"plaintext message";
        println!("{}", HEXLOWER.encode(plaintext));
    }

    /// ECB加密模式不安全，容易受到“分组重放”攻击
    #[test]
    fn aes_ecb() {
        use aes::cipher::generic_array::arr;
        use aes::cipher::{
            generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
        };
        use aes::{Aes128, Aes256};

        let inscription = String::from(
            "我们一定要建设一支海军，这支海军要能保卫我们的海防，有效地防御帝国主义的可能的侵略。",
        )
        .into_bytes();
        println!("{:?}", inscription);
        let inscription_bytes: [u8; 126] = [
            230, 136, 145, 228, 187, 172, 228, 184, 128, 229, 174, 154, 232, 166, 129, 229, 187,
            186, 232, 174, 190, 228, 184, 128, 230, 148, 175, 230, 181, 183, 229, 134, 155, 239,
            188, 140, 232, 191, 153, 230, 148, 175, 230, 181, 183, 229, 134, 155, 232, 166, 129,
            232, 131, 189, 228, 191, 157, 229, 141, 171, 230, 136, 145, 228, 187, 172, 231, 154,
            132, 230, 181, 183, 233, 152, 178, 239, 188, 140, 230, 156, 137, 230, 149, 136, 229,
            156, 176, 233, 152, 178, 229, 190, 161, 229, 184, 157, 229, 155, 189, 228, 184, 187,
            228, 185, 137, 231, 154, 132, 229, 143, 175, 232, 131, 189, 231, 154, 132, 228, 190,
            181, 231, 149, 165, 227, 128, 130,
        ];
        // let key = GenericArray::from([0u8; 16]);
        let key = GenericArray::from_slice(&inscription[0..16]);
        // let mut block = GenericArray::from_slice(&inscription);
        // 明文分组长度为128位（16字节）
        let mut block = GenericArray::from([43u8; 16]);

        // Initialize cipher  AES128 需要密钥128位（即16字节）
        let cipher = Aes128::new(&key);

        let block_copy = block.clone();

        // Encrypt block in-place
        cipher.encrypt_block(&mut block);

        // And decrypt it back
        cipher.decrypt_block(&mut block);
        assert_eq!(block, block_copy);

        // Implementation supports parallel block processing. Number of blocks
        // processed in parallel depends in general on hardware capabilities.
        // This is achieved by instruction-level parallelism (ILP) on a single
        // CPU core, which is differen from multi-threaded parallelism.
        let mut blocks = [block; 16];
        cipher.encrypt_blocks(&mut blocks);

        for block in blocks.iter_mut() {
            cipher.decrypt_block(block);
            assert_eq!(block, &block_copy);
        }

        // `decrypt_blocks` also supports parallel block processing.
        cipher.decrypt_blocks(&mut blocks);

        for block in blocks.iter_mut() {
            cipher.encrypt_block(block);
            assert_eq!(block, &block_copy);
        }

        println!("{}", base64::encode(block));

        let key = GenericArray::from_slice(&inscription[0..32]);
        let cipher = Aes256::new(&key);

        cipher.encrypt_block(&mut block);
        println!("{}", base64::encode(block));
    }

    #[test]
    fn aes_gcm() {
        use aes_gcm::aead::{Aead, NewAead};
        use aes_gcm::{Aes256Gcm, Key, Nonce};
        use data_encoding::HEXLOWER;
        // Or `Aes128Gcm`
        // 256 bits(32 bytes) key
        // openssl rand -hex 32
        // hex!() : converting hexadecimal string literals to a byte array
        let key = Key::from_slice(&hex!(
            "c2c567b1151904db13374ea7aef181a4b8509e331a7d6e952a11781d29ebfe52"
        ));
        let cipher = Aes256Gcm::new(key);

        // 96-bits; unique per message
        let nonce = Nonce::from_slice(b"unique nonce");

        let ciphertext = cipher
            .encrypt(nonce, b"plaintext message".as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        println!("{}", HEXLOWER.encode(&ciphertext));
        println!("{}", encode(&ciphertext));
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");

        //
        match File::open("why-rust.txt") {
            Ok(f) => {
                let mut reader = BufReader::new(f);
                let ciphertext = cipher
                    .encrypt(nonce, reader.fill_buf().unwrap())
                    .expect("encryption failure!");
                println!("{:?}", encode(&ciphertext));
                //1. 将密文保存到文件中
                if let Ok(_) = std::fs::write("why-rust.crypto", &ciphertext) {
                    let mut hasher = Sha3::sha3_256();
                    hasher.input(ciphertext.as_bytes());
                    println!("why-rust.crypto写入成功, sha3_256:{}", hasher.result_str());
                }

                //2. 从文件中读取密文进行解密
                if let Ok(cipher_data) = std::fs::read("why-rust.crypto") {
                    let plaintext = cipher
                        .decrypt(nonce, cipher_data.as_ref())
                        .expect("decryption failure!");
                    println!("{}", String::from_utf8(plaintext).unwrap());
                }
            }
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn chacha20poly1305() {
        use chacha20poly1305::aead::{Aead, NewAead};
        use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
        use data_encoding::HEXLOWER; // Or `XChaCha20Poly1305`
        let key_hex = hex!("98baa9548506c53497bae1b098e85cf26b1359baca7e31ad0c7e93b26e8e79d6");
        let key = Key::from_slice(&key_hex); // 32-bytes
        let cipher = ChaCha20Poly1305::new(key);

        let nonce = Nonce::from_slice(b"unique nonce"); // 12-bytes; unique per message

        let ciphertext = cipher
            .encrypt(nonce, b"plaintext message".as_ref())
            .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

        println!("{}", HEXLOWER.encode(&ciphertext));

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

        assert_eq!(&plaintext, b"plaintext message");
    }
}

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
    bcrypt::bcrypt(cost, &salt.as_bytes(), &password.as_bytes(), &mut output);

    println!("bcrypt:{:?}", output);

    let output2 = output.as_slice();
    println!("bcrypt hex:{:?}", output2.to_hex());
    println!("bcrypt base64:{:?}", output2.to_base64(STANDARD));
    //    rustc_serialize::hex::

    //    let s = match str::from_utf8(output.as_slice()) {
    //        Ok(v) => v,
    //        Err(e) => panic!("Invaild UTF-8 sequence:{}",e),
    //    };
    //
    //    println!("bcrypt result : {}",s);
}

const ALIPAY_SIGN_SECRET_KEY: &str = "abcdefgh";
lazy_static! {
    static ref SUPPORT_SIGN_TYPE: Vec<&'static str> = { vec!["MD5", "SHA256"] };
}

#[test]
fn alipay_sign() {
    use chrono::Utc;
    let mut params_map = HashMap::<String, String>::new();

    params_map.insert("service".to_string(), "api-demo".to_string());
    params_map.insert("partner".to_string(), "2088101568338364".to_string());

    params_map.insert(
        "timestamp".to_string(),
        Utc::now().timestamp_millis().to_string(),
    );

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

    params_str += ALIPAY_SIGN_SECRET_KEY;

    println!("params_str=>{}", params_str);

    let sign_type = SUPPORT_SIGN_TYPE[1];

    let mut sign: String = String::from("");
    if sign_type == "MD5" {
        let mut md = Md5::new();
        md.input_str(&params_str);
        sign = md.result_str();
    } else if sign_type == "SHA256" {
        let mut sha256 = Sha256::new();
        sha256.input_str("hello world");
        sign = sha256.result_str();
    }
    println!("sign=>{}", sign);

    params_map.insert("sign_type".to_string(), sign_type.to_string());
    params_map.insert("sign".to_string(), sign);
    // params_map.insert("sign".to_string(), "".to_string());

    let ok = verify_alipay_sign(params_map);
    println!("verify_alipay_sign : {:?}", ok);

    let mut params_map2 = HashMap::<String, String>::new();
    params_map2.insert("sign_type".to_string(), "SHA512".to_string());
    params_map2.insert(
        "timestamp".to_string(),
        Utc::now().timestamp_millis().to_string(),
    );
    verify_alipay_sign(params_map2).expect_err("Not support SHA512");
}

/// verify sign is valid
pub fn verify_alipay_sign(params_map: HashMap<String, String>) -> Result<bool, &'static str> {
    params_map.get("timestamp").expect("required timestamp");
    let sign_type = params_map.get("sign_type").expect("required sign_type");
    let sign_type_str = String::from(sign_type);

    // sign_type(&String) -> sign_type_str(String) -> &*sign_type_str(&str)
    if !SUPPORT_SIGN_TYPE.contains(&(&*sign_type_str)) {
        return Err("not support this sign type");
    }
    params_map.get("sign").expect("required sign");

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

    params_str += ALIPAY_SIGN_SECRET_KEY;

    let mut sign: String = String::from("");
    if sign_type == "MD5" {
        let mut md = Md5::new();
        md.input_str(&params_str);
        sign = md.result_str();
    } else if sign_type == "SHA256" {
        let mut sha256 = Sha256::new();
        sha256.input_str("hello world");
        sign = sha256.result_str();
    }

    if let Some(param_sign) = params_map.get("sign") {
        if *param_sign == sign {
            Ok(true)
        } else {
            Ok(false)
        }
    } else {
        Ok(false)
    }
}

#[test]
fn rust_analyzer_demo() {
    File::open("no-exists.txt").expect_err("must occured error");

    let demo_file = File::open("why-rust.txt").expect("can't open this file");
    let reader = BufReader::new(demo_file);
    let lines = reader.lines();
    for line in lines.map(|line| line.unwrap()) {
        println!("{}", line);
    }
}

#[test]
fn derive_demo() {
    let biz_content = r#"{"out_trade_no":"20150320010101001"}"#;
    let mut param = AlipayPayParam::new(
        "2014072300007148".to_owned(),
        "alipay.trade.page.pay".to_owned(),
        "utf-8".to_owned(),
        "RSA2".to_owned(),
        "2014-07-24 03:07:50".to_owned(),
        "1.0".to_owned(),
        biz_content.to_string(),
    );
    param.sign = "abcde".to_string();

    println!("{:?}", param);

    let param0 = AlipayPayParam {
        app_id: "2014072300007148".to_owned(),
        method: "alipay.trade.page.pay".to_owned(),
        charset: "utf-8".to_owned(),
        sign_type: "RSA2".to_owned(),
        sign: "abcde".to_string(),
        timestamp: "2014-07-24 03:07:50".to_owned(),
        version: "1.0".to_owned(),
        biz_content: biz_content.to_string(),
    };
    println!("{:?}", param0);
}

/// 使用 PBKDF2 对密码进行加密（salt）和散列（hash）运算
/// https://rust-cookbook.budshome.com/cryptography/encryption.html
use data_encoding::{DecodeError, HEXLOWER, HEXUPPER};
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;
#[test]
fn pbkdf2() {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let n_iter = NonZeroU32::new(100_000).unwrap();
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt);

    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());
}

use percent_encoding::{percent_decode, utf8_percent_encode, AsciiSet, CONTROLS};
use std::str::Utf8Error;
use url::form_urlencoded::{byte_serialize, parse};

/// https://url.spec.whatwg.org/#fragment-percent-encode-set
const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

#[test]
fn percent_encode() -> Result<(), Utf8Error> {
    let input = "confident, productive systems programming";

    let iter = utf8_percent_encode(input, FRAGMENT);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    let cn = utf8_percent_encode("通道转兵，强渡乌江，血战娄山关，四渡赤水，通道会议，黎平会议，猴场会议，遵义会议，扎西会议，苟坝会议，会理会议——伟大的转折", FRAGMENT);

    println!("{}", cn);

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
    Ok(())
}

#[test]
fn hex_encode() -> Result<(), DecodeError> {
    // 编码和解码十六进制
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);
    println!("{}", str::from_utf8(&decoded).unwrap());

    let cn = HEXUPPER.encode("功成不必在我".as_bytes());
    println!("{}", cn);

    Ok(())
}

#[test]
fn base64_demo() {
    let hello = b"hello rustaceans00";
    let encoded = base64::encode(hello);
    let decoded = base64::decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());

    // &[u8;T]/String <= Vec<u8> => Base64 String
    let common_base64_str = base64::encode("中文@123&");
    println!("common base64 string: {}", common_base64_str);
    let common_vecu8 = base64::decode(&common_base64_str).unwrap();
    println!("common string: {}", str::from_utf8(&common_vecu8).unwrap());

    println!("道路安全千万条，安全第一条");
}

use num::bigint::{BigInt, ToBigInt};
#[test]
fn big_int() {
    let big = factorial(100);
    println!("{}! equals {}", 100, big);

    let ten_radix_str = format!("{}", big);
    let iter = ten_radix_str.chars().rev();
    let mut count = 0;
    for ch in iter {
        if ch == '0' {
            count += 1;
        } else {
            break;
        }
    }

    println!("阶乘后的0：{}", count);

    println!("阶乘后的0：{}", trailing_zeroes(100));
    println!("阶乘后的0：{}", trailing_zeroes_v2(100));

    if let Some(count_zeros) = big.trailing_zeros() {
        println!("{}", count_zeros);
    };
}

/// 计算x的阶乘，即x!
fn factorial(x: i32) -> BigInt {
    if let Some(mut facatorial) = 1.to_bigint() {
        for i in 1..(x + 1) {
            facatorial *= i;
        }
        facatorial
    } else {
        panic!("Failed to calculate factorial!");
    }
}

/// 力扣（172. 阶乘后的零） https://leetcode-cn.com/problems/factorial-trailing-zeroes/
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut count_fives = 0;
    let mut steps: Vec<i32> = (5..=n).into_iter().filter(|x| *x % 5 == 0).collect();
    // println!("{:?}",steps);
    for step in steps {
        let mut remaining = step;
        while remaining % 5 == 0 {
            count_fives += 1;
            remaining /= 5;
        }
    }

    count_fives
}

/// 力扣（172. 阶乘后的零）
/// f(n) = n/5^1 + n/5^2 + n/5^3 + n/5^m (n < 5^m)
pub fn trailing_zeroes_v2(n: i32) -> i32 {
    let mut count_fives = 0;
    let mut remaining = n;
    while remaining > 0 {
        remaining /= 5;
        count_fives += remaining;
    }
    count_fives
}

use libsm::sm2::signature::{SigCtx, Signature};
use libsm::sm3::hash::Sm3Hash;
use libsm::sm4::Cipher;
use libsm::sm4::Mode;
#[test]
fn sm() {
    let string = String::from("abc");
    let mut hash = Sm3Hash::new(string.as_bytes());
    let digest: [u8; 32] = hash.get_hash();

    let hex_str = HEXLOWER.encode(&digest);
    println!("{}", hex_str);

    let key: [u8; 16] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54, 0x32,
        0x10,
    ];
    let cipher = Cipher::new(&key, Mode::Cbc);

    let iv = rand_block();
    let poem = String::from("断头今日意如何？创业艰难百战多。此去泉台招旧部 ，旌旗十万斩阎罗。");
    let encrypt_bytes = cipher.encrypt(&poem.as_bytes(), &iv);
    println!("{}", HEXLOWER.encode(&encrypt_bytes));
    let plaintext_bytes = cipher.decrypt(&encrypt_bytes, &iv);
    let poem1 = String::from_utf8(plaintext_bytes.to_vec()).unwrap();
    println!("{}", poem1);

    let poem2 = String::from("南国烽烟正十年，此头须向国门悬。后死诸君多努力，捷报飞来当纸钱。");
    let msg = poem2.as_bytes();

    let mut sw = Stopwatch::new();
    // SM2签名速度快，验签速度慢
    let ctx = SigCtx::new();
    let (pk, sk) = ctx.new_keypair();

    println!("elapsed_ms 0:{:?}", sw.elapsed());

    sw.restart();
    let signature = ctx.sign(msg, &sk, &pk);

    println!("elapsed_ms 1:{:?}", sw.elapsed());

    sw.restart();
    let valid = ctx.verify(msg, &pk, &signature);

    println!("valid:{},elapsed_ms:{:?}", valid, sw.elapsed());
}

// rand 和 ring::rand冲突了
extern crate rand as random;
fn rand_block() -> [u8; 16] {
    use random::prelude::*;
    // let mut rng = OsRng::new().unwrap();
    let mut rng = random::thread_rng();
    let mut block: [u8; 16] = [0; 16];
    rng.fill_bytes(&mut block[..]);

    println!("block:{}", HEXLOWER.encode(&block));
    block
}

#[test]
fn highwayhash() {
    use highway::{HighwayHash, HighwayHasher, Key};

    // Generate 128bit hash
    let key = Key([1, 2, 3, 4]);
    let mut hasher128 = HighwayHasher::new(key);
    hasher128.append(&[255]);
    let res128: [u64; 2] = hasher128.finalize128();
    assert_eq!([0xbb007d2462e77f3c, 0x224508f916b3991f], res128);

    // Generate 256bit hash
    let key = Key([1, 2, 3, 4]);
    let mut hasher256 = HighwayHasher::new(key);
    hasher256.append(&[255]);
    let res256: [u64; 4] = hasher256.finalize256();
    let expected: [u64; 4] = [
        0x7161cadbf7cd70e1,
        0xaac4905de62b2f5e,
        0x7b02b936933faa7,
        0xc8efcfc45b239f8d,
    ];
    assert_eq!(expected, res256);
}

#[test]
fn test_rsa() {
    use rand_core::CryptoRngCore;
    use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

    let mut rng = random::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, &data[..])
        .expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);

    println!("{:?}", enc_data);

    // Decrypt
    let dec_data = priv_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

#[test]
fn test_jwt() {
    let mut header = Header::new(Algorithm::HS512);
    header.kid = Some("blabla".to_owned());
    let iat = Utc::now().timestamp() as usize;
    let exp = iat + 86400;
    // let token = encode(&header, &my_claims, &EncodingKey::from_secret("secret".as_ref()))?;
    let my_claims = Claims {
        sub: "token".to_owned(),
        aud: "GEM".to_owned(),
        exp,
        iat,
        iss: "zln".to_owned(),
        nbf: iat,
    };
    let secret = "纨缟夏裔ChatGPT文心一言";
    let token = jsonwebtoken::encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap();
    println!("{}", token);

    if let Ok(jwt_token) = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        println!("{:?}", jwt_token);
    }
}
