use std::env;

#[macro_use]
extern crate cached;

cached! {
    FIB;
    fn fibonacci(n_term: usize) -> usize = {
        match n_term {
            0 | 1 => n_term,
            n => fibonacci(n - 1) + fibonacci(n - 2),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let x = args[1].parse::<usize>().unwrap_or_default();
    let res_memo_minus_1 = fibonacci(x - 1);
    let res_memo = fibonacci(x);
    println!();
    println!("fibonacci({}) = {}", x, res_memo);
    println!();
    println!(
        "[golden ratio = {}]",
        (res_memo as f64 / res_memo_minus_1 as f64)
    );
    println!()
}
