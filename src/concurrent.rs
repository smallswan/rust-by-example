use crossbeam;
use std::thread;
#[test]
fn move_ref_to_thread() {
    let duration = std::time::Duration::from_millis(500);
    println!("Main thread");

    let handle = thread::spawn(move || {
        println!("sub thread 1");
        thread::sleep(duration);

        //注：其父线程为主线程，而不是线程1
        let handle2 = thread::spawn(move || {
            println!("sub thread 2");
            thread::sleep(duration);
        });

        handle2.join().unwrap();
        thread::sleep(duration);
    });

    handle.join().unwrap();

    let mut vec = vec![1, 2, 3, 4, 5, 6];
    crossbeam::scope(|scope| {
        for e in &vec {
            scope.spawn(move |_| {
                println!("{:?}", e);
            });
        }
    })
    .expect("A child thread panicked");

    println!("{:?}", vec);
}
