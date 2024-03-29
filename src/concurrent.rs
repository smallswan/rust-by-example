extern crate rayon;

use crossbeam::{self, select};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    /// 线程间传递消息导致主线程无法结束
    /// https://course.rs/compiler/pitfalls/main-with-channel-blocked.html
    #[test]
    fn drop_send() {
        use std::thread;

        let (send, recv) = mpsc::channel();
        let num_threads = 3;
        for i in 0..num_threads {
            let thread_send = send.clone();
            thread::spawn(move || {
                thread_send.send(i).unwrap();
                println!("thread {:?} finished", i);
            });
        }

        drop(send);
        for x in recv {
            println!("Got: {}", x);
        }
        println!("finished iterating");
    }
}

#[test]
fn move_ref_to_thread() {
    //1. 标准库
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

    //2. crossbeam库
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

    //3. 标准库Arc,Mutex
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

use rand::prelude::*;
#[test]
fn crossbeam_demo() {
    let mut nums: Vec<i32> = (1..100).collect();
    let mut rng = rand::thread_rng();

    // 将nums 数组打乱顺序（shuffle 洗牌）
    nums.shuffle(&mut rng);

    println!("{:?}", nums);

    let max = find_max(&nums[..]);
    assert_eq!(Some(99), max);
}

// 将数组分成两个部分，并使用新的线程对它们进行处理
fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use walkdir::WalkDir;

// Verify the iso extension
fn is_iso(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if e.to_string_lossy().to_lowercase() == "txt" => true,
        _ => false,
    }
}

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok((context.finish(), filepath))
}

#[test]
#[ignore]
fn iso_digest() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new("C:\\data")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).expect("Could not send data!");
        });
    }

    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}

mod image {
    use error_chain::error_chain;
    use image::{ImageBuffer, Pixel, Rgb};
    use num::complex::Complex;
    use std::sync::mpsc::{channel, RecvError};
    use threadpool::ThreadPool;

    error_chain! {
        foreign_links {
            MpscRecv(RecvError);
            Io(std::io::Error);
            ImageError(image::error::ImageError);
        }
    }

    // Function converting intensity values to RGB
    // Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
    fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
        let wave = wavelength as f32;

        let (r, g, b) = match wavelength {
            380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
            440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
            490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
            510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
            580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
            645..=780 => (1.0, 0.0, 0.0),
            _ => (0.0, 0.0, 0.0),
        };

        let factor = match wavelength {
            380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
            701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
            _ => 1.0,
        };

        let (r, g, b) = (
            normalize(r, factor),
            normalize(g, factor),
            normalize(b, factor),
        );
        Rgb::from_channels(r, g, b, 0)
    }

    // Maps Julia set distance estimation to intensity values
    fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
        let width = width as f32;
        let height = height as f32;

        let mut z = Complex {
            // scale and translate the point to image coordinates
            re: 3.0 * (x as f32 - 0.5 * width) / width,
            im: 2.0 * (y as f32 - 0.5 * height) / height,
        };

        let mut i = 0;
        for t in 0..max_iter {
            if z.norm() >= 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }
        i
    }

    // Normalizes color intensity values within RGB range
    fn normalize(color: f32, factor: f32) -> u8 {
        ((color * factor).powf(0.8) * 255.) as u8
    }

    #[test]
    fn image() -> Result<()> {
        let (width, height) = (1920, 1080);
        // 为指定宽高的输出图片分配内存
        let mut img = ImageBuffer::new(width, height);
        let iterations = 300;

        let c = Complex::new(-0.8, 0.156);

        let pool = ThreadPool::new(num_cpus::get());
        let (tx, rx) = channel();

        for y in 0..height {
            let tx = tx.clone();
            // execute 将每个像素作为单独的作业接收
            pool.execute(move || {
                for x in 0..width {
                    let i = julia(c, x, y, width, height, iterations);
                    let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                    tx.send((x, y, pixel)).expect("Could not send data!");
                }
            });
        }

        for _ in 0..(width * height) {
            let (x, y, pixel) = rx.recv()?;
            // 使用数据来设置像素的颜色
            img.put_pixel(x, y, pixel);
        }

        // 输出图片内容到指定文件中
        let _ = img.save("output.png")?;
        Ok(())
    }
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

