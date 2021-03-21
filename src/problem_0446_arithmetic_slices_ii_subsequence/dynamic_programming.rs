pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let mut cache = Vec::<HashMap<_, _>>::with_capacity(a.len());
        let mut result = 0;

        for num in &a {
            let mut step_to_count = HashMap::with_capacity(cache.len());

            for (&prev, prev_step_to_count) in a.iter().zip(&cache) {
                if let Some(step) = num.checked_sub(prev) {
                    if let Some(prev_count) = prev_step_to_count.get(&step) {
                        step_to_count
                            .entry(step)
                            .and_modify(|count| *count += prev_count + 1)
                            .or_insert(prev_count + 1);

                        result += prev_count;
                    } else {
                        step_to_count.entry(step).and_modify(|count| *count += 1).or_insert(1);
                    }
                }
            }

            cache.push(step_to_count);
        }

        result
    }
}

impl super::Solution for Solution {
    fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        Self::number_of_arithmetic_slices(a)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
