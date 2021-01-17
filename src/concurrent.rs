use crossbeam;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn move_ref_to_thread() {
    let duration = Duration::from_millis(500);
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

    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    for i in 0..3 {
        let cloned_v = v.clone();
        thread::spawn(move || {
            cloned_v.lock().unwrap().push(i);

            println!("{} {:?}", i, cloned_v);
        });
    }
    thread::sleep(Duration::from_millis(500));

    println!("main {:?}", v);
}

use rayon::prelude::*;
#[test]
fn rayon_par() {
    let nums = vec![1, 2, 3, 4, 5, 6, 99];
    let result1 = sum_of_squares(&nums[..]);
    println!("{:?}", result1);

    let reuslt2: i32 = nums.iter().fold(0, |sum, i| sum + i * i);
    println!("{:?}", reuslt2);

    assert_eq!(result1, reuslt2);

    let reuslt3: i32 = nums.iter().map(|i| i * i).sum();
    println!("{:?}", reuslt3);
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter().map(|i| i * i).sum()
}

#[test]
fn work_stealing_demo() {
    let mut v = [1, 0, 3, 0, 5, 6];
    let (left, right) = v.split_at_mut(5);
    assert_eq!(left, [1, 0, 3, 0, 5]);
    assert_eq!(right, [6]);

    let mut v = vec![5, 1, 8, 22, 0, 44];
    quick_sort(&mut v);
    assert_eq!(v, vec![0, 1, 5, 8, 22, 44]);
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() > 1 {
        let mid = partition(v);
        println!("mid : {:?}", mid);
        let (lo, hi) = v.split_at_mut(mid);
        rayon::join(|| quick_sort(lo), || quick_sort(hi));
    }
}

// 分区会将分界值左侧所有的元素重新排列到切片的第一部分中
// (分界值被任意选取为切片中的最后一个元素)
// 然后返回分界值的索引
fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            println!("swap(i,j) : {:?},{:?}", i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);

    i
}
