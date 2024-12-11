use std::ops::Add;
use std::fmt;
use std::fmt::Display;

#[derive(Debug, Copy, Clone)]
struct Money {
    amount: f64
}

impl Money {
    pub fn new(amount: f64) -> Self{
        Money {
            amount: amount
        }
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} €", self.amount)
    }
}

impl Into<f64> for Money {
    fn into(self: Money) -> f64 {
        self.amount
    }
}


impl<T> Add<T> for Money
where
    T: Into<f64>,
{
    type Output = Money;

    fn add(self: Money, other: T) -> Money {
        let result = self.amount + other.into();
        Money { amount: result}
    }

}

fn arithmetic_operation<T, U, F>(a: T, b: U, operation: F) -> T
where 
    F: Fn(T, U) -> T,
{
    operation(a, b)
}

fn main() {
    // We support adding either f64
    let money = Money::new(42.0);
    let res = money + 1337.0; // add f64
    println!("{}", res);

    //  or i32
    let i: i32 = 22;
    let res = money + i; // add i32 
    println!("{}", res);

    // With our generic function
    let a = Money::new(42.0);
    let b = 1337.0;
    let res = arithmetic_operation(a, b, |x, y| x+y);
    println!("Generic function: {}", res);
}
