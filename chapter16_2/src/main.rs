// Do not communicate by sharing memory; instead, share memory by communicating
// Rust provides an implementation of channels to send data between threads
// a channel has 2 halves: a transmitter and receiver.
// - transmitter: upstream location that fires a message
// - receiver: downstream recipient of messages

use std::sync::mpsc; // mspc stands for multiple producer, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // move transmitter tx into treah closure so the spawned thread owns tx
    // the spwaned thread needs to own tx so it can send messages through the channel
    // tx.send returns a Result<T, E> type so if the receiver has already been dropped,
    // there send op will return an error which can be handled
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx1, rx1) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx1.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received1 = rx1.recv().unwrap();
    // println!("received1 got: {}", received1);
    // ^^^ commented block won't compile: calling tx1.send with val in closure moves val out of thread
    // send() takes ownership, and then when the value is moved, the receiver rx1 takes ownership

    let tx2 = tx1.clone();
    thread::spawn(move || {
        let vals =  vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx1 {
        println!("Got: {}", received);
    }

    // cloning transmitter tx1 and sending messages from two threads to receiver rx1 offers no
    // delivery order guarantees
    // output:
    // Got: hi
    // Got: hi
    // Got: more
    // Got: from
    // Got: messages
    // Got: the
    // Got: for
    // Got: you
    // Got: thread
}
