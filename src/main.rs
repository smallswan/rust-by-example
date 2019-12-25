#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

use regex::Regex;
const TO_SEARCH: &'static str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

const VERIFY_CODE: &'static str = "0123456789ABCDEFGHJKLMNPQRTUWXY";

lazy_static!{
       static ref RE: Regex = Regex::new("^([0-9ABCDEFGY]{1})([1239]{1})([0-9ABCDEFGHJKLMNPQRTUWXY]{6})([0-9ABCDEFGHJKLMNPQRTUWXY]{9})([0-90-9ABCDEFGHJKLMNPQRTUWXY])$").unwrap();
}

use std::mem;
use List::*;


enum Wealth{
    Rich,
    Poor,
}

enum Occupation{
    Civilians,
    Solders,
}

enum List{
    Cons(u32,Box<List>),
    Nil,
}

impl List{
    fn new() -> List{
        Nil
    }

    fn prepend(self,elem:u32) -> List{
        Cons(elem,Box::new(self))
    }

    fn len(&self) -> u32{
        match *self {
            Cons(_,ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String{
        match *self {
            Cons(head,ref tail) => {
                format!("{},{}",head,tail.stringify())
            },
            Nil =>{
                format!("Nil")
            },
        }
    }
}

fn main() {
    println!("Hello, world!");

    let languages:[&str;5] = ["C","Java","JavaScript","Python","Rust"];

    println!("I can coding by {:?} computer languages",languages.len());

    println!("array occupies {} bytes",mem::size_of_val(&languages));

    println!("I am studying {}",languages[languages.len() -1]);

    use Wealth::{Rich,Poor};
    use Occupation::*;

    let wealth = Rich;
    let work = Occupation::Solders;

    match wealth {
        Rich => println!("The rich have lots of money."),
        Poor => println!("The poor have no money."),
    }

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length:{}",list.len());
    println!("{}",list.stringify());

    match work {
        Civilians => println!("Civilians work."),
        Solders => println!("Solders fight."),
    }

    println!("Hello, world!");
    let identifier = "91330109MA27WFB10H";
    println!("{} is {}",identifier,is_unified_social_credit_identifier(identifier));

    let identifier = "Q1330109MA27WFB10H";
    println!("{} is {}",identifier,is_unified_social_credit_identifier(identifier));
}

fn is_unified_social_credit_identifier(identifier : &str) -> bool{
    assert!(!identifier.is_empty());
    assert_eq!(18 , identifier.trim().len());
    println!("{}",identifier.trim());

    for c in VERIFY_CODE.chars(){
        println!("{}",c);
    }
    RE.is_match(identifier)
}