#![allow(dead_code)]
#![allow(unused)]
#[macro_use]
extern crate lazy_static;

mod asyncprogram;
mod closures;
mod concurrent;
mod datetime;
mod dfa;
mod easyrust;
mod random;
mod secret;
mod serializing;
mod smartpointer;
mod snowflake;
mod validators;
pub mod visible;

const TO_SEARCH: &str = "On 2010-03-14, foo happened. On 2014-10-14, bar happened.";

use dfa::MatchType::*;
use std::collections::HashMap;
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
            Nil => "Nil".to_string(),
        }
    }
}

fn interest_rate(capital: f64, rate: f64, period: f64) -> f64 {
    capital * (1.0 + rate).powf(period)
}
/// 等比数列求和
fn geometric_series_sum(a1: f64, q: f64, n: u32) -> f64 {
    match q {
        x if (x - 1.0).abs() == 0.0 => (n as f64) * a1,
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

    let len = String::from("Здравствуйте").len();

    println!("len:{}", len);

    for b in "नमसते".bytes() {
        println!("{}", b);
    }

    let mut hello = "नमसते你好".chars();
    for ch in hello {
        println!("{}", ch);
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("{:?}", scores);

    let set = dfa::find_sensitive_word("信用卡套现成本较大", &MinMatchType);
    println!("{:?}", set);

    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。

    let max = |left: i32, right: i32| -> i32 {
        if left > right {
            left
        } else {
            right
        }
    };

    println!("max : {}", max(20, 50));
    println!("max : {}", max(30, 90));

    let mut count = 0;

    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 调用闭包。
    inc();
    inc();

    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^

    println!("{}", GREETING);

    let mut s = String::from("foo");

    s.push_str("bar");

    assert_eq!("foobar", s);
    println!("{}", s);

    let mut f = counter(2);
    assert_eq!(3, f(1));
    println!("counter : {}", f(2));

    // https://doc.rust-lang.org/std/primitive.fn.html
    // not_bar_ptr is function item
    let not_bar_ptr = bar; //`not_bar_ptr` is zero-sized, uniquely identifying `bar`
    assert_eq!(mem::size_of_val(&not_bar_ptr), 0);

    let bar_ptr: fn(i32) = not_bar_ptr; //force coercion to function pointer
    assert_eq!(mem::size_of_val(&bar_ptr), mem::size_of::<usize>());

    let footgun = &bar; // this is a shared reference to the zero-sized type identifying `bar`
    println!("size of footgun {}", mem::size_of_val(footgun));

    // let closures_vec = vec![contains,max];

    let a = A(1, 2);
    let add = A::sum; //Fn Item
    let add_math = A::math;
    assert_eq!(add(1, 2), A::sum(1, 2));
    assert_eq!(add_math(&a), a.math());

    let rgb = color;
    println!("size1 {:?}", mem::size_of_val(&rgb));

    show(rgb); //函数项隐式转换为函数指针

    //1. 函数项类型可以通过显式指定函数类型转换为一个函数指针类型
    let c: fn(&str) -> RGB = rgb;
    println!("size2 {:?}", mem::size_of_val(&c));

    show(c);

    //  闭包与函数指针互通
    let c1 = |s: &str| (1, 2, 3);
    show(c1);

    visible::outer_mod::inner_mod::crate_visible_fn();

    visible::outer_mod::foo();
}

const fn hello() -> &'static str {
    "Hello world"
}

const GREETING: &str = hello();

fn counter(i: i32) -> impl FnMut(i32) -> i32 {
    move |n| n + i
}

fn bar(x: i32) {}

struct A(i32, i32);

impl A {
    //常规函数
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    // 方法
    fn math(&self) -> i32 {
        Self::sum(self.0, self.1)
    }
}

enum Color {
    R(i16),
    G(i16),
    B(i16),
}

type RGB = (i16, i16, i16);

fn color(c: &str) -> RGB {
    (1, 1, 1)
}

fn show(c: fn(&str) -> RGB) {
    println!("{:?}", c("black"));
}
