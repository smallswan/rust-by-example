use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn ser_de_demo() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let bytes = "AVDdddd33433中文".as_bytes();
    println!("hex = {:?}", bytes.to_hex());
    println!("base64 = {:?}", bytes.to_base64(STANDARD));

    //QVZEZGRkZDMzNDMzJXU0RTJEJXU2NTg3
    if let Ok(s) = "QVZEZGRkZDMzNDMzJXU0RTJEJXU2NTg3".from_base64() {
        println!("{:?}", s);

        let str = match str::from_utf8(&s) {
            Ok(v) => v,
            Err(e) => panic!("Invaild UTF-8 sequence:{}", e),
        };
        println!("str : {:?}", str);
    };

    // &str -> &[u8] -> String (base64)
    let demo_str = "abcd1234中国加油".as_bytes().to_base64(STANDARD);
    println!("base64 output: {}", demo_str);

    // String (base64) -> Vec<u8> -> String
    let res = demo_str.from_base64();
    if res.is_ok() {
        let opt_bytes = String::from_utf8(res.unwrap());
        if opt_bytes.is_ok() {
            println!("decoded from base64: {:?}", opt_bytes.unwrap());
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct AdministrativeDivisions {
    code: String,
    name: String,
}
use std::fs::File;
use std::io::BufReader;
#[test]
fn json_file() {
    match File::open("provinces.json") {
        Ok(f) => {
            println!("open provinces.json...");
            let reader = BufReader::new(f);
            let v: Vec<AdministrativeDivisions> = serde_json::from_reader(reader).unwrap();

            for ad in v {
                println!("{:?}={:?}", ad.code, ad.name);
            }
        }
        Err(e) => panic!("can't open this file : {}", e),
    }
}
