// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }

    // 计算总长度的方法
    fn length(&self) -> usize {
        self.first_half.len() + self.second_half.len()
    }
}

fn send_tx(q: Arc<Queue>, tx: mpsc::Sender<u32>) {
    let q1 = Arc::clone(&q);
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for val in &q1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let q2 = Arc::clone(&q);
    let tx2 = tx.clone();
    let handle2 = thread::spawn(move || {
        for val in &q2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Queue::new());
    let queue_length = queue.length(); 

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length as u32); 
}