use std::ops::{Add, AddAssign, Mul, SubAssign};
pub mod prelude;
pub mod shapes;

pub trait Approach<F> {
    fn approach(self, to: Self, amount: Self) -> Self;
}

impl<T> Approach<T> for T
where
    T: Add<Output = T> + Mul<T, Output = T> + SubAssign + AddAssign + PartialOrd,
{
    fn approach(mut self, to: Self, amount: Self) -> Self {
        if self < to {
            self += amount;

            if self > to {
                return to;
            }
        } else {
            self -= amount;

            if self < to {
                return to;
            }
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::Approach;

    #[test]
    pub fn approach_float_positive() {
        let result = 15f32.approach(30., 2.5);

        assert_eq!(result, 17.5);
    }

    #[test]
    pub fn approach_float_positive_prevent_further() {
        let result = 25f32.approach(30., 10.);

        assert_eq!(result, 30.);
    }

    #[test]
    pub fn approach_float_negative() {
        let result = 15f32.approach(8., 2.5);

        assert_eq!(result, 12.5);
    }

    #[test]
    pub fn approach_float_negative_prevent_further() {
        let result = 15f32.approach(8., 5.);

        assert_eq!(result, 10.);
    }
}
