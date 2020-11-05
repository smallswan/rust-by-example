extern crate chrono;
extern crate image;
use chrono::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail !")]
    fn another() {
        panic!("Make this test fail !");
    }
    #[test]
    fn return_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    #[test]
    fn hof() {
        println!("Find the sum of all the squared odd numbers under 1000");
        let upper = 1000;

        // 命令式（imperative）的写法
        // 声明累加器变量
        let mut acc = 0;
        // 迭代：0，1, 2, ... 到无穷大
        for n in 0.. {
            // 数字的平方
            let n_squared = n * n;

            if n_squared >= upper {
                // 若大于上限则退出循环
                break;
            } else if is_odd(n_squared) {
                // 如果是奇数就计数
                acc += n_squared;
            }
        }
        println!("imperative style: {}", acc);
        // 函数式的写法
        let sum_of_squared_odd_numbers: u32 = (0..)
            .map(|n| n * n) // 所有自然数取平方
            .take_while(|&n| n < upper) // 取小于上限的
            .filter(|&n| is_odd(n)) // 取奇数
            .fold(0, |sum, i| sum + i); // 最后加起来
        println!("functional style: {}", sum_of_squared_odd_numbers);

        assert_eq!(acc, sum_of_squared_odd_numbers);
    }

    // 这个函数仅当目标系统是 Linux 的时候才会编译
    #[cfg(target_os = "linux")]
    fn are_you_on_linux() {
        println!("You are running linux!")
    }

    // 而这个函数仅当目标系统 **不是** Linux 时才会编译
    #[cfg(not(target_os = "linux"))]
    fn are_you_on_linux() {
        println!("You are *not* running linux!")
    }

    #[test]
    fn os_test() {
        are_you_on_linux();

        println!("Are you sure");

        if cfg!(target_os = "linux") {
            println!("Yes. It's definitely linux!");
        } else {
            println!("Yes. It's definitely *not* linux!");
        }
    }

    /// 遍历并修改Vec
    #[test]
    fn for_iterator() {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello",
            }
        }
        println!("names: {:?}", names);
    }

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    /// 逐行读取文件
    #[test]
    fn read_file_lines() {
        match File::open("why-rust.txt") {
            Ok(f) => {
                let reader = BufReader::new(f);
                let lines = reader.lines();
                for line in lines.map(|x| x.unwrap()) {
                    println!("{}", line);
                }
            }
            Err(e) => panic!("can't open this file :{}", e),
        }
    }

    use qrcode::render::svg;
    use qrcode::{EcLevel, QrCode, Version};
    //    use qrcode::render::Pixel;
    //    use image::Luma;
    #[test]
    fn qrcode_image() {
        let code = QrCode::new(b"Hello").unwrap();
        let string = code
            .render::<char>()
            .quiet_zone(false)
            .module_dimensions(2, 1)
            .build();
        println!("{}", string);

        let code = QrCode::with_version(b"01234567", Version::Micro(2), EcLevel::L).unwrap();
        let image = code
            .render()
            .min_dimensions(200, 200)
            .dark_color(svg::Color("#800000"))
            .light_color(svg::Color("#ffff80"))
            .build();
        println!("{}", image);
    }

    use std::ops::Deref;
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &self.0
        }
    }

    /// 通过 Deref trait 将智能指针当作常规引用处理 ,http://120.78.128.153/rustbook/ch15-02-deref.html
    #[test]
    fn demo_box() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y)
    }

    struct CustomSmartPointer {
        data: String,
    }
    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    use std::mem::drop;
    /// 通过 std::mem::drop 提早丢弃值, http://120.78.128.153/rustbook/ch15-03-drop.html
    #[test]
    fn drop_trait_demo() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");

        println!("{:?},{:?}", c.data, d.data);
        drop(c);
    }

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use self::List::{Cons, Nil};
    use std::rc::Rc;

    /// 引用计数智能指针,  http://120.78.128.153/rustbook/ch15-04-rc.html
    #[test]
    fn rc_demo() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after created a ={}", Rc::strong_count(&a));
        //Rc::clone 只会增加引用计数，这并不会花费多少时间。而不像深拷贝花费大量时间
        let _b = Cons(3, Rc::clone(&a));
        println!("count after created b ={}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after created c ={}", Rc::strong_count(&a));
        }

        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_message: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_message: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message));
        }
    }

    /// RefCell<T> 和内部可变性模式, http://120.78.128.153/rustbook/ch15-05-interior-mutability.html
    ///
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
        limit_tracker.set_value(90);
        if let Some(msg) = mock_messenger.sent_message.borrow().get(1) {
            println!("the second msg :{}", msg);
        };
    }

    // use super::ConsList::{Cons2,Nil2};
    #[derive(Debug)]
    enum ConsList {
        Cons2(Rc<RefCell<i32>>, Rc<ConsList>),
        Nil2,
    }

    use self::ConsList::{Cons2, Nil2};
    #[test]
    fn rc_rcfcell() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons2(Rc::clone(&value), Rc::new(Nil2)));
        let b = Cons2(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons2(Rc::new(RefCell::new(10)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    #[test]
    fn rc() {
        let c = RefCell::new("hello".to_owned());
        *c.borrow_mut() = "bonjour".to_owned();
        assert_eq!(&*c.borrow(), "bonjour");

        let mut c = RefCell::new(5);
        *c.get_mut() += 1;
        assert_eq!(c, RefCell::new(6));

        let x_vec = RefCell::new(vec![1, 2, 3, 4, 5]);
        let mut mut_v = x_vec.borrow_mut();
        mut_v.push(6);

        // 运行时借用检查
        // let mut mut_v2 = x_vec.borrow_mut();
        // mut_v2.push(7);
    }

    #[derive(Debug)]
    enum List3 {
        Cons3(i32, RefCell<Rc<List3>>),
        Nil3,
    }

    impl List3 {
        fn tail(&self) -> Option<&RefCell<Rc<List3>>> {
            match self {
                Cons3(_, item) => Some(item),
                Nil3 => None,
            }
        }
    }

    /// 引用循环与内存泄漏 , http://120.78.128.153/rustbook/ch15-06-reference-cycles.html
    use self::List3::{Cons3, Nil3};
    #[test]
    fn reference_cycles() {
        let a = Rc::new(Cons3(5, RefCell::new(Rc::new(Nil3))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        let b = Rc::new(Cons3(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));
    }

    use std::rc::Weak;
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    /// 引用循环与内存泄漏, http://120.78.128.153/rustbook/ch15-06-reference-cycles.html
    #[test]
    fn leaf_node() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

/// 第一行是对函数的简短描述。
///
/// 接下来数行是详细文档。代码块用三个反引号开启，Rust 会隐式地在其中添加
/// `fn main()` 和 `extern crate <cratename>`。比如测试 `doccomments` crate：
///
/// ```
/// let result = rust_by_example::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常可能带有 "Examples"、"Panics" 和 "Failures" 这些部分。
///
/// 下面的函数将两数相除。
///
/// # Examples
///
/// ```
/// let result = rust_by_example::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// 如果第二个参数是 0，函数将会 panic。
///
/// ```rust,should_panic
/// // panics on division by zero
/// rust_by_example::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// 前时间(毫秒)
pub fn time_gen() -> i64 {
    // Local::now().timestamp_millis()方法比java 的System.currentTimeMillis();方法要慢很多
    // Utc::now().timestamp_millis() 速度与 System.currentTimeMillis() 相近
    Utc::now().timestamp_millis()
}
