use crate::{tasks::run_tasks, texts::ObjectSize};
use std::rc::Rc;
use std::sync::Arc;

pub fn arc_single(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Single shared Arc",
        |v| v.clone(),
        Arc::new(String::from(size.get_str())),
        thread_count,
        &size,
        !warmup,
    );
}

pub fn clone_into_arc(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Object cloned into Arc",
        |v| Arc::new(v.clone()),
        String::from(size.get_str()),
        thread_count,
        &size,
        !warmup,
    );
}

pub fn arc_into_arc(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Arc cloned into Arc",
        |v| Arc::new(v.clone()),
        Arc::new(String::from(size.get_str())),
        thread_count,
        &size,
        !warmup,
    );
}

pub fn clone_into_rc(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Object cloned into Rc",
        |v| Rc::new(v.clone()),
        String::from(size.get_str()),
        thread_count,
        &size,
        !warmup,
    );
}

pub fn arc_into_rc(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Arc cloned into RC",
        |v| Rc::new(v.clone()),
        Arc::new(String::from(size.get_str())),
        thread_count,
        &size,
        !warmup,
    );
}

pub fn cloning(size: ObjectSize, warmup: bool, thread_count: usize) {
    run_tasks(
        "Full Cloning",
        |v| v.clone(),
        String::from(size.get_str()),
        thread_count,
        &size,
        !warmup,
    );
}
