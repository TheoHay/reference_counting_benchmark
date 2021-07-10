#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

mod texts;
use texts::*;
mod benchmarks;
pub use benchmarks::*;
mod tasks;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut thread_count: usize = num_cpus::get();
    if args.len() == 2 {
        if let Ok(c) = args[1].parse::<usize>() {
            thread_count = c;
        }
    }
    benchmark_loop(true, thread_count);
    benchmark_loop(false, thread_count);
}

fn benchmark_loop(warmup: bool, thread_count: usize) {
    if !warmup {
        println!();
    }

    arc_single(ObjectSize::Small, warmup, thread_count);
    arc_single(ObjectSize::Medium, warmup, thread_count);
    arc_single(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }

    clone_into_arc(ObjectSize::Small, warmup, thread_count);
    clone_into_arc(ObjectSize::Medium, warmup, thread_count);
    clone_into_arc(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }

    arc_into_arc(ObjectSize::Small, warmup, thread_count);
    arc_into_arc(ObjectSize::Medium, warmup, thread_count);
    arc_into_arc(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }

    arc_into_rc(ObjectSize::Small, warmup, thread_count);
    arc_into_rc(ObjectSize::Medium, warmup, thread_count);
    arc_into_rc(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }

    clone_into_rc(ObjectSize::Small, warmup, thread_count);
    clone_into_rc(ObjectSize::Medium, warmup, thread_count);
    clone_into_rc(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }

    cloning(ObjectSize::Small, warmup, thread_count);
    cloning(ObjectSize::Medium, warmup, thread_count);
    cloning(ObjectSize::Big, warmup, thread_count);
    if !warmup {
        println!();
    }
}
