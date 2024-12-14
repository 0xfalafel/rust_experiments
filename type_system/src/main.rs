mod units;
use units::percentage::Percentage;
use units::money::{Money, Currency};

fn main() {
    println!("12% of 42€: {}", Money::new(42.0, Currency::Euros) * Percentage::new(12.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_money () {
        let bob = Money::new(1337.0, Currency::Dollars);
        let alice = Money::new(42.0, Currency::Euros);
        assert_eq!(bob + alice, Money{amount: 1381.1, currency: Currency::Dollars});
    }

    #[test]
    fn add_percent() {
        assert_eq!(Percentage::new(15.0) + Percentage::new(22.0), Percentage { value: 37.0});
    }

    #[test]
    fn money_add_percent() {
        assert_eq!(Money::new(42.0, Currency::Euros) + Percentage::new(12.0), Money {amount: 47.04, currency: Currency::Euros});
    }
    
    #[test]
    fn money_percentage_of() {
        assert_eq!(Money::new(42.0, Currency::Euros) * Percentage::new(12.0), Money {amount: 5.04, currency: Currency::Euros});
    }
}
