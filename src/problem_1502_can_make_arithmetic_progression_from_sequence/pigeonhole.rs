pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::NonZeroU32;

impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        let mut arr = arr;
        let mut min = arr[0];
        let mut max = min;

        for &num in &arr[1..] {
            if num < min {
                min = num;
            } else if num > max {
                max = num;
            }
        }

        let n = arr.len();

        if let Some(interval) = NonZeroU32::new((max - min) as u32 / (n - 1) as u32) {
            for num in &mut arr {
                let diff = (*num - min) as u32;

                if diff % interval == 0 {
                    *num = (diff / interval) as i32;
                } else {
                    return false;
                }
            }

            for start in 0..n {
                let mut pending = arr[start];

                if pending != start as i32 {
                    // We have an empty slot at `start` and a number `pending` to place.

                    loop {
                        if let Some(occupied) = arr.get_mut(pending as u32 as usize) {
                            if *occupied == pending {
                                return false;
                            }

                            pending = mem::replace(occupied, pending);

                            if pending == start as i32 {
                                break;
                            }
                        } else {
                            return false;
                        }
                    }

                    arr[start] = pending;
                }
            }

            true
        } else {
            min == max
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
        Self::can_make_arithmetic_progression(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
