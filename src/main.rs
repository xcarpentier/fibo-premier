fn fibonacci(n: i32) -> i64 {
    (1.618_f64.powi(n) / 5.0_f64.sqrt()).round() as i64
}

fn isPremier(n: i64) -> bool {
    isPremierFull(n as f64, 3 as f64)
}

fn isPremierFull(n: f64, i: f64) -> bool {
    let next: f64 = i + 1.0;
    if n % 2.0 == 0.0 {
        return false;
    }
    if next >= n.sqrt() {
      true
    } else {
        n % i == 0.0 && isPremierFull(n, next)
    }
}

fn main() {
    let mut res: i64 = 1;
    let mut x = 0;
    while x < 92 {
        res = fibonacci(x);
        println!("fibonacci: {} = {} // premier ? {}", x, res, isPremier(res));
        x+=1
    }
}