/// parallel quicksort
/// https://github.com/nikomatsakis/rayon/blob/22f04aee0e12b31e029ec669299802d6e2f86bf6/src/test.rs#L6-L28
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

enum WorkMsg {
    Work(u8),
    Exit,
}

#[derive(Debug, Eq, PartialEq)]
enum WorkPerformed {
    FromCache,
    New,
}

enum ResultMsg {
    Result(u8, WorkPerformed),
    Exited,
}

struct WorkState {
    ongoing: i16,
    exiting: bool,
}

#[derive(Debug, Eq, PartialEq)]
enum CacheState {
    Ready,
    WorkInProgress,
}

#[derive(Hash, Eq, PartialEq)]
struct CacheKey(u8);

impl WorkState {
    fn init() -> Self {
        WorkState {
            ongoing: 0,
            exiting: false,
        }
    }

    fn set_ongoing(&mut self, count: i16) {
        self.ongoing += count;
    }

    fn set_exiting(&mut self, exit_state: bool) {
        self.exiting = exit_state;
    }

    fn is_exiting(&self) -> bool {
        self.exiting
    }

    fn is_nomore_work(&self) -> bool {
        self.ongoing == 0
    }
}

use crossbeam_channel::unbounded;
use std::collections::HashMap;
use std::sync::Condvar;

