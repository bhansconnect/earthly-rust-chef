use lib_c::right_now;
use log::{info, warn};

pub fn logging_fib(n: i64) -> i64 {
    if n < 0 {
        warn!("Someone is sending bad args to fib");
    }
    info!("input was: {}", n);
    info!("Started at: {}", right_now());
    let out = fib(n);
    info!("Ended at: {}", right_now());
    info!("Output was: {}", out);
    out
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fib() {
        assert_eq!(fib(10), 55);
    }
}
