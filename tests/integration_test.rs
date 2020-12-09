extern crate rust_by_example;

#[test]
fn sensitive_word() {
    assert_eq!(rust_by_example::add(3, 2), 5);

    let timestamp = rust_by_example::time_gen();
    println!("now timestamp : {:?}", timestamp);
}
