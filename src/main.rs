// Portions from Programming Rust chapter 19 concurrency

use std::io;
use std::thread::spawn;


fn process(nums: i32) {
    println!("Process: {:?}", nums);
}

fn process_in_parallel() -> io::Result<()> {
    // Fork: Spawn a thread to handle each chunk
    let mut thread_handles = vec![];
    for i in 0..8 {
        thread_handles.push(spawn(move || process(i)));
    }

    // Join: Wait for all threads to finish.
    for handle in thread_handles {
        handle.join().unwrap();
    }

    Ok(())
}


fn main() {
    println!("Hello, world!");
    match process_in_parallel() {
        Ok(_) => { println!("Ok!") }
        Err(e) => { println!("Error! {:?}", e); }
    }
}