/// 无悔并发
///
/// [示例5: 确保从缓存中取共享数据的行为是确定的](file:///C:/repositories/github/geektime-Rust/Codes/source_codes/target/doc/inviting_rust/ch02/s3_thread_safe/fn.understand_channel_and_condvar.html )
#[test]
fn understand_channel_and_condvar() {
    let (work_sender, work_receiver) = unbounded();
    let (result_sender, result_receiver) = unbounded();
    let (pool_result_sender, pool_result_receiver) = unbounded();
    let mut worker_state = WorkState::init();

    let cache_state: Arc<Mutex<HashMap<CacheKey, Arc<(Mutex<CacheState>, Condvar)>>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(2)
        .build()
        .unwrap();

    let cache: Arc<Mutex<HashMap<CacheKey, u8>>> = Arc::new(Mutex::new(HashMap::new()));

    let _ = thread::spawn(move || loop {
        // 使用 corssbeam 提供的 select! 宏 选择一个就绪工作
        select! {
            recv(work_receiver) -> msg => {
                match msg {
                    Ok(WorkMsg::Work(num)) => {
                        let result_sender = result_sender.clone();
                        let pool_result_sender = pool_result_sender.clone();
                        // 使用缓存
                        let cache = cache.clone();
                        let cache_state = cache_state.clone();

                        // 注意，这里正在池上启动一个新的工作单元。
                        worker_state.set_ongoing(1);

                        pool.spawn(move || {
                            let num = {
                                let (cache_state_lock, cvar) = {
                                    //  `cache_state` 临界区开始
                                    let mut state_map = cache_state.lock().unwrap();
                                    &*state_map
                                        .entry(CacheKey(num.clone()))
                                        .or_insert_with(|| {
                                            Arc::new((
                                                Mutex::new(CacheState::Ready),
                                                Condvar::new(),
                                            ))
                                        })
                                        .clone()
                                    //  `cache_state` 临界区结束
                                };

                                //  `state` 临界区开始
                                let mut state = cache_state_lock.lock().unwrap();

                                // 注意：使用while循环来防止条件变量的虚假唤醒
                                while let CacheState::WorkInProgress = *state {
                                    // 阻塞直到状态是 `CacheState::Ready`.
                                    //
                                    // 当唤醒时会自动释放锁
                                    let current_state = cvar
                                        .wait(state)
                                        .unwrap();
                                    state = current_state;
                                }

                                // 循环外可以认为state 已经是 Ready 的了
                                assert_eq!(*state, CacheState::Ready);

                                let (num, result) = {
                                    // 缓存临界区开始
                                    let cache = cache.lock().unwrap();
                                    let key = CacheKey(num);
                                    let result = match cache.get(&key) {
                                        Some(result) => Some(result.clone()),
                                        None => None,
                                    };
                                    (key.0, result)
                                    // 缓存临界区结束
                                };

                                if let Some(result) = result {
                                    // 从缓存中获得一个结果，并将其发送回去，
                                    // 同时带有一个标志，表明是从缓存中获得了它
                                    let _ = result_sender.send(ResultMsg::Result(result, WorkPerformed::FromCache));
                                    let _ = pool_result_sender.send(());

                                    // 不要忘记通知等待线程
                                    cvar.notify_one();
                                    return;
                                } else {
                                    // 如果缓存里没有找到结果，那么切换状态
                                    *state = CacheState::WorkInProgress;
                                    num
                                }
                                // `state` 临界区结束
                            };

                            // 在临界区外做更多「昂贵工作」

                            let _ = result_sender.send(ResultMsg::Result(num.clone(), WorkPerformed::New));

                            {
                                // 缓存临界区开始
                                // 插入工作结果到缓存中
                                let mut cache = cache.lock().unwrap();
                                let key = CacheKey(num.clone());
                                cache.insert(key, num);
                                // 缓存临界区结束
                            }

                            let (lock, cvar) = {
                                let mut state_map = cache_state.lock().unwrap();
                                &*state_map
                                    .get_mut(&CacheKey(num))
                                    .expect("Entry in cache state to have been previously inserted")
                                    .clone()
                            };
                            // 重新进入 `state` 临界区
                            let mut state = lock.lock().unwrap();

                            // 在这里，由于已经提前设置了state，并且任何其他worker都将等待状态切换回ready，可以确定该状态是“in-progress”。
                            assert_eq!(*state, CacheState::WorkInProgress);

                            // 切换状态为 Ready
                            *state = CacheState::Ready;

                            // 通知等待线程
                            cvar.notify_one();

                            let _ = pool_result_sender.send(());
                        });
                    },
                    Ok(WorkMsg::Exit) => {
                        // N注意，这里接收请求并退出
                        // exiting = true;
                        worker_state.set_exiting(true);

                        // 如果没有正则进行的工作则立即退出
                        if worker_state.is_nomore_work() {
                            result_sender.send(ResultMsg::Exited);
                            break;
                        }
                    },
                    _ => panic!("Error receiving a WorkMsg."),
                }
            },
            recv(pool_result_receiver) -> _ => {
                if worker_state.is_nomore_work() {
                    panic!("Received an unexpected pool result.");
                }

                // 注意，一个工作单元已经被完成
                worker_state.set_ongoing(-1);

                // 如果没有正在进行的工作，并且接收到了退出请求，那么就退出
                if worker_state.is_nomore_work() && worker_state.is_exiting() {
                    result_sender.send(ResultMsg::Exited);
                    break;
                }
            },
        }
    });

    let _ = work_sender.send(WorkMsg::Work(0));
    // 发送两个相同的work
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Work(1));
    let _ = work_sender.send(WorkMsg::Exit);

    let mut counter = 0;

    // 当work 是 1 的时候重新计数
    let mut work_one_counter = 0;

    loop {
        match result_receiver.recv() {
            Ok(ResultMsg::Result(num, cached)) => {
                counter += 1;

                if num == 1 {
                    work_one_counter += 1;
                }

                // 现在我们可以断言，当收到 num 为 1 的第二个结果时，它已经来自缓存。
                if num == 1 && work_one_counter == 2 {
                    assert_eq!(cached, WorkPerformed::FromCache);
                }
            }
            Ok(ResultMsg::Exited) => {
                assert_eq!(3, counter);
                break;
            }
            _ => panic!("Error receiving a ResultMsg."),
        }
    }
}
