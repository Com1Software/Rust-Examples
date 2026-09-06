use std::thread;
use std::time::Duration;

fn main() {
    let messages = vec![
        "Thread 1: starting work",
        "Thread 2: starting work",
        "Thread 3: starting work",
    ];

    let mut handles = Vec::new();

    for msg in messages {
        let handle = thread::spawn(move || {
            println!("{}", msg);
            thread::sleep(Duration::from_millis(500));
            println!("{} — done", msg);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("All threads completed");
}
