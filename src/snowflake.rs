extern crate chrono;
use chrono::prelude::*;

const TWEPOCH: i64 = 1577836800000i64; //2020-01-01 UTC
const WORKER_ID_BITS: i64 = 5i64;
const DATA_CENTER_ID_BITS: i64 = 5i64;
const MAX_WORKER_ID: i64 = 31i64;
const MAX_DATA_CENTER_ID: i64 = 31i64;
const SEQUENCE_MASK: i64 = 4095i64;
const TIMESTAMP_LEFT_SHIFT: i64 = 22i64;
const DATACENTER_ID_SHIFT: i64 = 17i64;
const WORKER_ID_SHIFT: i64 = 12i64;

pub struct SnowflakeIdWorker {
    data_center_id: i64,
    worker_id: i64,
    sequence: i64,
    last_timestamp: i64,
}

impl SnowflakeIdWorker {
    pub fn new(data_center_id: i64, worker_id: i64) -> SnowflakeIdWorker {
        if worker_id > MAX_WORKER_ID || worker_id < 0 {
            panic!(format!(
                "worker Id can't be greater than {} or less than 0",
                worker_id
            ));
        }

        if data_center_id > MAX_DATA_CENTER_ID || data_center_id < 0 {
            panic!(format!(
                "datacenter Id can't be greater than {} or less than 0",
                data_center_id
            ));
        }

        SnowflakeIdWorker {
            data_center_id,
            worker_id,
            sequence: 0i64,
            last_timestamp: -1i64,
        }
    }

    pub fn next_id(&mut self) -> i64 {
        //println!("data_center_id:{},worker_id:{},last_timestamp:{},sequence:{}",self.data_center_id,self.worker_id,self.last_timestamp,self.sequence);
        let mut timestamp = time_gen();
        if time_gen() < self.last_timestamp {
            panic!(format!(
                "Clock moved backwards.  Refusing to generate id for {} milliseconds",
                self.last_timestamp - timestamp
            ));
        }

        if self.last_timestamp == timestamp {
            self.sequence = (self.sequence + 1) & SEQUENCE_MASK;
            if self.sequence == 0 {
                timestamp = til_next_millis(self.last_timestamp);
            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        return ((timestamp - TWEPOCH) << TIMESTAMP_LEFT_SHIFT)
            | (self.data_center_id << DATACENTER_ID_SHIFT)
            | (self.worker_id << WORKER_ID_SHIFT)
            | self.sequence;
    }
}

/// 阻塞到下一个毫秒，直到获得新的时间戳
///
fn til_next_millis(last_timestamp: i64) -> i64 {
    let mut timestamp = time_gen();
    while timestamp <= last_timestamp {
        timestamp = time_gen();
    }

    timestamp
}

/// 前时间(毫秒)
fn time_gen() -> i64 {
    // Local::now().timestamp_millis()方法比java 的System.currentTimeMillis();方法要慢很多
    // Utc::now().timestamp_millis() 速度与 System.currentTimeMillis() 相近
    Utc::now().timestamp_millis()
}

#[test]
fn elapse_test() {
    let times = 1000_000;
    let start_time = Utc::now().timestamp_millis();
    for _ in 0..times {
        time_gen();
    }

    println!(
        "{} times elapsed (ms) :{}",
        times,
        Utc::now().timestamp_millis() - start_time
    );

    let mut id_worker = SnowflakeIdWorker::new(2, 1);

    let start_time = Local::now().timestamp_millis();
    let times = 1000_000;
    for _ in 0..times {
        id_worker.next_id();
        //        let id = id_worker.next_id();
        //        println!("{}", id);
    }
    println!(
        "生成{}次ID,耗时（ms）{}",
        times,
        Local::now().timestamp_millis() - start_time
    );
}
