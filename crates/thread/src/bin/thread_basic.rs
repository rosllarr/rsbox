use std::thread;
use std::time::Duration;

fn main() {
    // Spawn thread
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // When the Main thread complates, all spawned threads are shut down,
    // wheather or not they have finished running.
}
