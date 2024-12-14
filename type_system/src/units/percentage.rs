use std::fmt;
use std::ops::Add;

// Percentage
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Percentage {
    pub value: f64
}

impl Percentage {
    pub fn new(value: f64) -> Percentage {
        Percentage { value: value}
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.value)
    }
}

// Implement operations for Percent

impl Add<Percentage> for Percentage {
    type Output = Percentage;

    fn add(self, rhs: Percentage) -> Self::Output {
        Percentage::new(self.value + rhs.value)
    }
}

impl Add<f64> for Percentage {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        rhs + (rhs * self.value / 100.0)
    }
}

impl Add<Percentage> for f64 {
    type Output = f64;

    fn add(self, rhs: Percentage) -> Self::Output {
        rhs.add(self)
    }
}


impl Into<f64> for Percentage {
    fn into(self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Percentage::new(15.0) + Percentage::new(22.0), Percentage { value: 37.0});
    }
}