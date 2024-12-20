mod units;
use units::percentage::Percentage;
use units::restype::ResType;
use units::money::{Money, Currency};


fn main() {
    let a = ResType::Money(Money::new(42.0, Currency::Euros));
    let b = ResType::Percent(Percentage::new(20.0));

    let c = a + b;
    println!("a + b: {}", c);

    println!("12% of 42€: {}", Money::new(42.0, Currency::Euros) * Percentage::new(12.0));

    println!("42€ + 12: {}", Money::new(42.0, Currency::Euros) + 12);
    println!("12 + 11%: {}", ResType::Int(12) + ResType::Percent(Percentage::new(11.0)));

    let x = Percentage::new(13.0) / Percentage::new(0.0);
    println!("{}", x);
}
