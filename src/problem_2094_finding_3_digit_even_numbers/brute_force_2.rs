pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        #[allow(clippy::declare_interior_mutable_const)] // TODO: Use inline
        const ZERO_CELL: Cell<u8> = Cell::new(0);

        let mut counts = [ZERO_CELL; 10];

        for digit in digits {
            *counts[digit as u32 as usize].get_mut() += 1;
        }

        let mut result = Vec::new();

        for first in 1..10 {
            let first_count = &counts[first];

            if first_count.get() != 0 {
                let value = first as i32 * 100;

                first_count.set(first_count.get() - 1);

                for second in 0..10 {
                    let second_count = &counts[second];

                    if second_count.get() != 0 {
                        let value = value + second as i32 * 10;

                        second_count.set(second_count.get() - 1);

                        for third in (0..10).step_by(2) {
                            if counts[third].get() != 0 {
                                result.push(value + third as i32);
                            }
                        }

                        second_count.set(second_count.get() + 1);
                    }
                }

                first_count.set(first_count.get() + 1);
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        Self::find_even_numbers(digits)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
