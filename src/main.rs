use std::sync::mpsc::{self, channel};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const TIME: std::time::Duration = Duration::from_millis(500);

fn main() {
    let lock = Arc::new(RwLock::new(0usize));
    {
        let mut w = lock.write().expect("failed writing on lock");
        *w = std::usize::MAX / 100 as usize;
    }
    let mut children = vec![];
    let (tx, rx) = channel::<String>();
    for i in 0..4 {
        let c_lock = lock.clone();
        let builder = thread::Builder::new().name(i.to_string());
        let tx_local = mpsc::Sender::clone(&tx);
        children.push(
            builder
                .spawn(move || {
                    let multiplier = 2 * (i + 1);
                    let lock_local = *c_lock.read().expect("failed reading lock variable");
                    for j in 0..5 {
                        let text: String = format!(
                            "thread n°{} - j={} - lock*{}={}",
                            i,
                            j,
                            multiplier,
                            lock_local * multiplier
                        );
                        tx_local
                            .send(text)
                            .expect("failed sending message through channel");
                        thread::sleep(TIME);
                    }
                    match thread::current().name() {
                        Some(x) => String::from(x),
                        None => format!("{:?}", thread::current().id()),
                    }
                }).expect(&format!("failed spawning thread n°{}", i)),
        );
    }

    let mut status = true;

    while status {
        match rx.recv_timeout(
            TIME.checked_add(Duration::from_millis(50))
                .expect("failed adding delay to TIME"),
        ) {
            Ok(msg) => {
                println!("{}", msg);
            }
            Err(_) => {
                status = false;
                println!("timeout");
            }
        }
    }

    for child in children {
        let name = child.join().expect("failed to join thread");
        println!("joined thread \"{}\"", name);
    }
}
