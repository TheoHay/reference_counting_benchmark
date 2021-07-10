use crate::texts::ObjectSize;
use num_format::{Locale, ToFormattedString};
use std::{thread, time::Instant};

pub fn run_tasks<T: Send + Clone + 'static, B: Clone + 'static>(
    program_name: &'static str,
    builder: fn(T) -> B,
    shared: T,
    thread_count: usize,
    size: &ObjectSize,
    should_print: bool,
) where
    T: Send + Clone + 'static,
    B: Clone + 'static,
{
    let mut threads = Vec::with_capacity(thread_count);
    let now = Instant::now();

    for _ in 0..thread_count {
        let value = shared.clone();
        threads.push(thread::spawn(move || {
            let value = builder(value);
            for _ in 0..100000 {
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
                let _ = value.clone();
            }
        }));
    }

    for h in threads {
        h.join().expect("Join thread failed");
    }

    if should_print {
        println!(
            "{} with {} took {} μs",
            program_name,
            size.as_str(),
            now.elapsed().as_micros().to_formatted_string(&Locale::en)
        );
    }
}
