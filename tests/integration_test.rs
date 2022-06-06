extern crate rust_by_example;

#[test]
fn sensitive_word() {
    assert_eq!(rust_by_example::add(3, 2), 5);

    let timestamp = rust_by_example::time_gen();
    println!("now timestamp : {:?}", timestamp);

    //交换x,y的值
    let (mut x, mut y) = (254, 128);
    x ^= y;
    y ^= x;
    x ^= y;
    println!("now x ,y: {:?},{:?}", x, y);
    assert_eq!(254, y);
    assert_eq!(128, x);
}
