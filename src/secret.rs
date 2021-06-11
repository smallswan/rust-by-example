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
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        return Ok(false);
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

    let cn = HEXUPPER.encode("功成不必在我".as_bytes());
    println!("{}", cn);

    Ok(())
}

#[test]
fn base64_demo() {
    let hello = b"hello rustaceans";
    let encoded = base64::encode(hello);
    let decoded = base64::decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());
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
            facatorial = facatorial * i;
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
