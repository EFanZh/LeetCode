pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::ops::ControlFlow;

impl Solution {
    fn helper(left: usize, right: usize) -> [i32; 2] {
        match left.cmp(&3) {
            Ordering::Less => return if right < 3 { [-1, -1] } else { [2, 3] },
            Ordering::Equal => return if right < 5 { [-1, -1] } else { [3, 5] },
            Ordering::Greater => {}
        }

        let mut result = [-1, -1];

        if right > 2 {
            let mut is_composite = vec![false; (right - 1) / 2].into_boxed_slice(); // `(3..).step_by(2)`.

            (3..=right.isqrt()).step_by(2).for_each(|step| {
                if !is_composite[step / 2 - 1] {
                    let start = step * step;

                    is_composite[start / 2 - 1..]
                        .iter_mut()
                        .step_by(step)
                        .for_each(|x| *x = true);
                }
            });

            let (mut prev, start) = if left < 3 {
                (2, 0)
            } else {
                let from = left / 2 - 1;

                if let Some(i) = is_composite[from..].iter().position(|&is_composite| !is_composite) {
                    let index = from + i;

                    (index as i32 * 2 + 3, index + 1)
                } else {
                    return result;
                }
            };

            let mut min_gap = u32::MAX;

            if let ControlFlow::Break(early_return) =
                (3..)
                    .step_by(2)
                    .zip(&is_composite)
                    .skip(start)
                    .try_for_each(|(x, &is_composite)| {
                        if !is_composite {
                            let gap = (x - prev).cast_unsigned();

                            if gap < min_gap {
                                if gap == 2 {
                                    return ControlFlow::Break([prev, x]);
                                }

                                min_gap = gap;
                                result = [prev, x];
                            }

                            prev = x;
                        }

                        ControlFlow::Continue(())
                    })
            {
                result = early_return;
            }
        }

        result
    }

    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        Vec::from(Self::helper(
            left.cast_unsigned() as usize,
            right.cast_unsigned() as usize,
        ))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        Self::closest_primes(left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
