use anyhow::{Ok, Result};
use rand::Rng;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PROCESSES: usize = 4;

// #[allow(dead_code)] 用于消除未使用的警告
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    // 创建producer线程
    for i in 0..NUM_PROCESSES {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    // 释放tx，否则rx会一直等待，导致consumer线程无法退出
    // 原因是，上面tx.clone()时，会多clone一个tx，所以这里需要释放tx
    drop(tx);

    // 创建consumer线程
    let consumer = thread::spawn(move || {
        let mut count = 0;
        for msg in rx {
            count += 1;
            println!("msg = {:?}", msg);
        }

        println!("consumer exit");
        count
    });

    println!("Hello, world!");

    let secret = consumer
        .join()
        .map_err(|e| anyhow::anyhow!("thread join error: {:?}", e))?;

    println!("secret = {}", secret);
    Ok(())
}
fn producer(index: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::rng().random_range(0..usize::MAX);
        tx.send(Msg::new(index, value))?;
        // thread::sleep(Duration::from_millis(1000));
        let sleep_time = (rand::rng().random_range(0..u8::MAX) as u64) * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        // random exit the loop
        if rand::rng().random_range(0..u8::MAX) % 5 == 0 {
            println!("producer {} exit", index);
            break;
            // return Ok(());
        }
    }
    // break; 后loop结束，返回Ok(())，也可以在这里再执行其他操作
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
