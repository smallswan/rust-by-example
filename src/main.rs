#![allow(dead_code)]
#[macro_use]
extern crate lazy_static;

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

use std::collections::HashMap;
/// 力扣第1题 https://leetcode-cn.com/problems/two-sum
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums_map = HashMap::<i32, i32>::new();
    for (idx, num) in nums.into_iter().enumerate() {
        let complement = target - num;

        let j = idx as i32;
        if let Some(idx) = nums_map.get(&complement) {
            return vec![*idx, j];
        }
        nums_map.insert(num, j);
    }
    vec![]
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

/// 力扣第724题 https://leetcode-cn.com/problems/find-pivot-index/
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for num in &nums {
        sum += *num;
    }
    let mut left_sum = 0;
    for (idx, num) in nums.iter().enumerate() {
        if *num + left_sum * 2 == sum {
            return idx as i32;
        }
        left_sum += *num;
    }
    -1
}

/// 力扣第747题 https://leetcode-cn.com/problems/largest-number-at-least-twice-of-others/submissions/
pub fn dominant_index(nums: Vec<i32>) -> i32 {
    let mut idx = 0;
    let mut max = nums[idx];
    // 找出最大值及其下表
    for (i, num) in nums.iter().enumerate() {
        if *num > max {
            max = *num;
            idx = i;
        }
    }
    //println!("max:{},idx:{}",max,idx);
    // 找出第二大的数,最小的值为0
    let mut second_biggest = 0;
    for (i, num) in nums.iter().enumerate() {
        if idx != i && *num > second_biggest {
            second_biggest = *num;
        } else {
            continue;
        }
    }

    //println!("second_biggest:{}",second_biggest);
    if max >= 2 * second_biggest {
        return idx as i32;
    }
    -1
}

pub fn find_diagonal_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    if m == 0 {
        return vec![];
    }
    let n = matrix[0].len();

    let mut result = Vec::<i32>::with_capacity(m * n);

    let mut i = 0;
    let mut j = 0;
    for _ in 0..m * n {
        result.push(matrix[i][j]);
        if (i + j) % 2 == 0 {
            //往右上角移动，即i-,j+
            if j == n - 1 {
                i += 1;
            } else if i == 0 {
                j += 1;
            } else {
                i -= 1;
                j += 1;
            }
        } else {
            //往左下角移动，即i+,j-
            if i == m - 1 {
                j += 1;
            } else if j == 0 {
                i += 1;
            } else {
                i += 1;
                j -= 1;
            }
        }
    }

    result
}

/// 力扣第54题 https://leetcode-cn.com/problems/spiral-matrix/
pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let m = matrix.len();
    if m == 0 {
        return vec![];
    }
    let n = matrix[0].len();

    let mut result = Vec::<i32>::with_capacity(m * n);

    let mut i = 0;
    let mut j = 0;
    let mut x = m - 1; //i的最大值
    let mut y = n - 1; //j的最大值
    let mut s = 0; //i的最小值
    let mut t = 0; //j的最小值
    let mut direct = 0;

    let mut push_times = 1;
    result.push(matrix[0][0]);

    while push_times < m * n {
        match direct % 4 {
            0 => {
                //右
                if j < y {
                    j += 1;
                    result.push(matrix[i][j]);
                    push_times += 1;
                    continue;
                } else {
                    s += 1;
                    direct += 1;
                }
            }
            1 => {
                //下
                if i < x {
                    i += 1;
                    result.push(matrix[i][j]);
                    push_times += 1;
                    continue;
                } else {
                    y -= 1;
                    direct += 1;
                }
            }
            2 => {
                //左
                if j > t {
                    j -= 1;
                    result.push(matrix[i][j]);
                    push_times += 1;
                    continue;
                } else {
                    x -= 1;
                    direct += 1;
                }
            }
            3 => {
                //上
                if i > s {
                    i -= 1;
                    result.push(matrix[i][j]);
                    push_times += 1;
                    continue;
                } else {
                    t += 1;
                    direct += 1;
                }
            }
            _ => {
                println!("不可能发生这种情况");
            }
        }
    }
    result
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

    let nums = vec![2, 7, 2, 11];
    let result = two_sum(nums, 9);

    println!("{:?}", result);

    let total = interest_rate(20000.00, 0.0488, 1.0);

    println!("{}", total);

    println!("{}", 3i32.pow(2));

    println!("{}", geometric_series_sum(1.0, 2.0, 20));

    println!("index:{}", pivot_index(vec![1, 7, 3, 6, 5, 6]));

    println!("{}", dominant_index(vec![2]));

    //vec![[1,2,3],[4,5,6],[7,8,9]]
    let mut matrix = Vec::<Vec<i32>>::new();
    matrix.push(vec![1, 2, 3, 4]);
    matrix.push(vec![5, 6, 7, 8]);
    matrix.push(vec![9, 10, 11, 12]);

    println!("{:?}", matrix);
    //    println!("{:?}",find_diagonal_order(matrix));

    println!("{:?}", spiral_order(matrix));
}
