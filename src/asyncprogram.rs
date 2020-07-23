extern crate futures;
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

#[test]
fn demo(){
    let future = hello_world();
    block_on(future);
}