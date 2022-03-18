pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashSet;

impl Solution {
    fn x_or_y_is_1(result: &mut Vec<i32>, other: u32, bound: u32) {
        let mut value = 1;

        while value < bound {
            result.push((value + 1) as _);

            value *= other;
        }
    }

    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        let x = x as u32;
        let y = y as u32;
        let bound = bound as u32;
        let mut result = Vec::new();

        if x == 1 {
            if y == 1 {
                if bound >= 2 {
                    result.push(2);
                }
            } else {
                Self::x_or_y_is_1(&mut result, y, bound);
            }
        } else if y == 1 {
            Self::x_or_y_is_1(&mut result, x, bound);
        } else {
            let mut set = HashSet::new();
            let mut x_powers = 1;

            while x_powers < bound {
                let mut y_powers = 1;

                loop {
                    let sum = x_powers + y_powers;

                    if sum <= bound {
                        set.insert(sum);

                        y_powers *= y;
                    } else {
                        break;
                    }
                }

                x_powers *= x;
            }

            result.extend(set.into_iter().map(|value| value as i32));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        Self::powerful_integers(x, y, bound)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
