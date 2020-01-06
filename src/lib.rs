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

    use rand::prelude::*;
    use regex::Regex;

    #[test]
    fn regex_demo(){
        let re = Regex::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})").unwrap();
        let before = "2012-03-14, 2013-01-01 and 2014-07-05";
        let after = re.replace_all(before, "$m/$d/$y");
        assert_eq!(after, "03/14/2012, 01/01/2013 and 07/05/2014");

        let re = Regex::new(r"(?i)Δ+").unwrap();
        let mat = re.find("ΔδΔ").unwrap();
        assert_eq!((mat.start(), mat.end()), (0, 6));
    }

    #[test]
    fn rand_demo(){
        if rand::random(){
            println!("char: {}", rand::random::<char>());
        }

        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        println!("{}",y);
        let mut nums: Vec<i32> = (1..100).collect();
        nums.shuffle(&mut rng);

        println!("{:?}",nums)
    }
}
