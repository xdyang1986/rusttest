use std::thread;
use std::time::Duration;
use chrono;
use async_std::task;

use super::http::send_heartbeat_async;

// hearbeat manager function.

pub fn start() {
    
    let send = thread::spawn(|| {
        send_heartbeat();
    });

    let recv = thread::spawn(|| {
        rec_heartbeat();
    });

    send.join().unwrap();
    recv.join().unwrap();
}


fn send_heartbeat(){
    loop {
        println!("{:?} : Send heartbeat message!", chrono::offset::Utc::now());
        task::spawn(send_heartbeat_async("Node1".to_string()));
        thread::sleep(Duration::from_secs(3));
    }
}

fn rec_heartbeat(){
    loop {
        println!("{:?} : Recv heartbeat message!", chrono::offset::Utc::now());
        thread::sleep(Duration::from_secs(2));
    }
}