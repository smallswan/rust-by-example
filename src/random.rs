extern crate rand;
use rand::prelude::*;
use rand::rngs::OsRng;

pub fn rand_demo() {
    //OsRng 使用操作系统提供的真随机数生成器，因此生成的随机数更加安全。
    let mut rng = OsRng;
    let rand_number: u32 = rng.gen();
    println!("rand_number : {}", rand_number);

    let mut data = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut data);
    println!("{:?}", data);

    // We can use random() immediately. It can produce values of many common types:
    let x: u8 = random();
    println!("{}", x);

    if random() {
        // generates a boolean
        println!("Heads!");
    }

    // If we want to be a bit more explicit (and a little more efficient) we can
    // make a handle to the thread-local generator:
    let mut rng = thread_rng();
    if rng.gen() {
        // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y = rng.gen_range(-40.0..1.3e5);
        println!("x is: {}", x);
        println!("y is: {}", y);
        println!("Number from 0 to 9: {}", rng.gen_range(0..10));
    }

    // Sometimes it's useful to use distributions directly:
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);

    // We can also interact with iterators and slices:
    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!(
        "Lets go in this direction: {}",
        arrows_iter.choose(&mut rng).unwrap()
    );
    let mut nums = [1, 2, 3, 4, 5];
    nums.shuffle(&mut rng);
    println!("I shuffled my {:?}", nums);
}

#[cfg(test)]
mod tests {
    use rand::prelude::*;

    #[test]
    fn first_demo() {
        super::rand_demo();

        if rand::random() {
            // generates a boolean
            // Try printing a random unicode code point (probably a bad idea)!
            println!("char: {:?}", rand::random::<char>());
        }

        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen(); // generates a float between 0 and 1

        println!("{}", y);

        let mut nums: Vec<i32> = (1..100).collect();
        // 将nums 数组打乱顺序（shuffle 洗牌）
        nums.shuffle(&mut rng);

        for num in nums {
            println!("{}", num);
        }
    }

    #[test]
    fn rand_demo2() {
        if rand::random() {
            println!("char: {}", rand::random::<char>());
        }

        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen();
        println!("{}", y);
        let mut nums: Vec<i32> = (1..100).collect();
        nums.shuffle(&mut rng);

        println!("{:?}", nums);
    }
}
