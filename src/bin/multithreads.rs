use std::sync::{mpsc, Mutex, Arc};
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    multithread_mutex();
}

fn create_new_thread() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("i = {}", i);
        }
    });
}

fn wait_new_thread() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("i = {}", i);
        }
    });

    handle.join();
}

fn move_closure_with_new_thread() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join();
}

fn create_channel() {
    let (in_ch, out_ch) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        in_ch.send(val).unwrap();
    });

    let received = out_ch.recv().unwrap();
    println!("Got: {}", received);
}

fn sending_multiple_values_in_channels() {
    let (in_ch, out_ch) = mpsc::channel();

    let in_ch2 = mpsc::Sender::clone(&in_ch);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            in_ch2.send(val).unwrap();
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
            in_ch.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in out_ch {
        println!("Got: {}", received);
    }
}

fn mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn multithread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}