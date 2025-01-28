#![warn(clippy::single_option_map)]

use std::sync::atomic::{AtomicUsize, Ordering};

static ATOM: AtomicUsize = AtomicUsize::new(42);
static MAYBE_ATOMIC: Option<&AtomicUsize> = Some(&ATOM);

fn h(arg: Option<u32>) -> Option<u32> {
    //~^ ERROR: `fn` that only maps over argument
    arg.map(|x| x * 2)
}

fn j(arg: Option<u64>) -> Option<u64> {
    //~^ ERROR: `fn` that only maps over argument
    arg.map(|x| x * 2)
}

// No lint: no `Option` argument argument
fn maps_static_option() -> Option<usize> {
    MAYBE_ATOMIC.map(|a| a.load(Ordering::Relaxed))
}

// No lint: wrapped by another function
fn manipulate(i: i32) -> i32 {
    i + 1
}
// No lint: wraps another function to do the optional thing
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
