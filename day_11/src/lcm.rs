use std::ops::{Div, Mul, Rem};
use num_traits::Zero;

///generic adaptation of https://www.hackertouch.com/lowest-common-multiple-in-rust.html

pub fn lcm<T>(first: T, second: T) -> T
where T: Mul<Output=T> + Div<Output=T> + PartialOrd + Rem<T,Output=T> +Zero + Copy{
    first * second / gcd(first, second)
}

fn gcd<T>(first: T, second: T) -> T
where T: PartialOrd + Rem<T,Output=T> + Zero + Copy{
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == T::zero() {
            return min;
        }

        max = min;
        min = res;
    }
}