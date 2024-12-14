mod units;
use units::percentage::Percentage;
use units::money::{Money, Currency};

fn main() {
    println!("12% of 42€: {}", Money::new(42.0, Currency::Euros) * Percentage::new(12.0));
}
