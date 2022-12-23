pub trait Zero{
    fn zero() ->Self;
}

impl Zero for usize{
    fn zero() -> Self {
        0
    }
}

impl Zero for i32{
    fn zero() -> Self {
        0
    }
}

pub trait One{
    fn zero() ->Self;
}

impl One for usize{
    fn zero() -> Self {
        1
    }
}

impl One for i32{
    fn zero() -> Self {
        1
    }
}