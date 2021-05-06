use rustc_serialize::base64::{FromBase64, ToBase64, STANDARD};
use rustc_serialize::hex::ToHex;
use serde::{Deserialize, Serialize};
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

#[derive(Deserialize, Debug)]
struct Request {
    // Use the result of a function as the default if "resource" is
    // not included in the input.
    #[serde(default = "default_resource")]
    resource: String,

    // Use the type's implementation of std::default::Default if
    // "timeout" is not included in the input.
    #[serde(default)]
    timeout: Timeout,

    // Use a method from the type as the default if "priority" is not
    // included in the input. This may also be a trait method.
    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

fn default_resource() -> String {
    "/".to_string()
}

/// Timeout in seconds.
#[derive(Deserialize, Debug)]
struct Timeout(u32);
impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

#[derive(Deserialize, Debug)]
enum Priority {
    ExtraHigh,
    High,
    Normal,
    Low,
    ExtraLow,
}
impl Priority {
    fn lowest() -> Self {
        Priority::ExtraLow
    }
}

#[macro_use]
use serde_json::json;

#[test]
fn marco_json_demo() {
    let code = 200;
    let features = vec!["serde", "json"];

    let value = json!({
    "code": code,
    "success": code == 200,
    "payload": {
        features[0]: features[1]
    }});
    println!(
        "{:?},{:?},{:?},{:?}",
        value["code"], value["payload"], value["success"], value["no_exist"]
    );

    let json = r#"
    [
      {
        "resource": "/users"
      },
      {
        "timeout": 5,
        "priority": "High"
      }
    ]
    "#;

    let requests: Vec<Request> = serde_json::from_str(json).unwrap();
    // The first request has resource="/users", timeout=30, priority=ExtraLow
    println!("{:?}", requests[0]);

    // The second request has resource="/", timeout=5, priority=High
    println!("{:?}", requests[1]);
}

use serde_json::Value;
use std::collections::hash_map::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    username: String,
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
struct Pagination {
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Serialize, Deserialize)]
struct Users {
    users: Vec<User>,

    #[serde(flatten)]
    pagination: Pagination,
}

#[test]
fn attr_flatten() {
    let users = r#"
    {
        "limit": 100,
        "offset": 200,
        "total": 1053,
        "users": [
          {"id": "49824073-979f-4814-be10-5ea416ee1c2f", "username": "john_doe", "mascot": "Ferris"}
        ]
    }
    "#;

    let users_json: Users = serde_json::from_str(&users).unwrap();

    println!(
        "{} , {:?}",
        users_json.pagination.limit, users_json.users[0]
    );
}

use serde::{de, Deserializer};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
struct Outer<'a, S, T: 'a + ?Sized> {
    // When deriving the Deserialize impl, Serde would want to generate a bound
    // `S: Deserialize` on the type of this field. But we are going to use the
    // type's `FromStr` impl instead of its `Deserialize` impl by going through
    // `deserialize_from_str`, so we override the automatically generated bound
    // by the one required for `deserialize_from_str`.
    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(bound(deserialize = "S: FromStr, S::Err: Display"))]
    s: S,

    // Here Serde would want to generate a bound `T: Deserialize`. That is a
    // stricter condition than is necessary. In fact, the `main` function below
    // uses T=str which does not implement Deserialize. We override the
    // automatically generated bound by a looser one.
    #[serde(bound(deserialize = "Ptr<'a, T>: Deserialize<'de>"))]
    ptr: Ptr<'a, T>,
}

/// Deserialize a type `S` by deserializing a string, then using the `FromStr`
/// impl of `S` to create the result. The generic type `S` is not required to
/// implement `Deserialize`.
fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}

/// A pointer to `T` which may or may not own the data. When deserializing we
/// always want to produce owned data.
#[derive(Debug)]
enum Ptr<'a, T: 'a + ?Sized> {
    Ref(&'a T),
    Owned(Box<T>),
}

impl<'de, 'a, T: 'a + ?Sized> Deserialize<'de> for Ptr<'a, T>
where
    Box<T>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Ptr::Owned)
    }
}

#[test]
fn generic_type() {
    let j = r#"
        {
            "s": "1234567890",
            "ptr": "owned"
        }
    "#;

    let result: Outer<u64, str> = serde_json::from_str(j).unwrap();

    // result = Outer { s: 1234567890, ptr: Owned("owned") }
    println!("result = {:?}", result);
}

fn untyped_example() -> serde_json::Result<()> {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones":[
            "+44 1234567",
            "+44 2345678"
        ]
    }
    "#;
    let v: Value = serde_json::from_str(data)?;
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> serde_json::Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    let p: Person = serde_json::from_str(data)?;

    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Address {
    street: String,
    city: String,
}

fn print_an_address() -> serde_json::Result<()> {
    // Some data structure.
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };

    // Serialize it to a JSON string.
    let j = serde_json::to_string(&address)?;

    // Print, write to a file, or send to an HTTP server.
    println!("{}", j);

    Ok(())
}

#[test]
fn three_ways_use_json() {
    untyped_example();
    typed_example();
    print_an_address();
}

use toml::de::Error;

#[test]
fn toml_example() -> Result<(), Error> {
    let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
    "#;

    let package_info: Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(
        package_info["package"]["name"].as_str(),
        Some("your_package")
    );

    let package_info: Config = toml::from_str(toml_content)?;
    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}
