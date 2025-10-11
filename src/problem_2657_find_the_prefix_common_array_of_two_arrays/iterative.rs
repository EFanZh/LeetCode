pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut states = [false; 50];
        let mut result = a;
        let mut common_count = 0;

        result.iter_mut().zip(b).for_each(|(target, rhs)| {
            for value in [*target, rhs] {
                common_count += i32::from(mem::replace(&mut states[value as usize - 1], true));
            }

            *target = common_count;
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        Self::find_the_prefix_common_array(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
