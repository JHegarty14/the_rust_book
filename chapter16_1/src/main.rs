use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // calling thread::sleep forces a thread to stop execution briefly, allowing a different thread to run
    // if the main thread exits before a spawned thread completes, the spawned thread will be forced to exit too

    // joining threads forces the main thread to wait until all threads are finished before exiting
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from joined spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the joined main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    // output will still be mixed between joined threads, but main will wait for spawned thread to exit before exiting itself

    let handle2 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from joined spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle2.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the joined main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // main thread will wait for spawned thread to finish before running its for loop, so output is no longer mixed

    // use move closures with threads
    let v = vec![1, 2, 3];

    // let handle3 = thread::spawn(|| {
    //     println!("in handle3 thread: {v}");
    // });
    // ^^^ won't compile: closure attempts to borrow `v`, but Rust cannot know how long the spawned thread will run
    // and if the reference to b will be valid for the lifetime of handle3

    // moving v into the spawned thread's closure, which will prevent v from being dropped by the main thread before the spawned
    // thread is done with it
    let handle3 = thread::spawn(move || {
        println!("in handle3 thread: {:?}", v);
    });
    handle3.join().unwrap();
}
