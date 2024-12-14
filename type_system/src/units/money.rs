use std::fmt;
use std::ops::{Add, Mul};

use crate::Percentage;

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Euros,
    Dollars
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Currency::Euros => '€',
            Currency::Dollars => '$',
        };
        write!(f, "{}", symbol)
    }
}


// Money Type
#[derive(Debug, PartialEq, Clone, Copy)]

pub struct Money {
    pub amount: f64,
    pub currency: Currency
}

impl Money {
    pub fn new(amount: f64, currency: Currency) -> Money {
        Money {amount: amount, currency: currency}
    }

    fn conversion(self, new_currency: Currency) -> Money {
        if self.currency == new_currency {
            return self;
        }

        if self.currency == Currency::Euros && new_currency == Currency::Dollars {
            Money::new(self.amount * 1.05, new_currency)
            
        } else if self.currency == Currency::Dollars && new_currency == Currency::Euros {
            Money::new(self.amount / 1.05, new_currency)
            
        } else {
            unreachable!("Fuck, we need to thing about conversion now :(")
        }
    }
}

impl Add<f64> for Money {
    type Output = Money;

    fn add(self, rhs: f64) -> Self::Output {
        Money::new(self.amount + rhs, self.currency)
    }
}

impl Add<Money> for Money {
    type Output = Money;

    fn add(self, other: Money) -> Self::Output {
        let other = other.conversion(self.currency);
        Money::new(self.amount + other.amount, self.currency)
    }
}

// Implement Percentage operations
impl Add<Percentage> for Money {
    type Output = Money;

    fn add(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount + (self.amount * rhs.value / 100.0),
            currency: self.currency
        }
    }
}

impl Mul<Percentage> for Money {
    type Output = Money;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount * rhs.value / 100.0,
            currency: self.currency
        }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.currency {
            Currency::Dollars => write!(f, "${:.2}", self.amount),
            Currency::Euros => write!(f, "{:.2}€", self.amount)         
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add () {
        let bob = Money::new(1337.0, Currency::Dollars);
        let alice = Money::new(42.0, Currency::Euros);
        assert_eq!(bob + alice, Money{amount: 1381.1, currency: Currency::Dollars});
    }

    #[test]
    fn add_percent() {
        assert_eq!(Money::new(42.0, Currency::Euros) + Percentage::new(12.0), Money {amount: 47.04, currency: Currency::Euros});
    }
    
    #[test]
    fn percentage_of() {
        assert_eq!(Money::new(42.0, Currency::Euros) * Percentage::new(12.0), Money {amount: 5.04, currency: Currency::Euros});
    }
}