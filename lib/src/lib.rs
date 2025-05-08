#![no_std]

pub fn hello_world() -> &'static str {
    "Hello, World!"
}

pub struct Calc {
    value: f64,
}

impl Calc {
    pub fn new(value: f64) -> Self {
        Calc { value }
    }

    pub fn sum(&self, other: f64) -> f64 {
        self.value + other
    }

    pub fn subtract(&self, other: f64) -> f64 {
        self.value - other
    }

    pub fn multiply(&self, other: f64) -> f64 {
        self.value * other
    }

    pub fn divide(&self, other: f64) -> Option<f64> {
        if other == 0.0 {
            None
        } else {
            Some(self.value / other)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(hello_world(), "Hello, World!");
    }

    #[test]
    fn test_calc_operations() {
        let calc = Calc::new(10.0);
        
        assert_eq!(calc.sum(5.0), 15.0);
        assert_eq!(calc.subtract(3.0), 7.0);
        assert_eq!(calc.multiply(2.0), 20.0);
        assert_eq!(calc.divide(2.0), Some(5.0));
        assert_eq!(calc.divide(0.0), None);
    }
}