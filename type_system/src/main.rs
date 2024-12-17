mod units;
use units::percentage::Percentage;
use units::money::{Money, Currency};

fn main() {
    println!("42€: {} + 12%", Money::new(42.0, Currency::Dollars) + Percentage::new(12.0));
}
