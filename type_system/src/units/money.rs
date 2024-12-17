use std::fmt;
use std::ops::Add;

use crate::Percentage;

// Currency Type
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Euros,
    Dollars
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
}

// 10€ + 20€ = 30€
impl Add<Money> for Money {
    type Output = Money;

    fn add(self, rhs: Money) -> Self::Output {
        if self.currency == rhs.currency {
            Money { 
                amount: self.amount + rhs.amount,
                currency: self.currency
            }
        } else {
            todo!() // implement some conversion operation
                    // but let's keep this simple for now
        }
    }
}

// 10€ + 5.0 = 15€
impl Add<f64> for Money {
    type Output = Money;

    fn add(self, rhs: f64) -> Self::Output {
        Money::new(self.amount + rhs, self.currency)
    }
}

// Implement Percentage operations
// 100€ + 15% = 115€
impl Add<Percentage> for Money {
    type Output = Money;

    fn add(self, rhs: Percentage) -> Self::Output {
        Money {
            amount: self.amount + (self.amount * rhs.value / 100.0),
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
    fn add_f64 () { // 100€ + 42€ = 142€
        assert_eq!(Money::new(100.0, Currency::Euros) + 42.0, Money{amount: 142.0, currency: Currency::Euros});
    }

    #[test]
    fn add_percent() { // 100€ + 30% = 130€
        assert_eq!(Money::new(100.0, Currency::Euros) + Percentage::new(30.0), Money {amount: 130.0, currency: Currency::Euros});
    }   
}