pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;
use std::iter;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
struct Rational(i32, i32);

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1 + rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1 - rhs.0 * self.1, self.1 * rhs.1)
    }
}

impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.1, self.1 * rhs.0)
    }
}

impl Solution {
    fn combinations(lhs: Rational, rhs: Rational) -> impl Iterator<Item = Rational> {
        let mut state = 0_u8;

        iter::from_fn(move || {
            if state == 6 {
                None
            } else {
                state += 1;

                Some(match state {
                    1 => lhs + rhs,
                    2 => lhs - rhs,
                    3 => rhs - lhs,
                    4 => lhs * rhs,
                    5 => lhs / rhs,
                    _ => rhs / lhs,
                })
            }
        })
    }

    fn helper_2([a, b]: [i32; 2]) -> impl Iterator<Item = Rational> {
        Self::combinations(Rational(a, 1), Rational(b, 1))
    }

    fn helper_3([a, b, c]: [i32; 3]) -> impl Iterator<Item = Rational> {
        let mut state = 0_u8;

        let split_iter = iter::from_fn(move || {
            if state == 3 {
                None
            } else {
                state += 1;

                Some(match state {
                    1 => (a, [b, c]),
                    2 => (b, [a, c]),
                    _ => (c, [a, b]),
                })
            }
        });

        split_iter.flat_map(|(left, right)| {
            Self::helper_2(right).flat_map(move |rhs| Self::combinations(Rational(left, 1), rhs))
        })
    }

    fn is_24(value: Rational) -> bool {
        value.1 != 0 && value.0 == value.1 * 24
    }

    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let [a, b, c, d]: [i32; 4] = cards.as_slice().try_into().unwrap();

        for (left, right) in [(a, [b, c, d]), (b, [a, c, d]), (c, [a, b, d]), (d, [a, b, c])] {
            for rhs in Self::helper_3(right) {
                if Self::combinations(Rational(left, 1), rhs).any(Self::is_24) {
                    return true;
                }
            }
        }

        for (left, right) in [([a, b], [c, d]), ([a, c], [b, d]), ([a, d], [b, c]), ([a, d], [b, c])] {
            for lhs in Self::helper_2(left) {
                for rhs in Self::helper_2(right) {
                    if Self::combinations(lhs, rhs).any(Self::is_24) {
                        return true;
                    }
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn judge_point24(cards: Vec<i32>) -> bool {
        Self::judge_point24(cards)
    }
}

#[cfg(test)]
mod tests {
    use super::Rational;

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }

    #[test]
    fn test_clone_rational() {
        let x = Rational(2, 3);

        assert_eq!(Clone::clone(&x).0, 2);
        assert_eq!(Clone::clone(&x).1, 3);
    }
}
