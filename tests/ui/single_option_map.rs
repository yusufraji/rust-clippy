#![warn(clippy::single_option_map)]

use std::sync::atomic::{AtomicUsize, Ordering};

static MAYBE_ATOMIC: Option<AtomicUsize> = Some(AtomicUsize::new(42));

fn h(arg: Option<u32>) -> Option<u32> {
    arg.map(|x| x * 2)
}

fn j(arg: Option<u64>) -> Option<u64> {
    arg.map(|x| x * 2)
}

fn maps_static_option() -> Option<usize> {
    MAYBE_ATOMIC.as_ref().map(|a| a.load(Ordering::Relaxed))
}

fn manipulate(i: i32) -> i32 {
    i + 1
}
fn manipulate_opt(opt_i: Option<i32>) -> Option<i32> {
    opt_i.map(manipulate)
}

fn main() {
    let answer = Some(42u32);
    let h_result = h(answer);

    let answer = Some(42u64);
    let j_result = j(answer);
    maps_static_option();
}
