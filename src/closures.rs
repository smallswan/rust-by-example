// 该函数将闭包作为参数并调用它。
fn apply<F>(f: F)
where
    // 闭包没有输入值和返回值。
    F: FnOnce(),
{
    // ^ 试一试：将 `FnOnce` 换成 `Fn` 或 `FnMut`。

    f();
}

// 输入闭包，返回一个 `i32` 整型的函数。
fn apply_to_3<F>(f: F) -> i32
where
    // 闭包处理一个 `i32` 整型并返回一个 `i32` 整型。
    F: Fn(i32) -> i32,
{
    f(3)
}

use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

/// 闭包：可以捕获环境的匿名函数，http://120.78.128.153/rustbook/ch13-01-closures.html
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

#[test]
fn closure_demo() {
    use std::mem;

    let greeting = "hello";
    // 不可复制的类型。
    // `to_owned` 从借用的数据创建有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获 2 个变量：通过引用捕获 `greeting`，通过值捕获 `farewell`。
    let diary = || {
        // `greeting` 通过引用捕获，故需要闭包是 `Fn`。
        println!("I said {}.", greeting);

        // 下文改变了 `farewell` ，因而要求闭包通过可变引用来捕获它。
        // 现在需要 `FnMut`。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // 手动调用 drop 又要求闭包通过值获取 `farewell`。
        // 现在需要 `FnOnce`。
        mem::drop(farewell);
    };

    // 以闭包作为参数，调用函数 `apply`。
    apply(diary);

    // 闭包 `double` 满足 `apply_to_3` 的 trait 约束。
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));

    generate_workout(30, 20);
}

use itertools::interleave;
use itertools::Itertools;
#[test]
fn iter_adaptors() {
    let rev_v: Vec<i32> = vec![1, 2, 3]
        .into_iter()
        .map(|item| item + 1)
        .rev()
        .collect();
    assert_eq!(rev_v, [4, 3, 2]);

    let mut c = 0;

    for pair in vec!['a', 'b', 'c']
        .into_iter()
        .map(|letter| {
            c += 1;
            (letter, c)
        })
        .rev()
    {
        println!("{:?}", pair);
    }

    let it = (1..3).interleave(vec![-1, -2]);
    itertools::assert_equal(it, vec![1, -1, 2, -2]);

    for elt in interleave(&[1, 2, 3], &[2, 3, 4]) {
        /* loop body */
        println!("{:?}", elt);
    }

    itertools::assert_equal((0..3).intersperse(8), vec![0, 8, 1, 8, 2]);

    // An adaptor that gathers elements in pairs
    let pit = (0..4).batching(|it| match it.next() {
        None => None,
        Some(x) => match it.next() {
            None => None,
            Some(y) => Some((x, y)),
        },
    });

    itertools::assert_equal(pit, vec![(0, 1), (2, 3)]);

    let input = vec![vec![1], vec![2, 3], vec![4, 5, 6]];
    assert_eq!(input.into_iter().concat(), vec![1, 2, 3, 4, 5, 6]);

    let mut iter = "αβγ".chars().dropping(2);
    itertools::assert_equal(iter, "γ".chars());

    let a = (0..).zip("bc".chars());
    let b = (0..).zip("ad".chars());
    let it = a.merge_by(b, |x, y| x.1 <= y.1);
    itertools::assert_equal(it, vec![(0, 'a'), (0, 'b'), (1, 'c'), (1, 'd')]);
}
