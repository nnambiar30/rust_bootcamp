use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..4 {
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 + (i as u64) * 10));
            tx.send(format!("Thread {} has completed", i)).unwrap();
        });
    }
    
    drop(tx);

    for _ in 0..4 {
        println!("Received: {}", rx.recv().unwrap());
    }
}
