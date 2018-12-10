use std::sync::{Arc, Mutex};
use std::thread;

fn is_prime(n: u128) -> bool {
    let mut res: bool = true;
    match n {
        0 | 1 => res = false,
        2 | 3 => res = true,
        _ if n % 2 == 0 => res = false,
        _ => {
            let sqrtn: u128 = ((n as f64).sqrt() as u128) + 1;
            let mut i: u128 = 3;
            while (i <= sqrtn) && res {
                if n % i == 0 {
                    res = false;
                }
                i += 2;
            }
        }
    }
    res
}

pub fn calc_prime(stop: u128, num_thread: usize, array: &mut Vec<u128>) {
    let mut children = vec![];
    let matrix = Arc::new(Mutex::new(Vec::<Vec<u128>>::new()));
    for num in 0..num_thread {
        let builder = thread::Builder::new().name(num.to_string());
        let matrix = matrix.clone();
        children.push(
            builder
                .spawn(move || {
                    let mut array_in = Vec::<u128>::new();
                    let mut var = (num as u128) * stop / (num_thread as u128) + 1;
                    let stop = ((num + 1) as u128) * stop / (num_thread as u128);
                    while var <= stop {
                        if is_prime(var) {
                            array_in.push(var);
                        }
                        var += 1;
                    }
                    matrix.lock().unwrap().push(array_in);
                    match thread::current().name() {
                        Some(x) => String::from(x),
                        None => format!("{:?}", thread::current().id()),
                    }
                })
                .unwrap_or_else(|_| panic!("failed spawning thread n°{}", num)),
        );
    }

    for child in children {
        let name = child.join().expect("failed to join thread");
        println!("joined thread \"{}\"", name);
    }

    for marray in matrix.lock().unwrap().iter() {
        array.extend(marray);
    }
}

pub fn print_vec(array: &[u128]) {
    println!("Results:");
    println!("{:?}", array);
}
