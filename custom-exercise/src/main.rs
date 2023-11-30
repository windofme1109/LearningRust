use std::thread;
use std::sync::mpsc;
use std::time::Duration;


/**
 * 多线程之间的通信 - Channel
 */
fn main() {
    let (tx, rx) = mpsc::channel();


    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        // let msg = String::from("hello world");
        // tx.send(msg).unwrap();

        let msg_list = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for msg in msg_list {
            tx1.send(msg);
            thread::sleep(Duration::from_millis(1000));
        }



        // println!("msg is {}", msg);
    });

    thread::spawn(move || {
        // let msg = String::from("hello world");
        // tx.send(msg).unwrap();

        let msg_list = vec![
            String::from("more"),
            String::from("message"),
            String::from("for"),
            String::from("you"),
        ];

        for msg in msg_list {
            tx.send(msg);
            thread::sleep(Duration::from_millis(1000));
        }



        // println!("msg is {}", msg);
    });
    // let res = rx.recv().unwrap();

    // println!("info is: {}", res);
   
    for recv_msg in rx {
        println!("received msg is: {}", recv_msg);
    }
}