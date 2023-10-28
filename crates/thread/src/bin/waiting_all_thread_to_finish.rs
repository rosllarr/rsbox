use std::thread;
use std::time::Duration;

fn main() {
    // The return type of thread::spawn is JoinHandle.
    // A JoinHandle is an owned value that, when we call `join` method on it, will wait for its
    // thread to finish
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    //handle.join().unwrap();
    // TODO: uncomment above line and comment out line 27, see output

    // Main thread
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Calling `join` on the handle blocks the thread currently running until the thread
    // represented by the handle terminates.
    // Blocking a thred means that thread is prevented from performing work or exiting.
    handle.join().unwrap();
}
