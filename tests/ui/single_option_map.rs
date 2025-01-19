#![warn(clippy::single_option_map)]

fn h(arg: Option<u32>) -> Option<u32> {
    arg.map(|x| x * 2)
}

fn j(arg: Option<u64>) -> Option<u64> {
    arg.map(|x| x * 2)
}

fn main() {
    let answer = Some(42u32);
    let h_result = h(answer);

    let answer = Some(42u64);
    let j_result = j(answer);
}
