pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cell::Cell;

impl Solution {
    pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
        let mut counts = [const { Cell::new(0) }; 10];

        for digit in digits {
            *counts[digit as u32 as usize].get_mut() += 1;
        }

        let mut result = Vec::new();

        for candidate in (100_u32..1000).step_by(2) {
            let first_count = &counts[(candidate / 100) as usize];

            if first_count.get() != 0 {
                first_count.set(first_count.get() - 1);

                let second_count = &counts[((candidate / 10) % 10) as usize];

                if second_count.get() != 0 {
                    second_count.set(second_count.get() - 1);

                    let third_count = &counts[(candidate % 10) as usize];

                    if third_count.get() != 0 {
                        result.push(candidate as _);
                    }

                    second_count.set(second_count.get() + 1);
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
