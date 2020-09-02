extern crate futures;
use futures::executor::block_on;
use std::future::Future;

async fn foo() -> u8 {
    5
}

fn bar() -> impl Future<Output = u8> {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}

// fn baz() -> impl Future<Output = u8>{
// async closures are unstable
// let closure = async move |x : u8| {
//     bar().await + x
// };

// closure(5)
// }

async fn hello_world() {
    println!("hello, world!");
}

#[test]
fn demo() {
    let future = hello_world();

    let res1 = block_on(future);
    println!("---{:?}", res1);
}
