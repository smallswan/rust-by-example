#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate crypto;

mod datetime;
mod random;
mod secret;
mod snowflake;
mod validators;

const TO_SEARCH: &'static str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

use std::mem;
use List::*;

enum Wealth {
    Rich,
    Poor,
}

enum Occupation {
    Civilians,
    Solders,
}

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{},{}", head, tail.stringify()),
            Nil => format!("Nil"),
        }
    }
}

fn interest_rate(capital: f64, rate: f64, period: f64) -> f64 {
    let total = capital * (1.0 + rate).powf(period);
    total
}
/// 等比数列求和
fn geometric_series_sum(a1: f64, q: f64, n: u32) -> f64 {
    match q {
        x if x == 1.0 => (n as f64) * a1,
        _ => a1 * (1.0 - q.powf(n as f64)) / (1.0 - q),
    }
}

fn main() {
    println!("Hello, world!");

    let languages: [&str; 5] = ["C", "Java", "JavaScript", "Python", "Rust"];

    println!("I can coding by {:?} computer languages", languages.len());

    println!("array occupies {} bytes", mem::size_of_val(&languages));

    println!("I am studying {}", languages[languages.len() - 1]);

    use Occupation::*;
    use Wealth::{Poor, Rich};

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

    println!("linked list has length:{}", list.len());
    println!("{}", list.stringify());

    match work {
        Civilians => println!("Civilians work."),
        Solders => println!("Solders fight."),
    }
    validators::is_unified_social_credit_identifier("92130827MA0E9FUBOY");
    validators::is_18_id_card("452726");

    let filtered_name = validators::filter_company_name("\\中道集团--中付支付（广州分公司）~!@#$%^&*+=|{}':;',\\\\\\\\[\\\\\\\\].<>/?~！@#￥%……&*+|{}\\[\\]【】‘；：\"”“’。，、？《》\",\"Hong Kong ABC Company(DEF branch)（中文括号）");

    println!("{}", filtered_name);

    let total = interest_rate(20000.00, 0.0488, 1.0);

    println!("{}", total);

    println!("{}", 3i32.pow(2));

    println!("{}", geometric_series_sum(1.0, 2.0, 20));

    //let s = "Löwe 老虎 Léopard";
    let t = String::from("Löwe 老虎 Léopard");
    let t1 = &t[..];
    assert_eq!(t1.find("Léopard"), Some(13));

    datetime::formatting_and_parsing();

    random::rand_demo();
}
