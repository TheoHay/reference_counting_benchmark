use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    single_arc();
    arc_in_arc();
    clone_in_rc();
    arc_in_rc();
    with_clone();
}

fn single_arc() {
    println!("_____________________________________________");
    println!("TRYING WITH SINGLE ARC SHARED BETWEEN THREADS");
    let value = Arc::new(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia a elit eu efficitur. Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex. Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum. Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet. Phasellus sollicitudin leo at risus."));
    let mut threads = Vec::with_capacity(6);
    let now = std::time::Instant::now();
    for _ in 0..6 {
        let value = value.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..100000 {
                let _c0 = value.clone();
                let _c1 = value.clone();
                let _c2 = value.clone();
                let _c3 = value.clone();
                let _c4 = value.clone();
                let _c5 = value.clone();
                let _c6 = value.clone();
                let _c7 = value.clone();
                let _c8 = value.clone();
                let _c9 = value.clone();
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    println!("{} microseconds", now.elapsed().as_micros());
    println!("_____________________________________________");
}

fn arc_in_arc() {
    println!("_____________________________________________");
    println!("TRYING WITH AN ARC IN A ARC FOR EACH THREADS");
    let value = Arc::new(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia a elit eu efficitur. Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex. Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum. Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet. Phasellus sollicitudin leo at risus."));
    let mut threads = Vec::with_capacity(6);
    let now = std::time::Instant::now();
    for _ in 0..6 {
        let value = Arc::new(value.clone());
        threads.push(thread::spawn(move || {
            for _ in 0..100000 {
                let _c0 = value.clone();
                let _c1 = value.clone();
                let _c2 = value.clone();
                let _c3 = value.clone();
                let _c4 = value.clone();
                let _c5 = value.clone();
                let _c6 = value.clone();
                let _c7 = value.clone();
                let _c8 = value.clone();
                let _c9 = value.clone();
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    println!("{} microseconds", now.elapsed().as_micros());
    println!("_____________________________________________");
}

fn clone_in_rc() {
    println!("_____________________________________________");
    println!("TRYING BY CLONING INTO AN RC FOR EACH THREADS");
    let value = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia a elit eu efficitur. Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex. Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum. Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet. Phasellus sollicitudin leo at risus.");
    let mut threads = Vec::with_capacity(6);
    let now = std::time::Instant::now();
    for _ in 0..6 {
        let value = value.clone();
        threads.push(thread::spawn(move || {
            let value = Rc::new(value);
            for _ in 0..100000 {
                let _c0 = value.clone();
                let _c1 = value.clone();
                let _c2 = value.clone();
                let _c3 = value.clone();
                let _c4 = value.clone();
                let _c5 = value.clone();
                let _c6 = value.clone();
                let _c7 = value.clone();
                let _c8 = value.clone();
                let _c9 = value.clone();
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    println!("{} microseconds", now.elapsed().as_micros());
    println!("_____________________________________________");
}

fn arc_in_rc() {
    println!("_____________________________________________");
    println!("TRYING BY CLONING AN ARC INTO AN RC FOR EACH THREADS");
    let value = Arc::new(String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia a elit eu efficitur. Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex. Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum. Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet. Phasellus sollicitudin leo at risus."));
    let mut threads = Vec::with_capacity(6);
    let now = std::time::Instant::now();
    for _ in 0..6 {
        let value = value.clone();
        threads.push(thread::spawn(move || {
            let value = Rc::new(value);
            for _ in 0..100000 {
                let _c0 = value.clone();
                let _c1 = value.clone();
                let _c2 = value.clone();
                let _c3 = value.clone();
                let _c4 = value.clone();
                let _c5 = value.clone();
                let _c6 = value.clone();
                let _c7 = value.clone();
                let _c8 = value.clone();
                let _c9 = value.clone();
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    println!("{} microseconds", now.elapsed().as_micros());
    println!("_____________________________________________");
}

fn with_clone() {
    println!("_____________________________________________");
    println!("TRYING BY CLONING THE STRING");
    let value = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nullam lacinia a elit eu efficitur. Morbi feugiat, sem et consectetur pharetra, metus orci maximus enim, non tristique elit augue ut ex. Praesent tempus arcu ut lacus molestie, a scelerisque libero bibendum. Curabitur iaculis felis id magna scelerisque, vel ultrices felis laoreet. Phasellus sollicitudin leo at risus.");
    let mut threads = Vec::with_capacity(6);
    let now = std::time::Instant::now();
    for _ in 0..6 {
        let value = value.clone();
        threads.push(thread::spawn(move || {
            for _ in 0..100000 {
                let _c0 = value.clone();
                let _c1 = value.clone();
                let _c2 = value.clone();
                let _c3 = value.clone();
                let _c4 = value.clone();
                let _c5 = value.clone();
                let _c6 = value.clone();
                let _c7 = value.clone();
                let _c8 = value.clone();
                let _c9 = value.clone();
            }
        }));
    }
    for th in threads {
        th.join().unwrap();
    }
    println!("{} microseconds", now.elapsed().as_micros());
    println!("_____________________________________________");
}
