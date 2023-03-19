// Using Mutexes to control access to data
// mutex: mutual exclusion
// a mutex allows only one thread to access some data at any given time. To access data, a thread first must
// ask to acquire access to the mutex's lock. 

// gotchas with mutexes:
// - you must always attempt to acquire the lock before using the data
// - you must alway release the lock when finished with the data to allow other threads to access

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        let mut exnum = m.lock().unwrap(); // if lock were to be held by another thread, this would panic
        // calling Mutex::lock returns a LockResult<MutexGuard<'_, T>>
        *exnum = 6;
        // notice there is no call to Mutex::unlock. LockResult is a smart pointer that implements Deref, so if a MutexGuard
        // goes out of scope, the lock will be dropped automatically
        // this avoids the risk of blocking other threads if we forget to explicitly call unlock
    }

    println!("m = {:?}", m);
    // output: m = Mutex { data: 6, poisoned: false, .. }

    // sharing mutexes across threads
    // Rc<T> is not thread-safe, so we must use Arc<T> (atomic reference counter)
    // Arc<T> has the same API as Rc<T>

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // each thread will, one-by-one, take ownership of the MutexGuard and increment its value
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Risks of using Mutex<T>/Arc<T>
    // similar to how Rust can't prevent reference cycles when two Rc<T> values reference each other, Rust cannot
    // prevent deadlocks when an operation needs to lock two resources and two threads have each acquired one of the locks.
}
