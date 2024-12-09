pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn sort_by_index(memory: &[u8], num_length: usize, index: usize, nums: &[u8], buffer: &mut [u8]) {
        let mut counts = [0_u8; 10];

        memory.iter().skip(index).step_by(num_length).for_each(|&c| {
            if let Some(target) = counts.get_mut(usize::from(c) - usize::from(b'0') + 1) {
                *target += 1;
            }
        });

        let mut sum = 0;

        for count in &mut counts[1..] {
            sum += *count;
            *count = sum;
        }

        for &num_index in nums {
            let target_index =
                &mut counts[usize::from(memory[num_length * usize::from(num_index) + index]) - usize::from(b'0')];

            buffer[usize::from(*target_index)] = num_index;

            *target_index += 1;
        }
    }

    pub fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = nums.len();
        let num_length = nums.first().map_or(0, String::len);
        let memory = nums.into_iter().flat_map(String::into_bytes).collect::<Vec<_>>();
        let mut result = vec![0; queries.len()];

        let mut queries = result
            .iter_mut()
            .zip(queries)
            .map(|(target, query)| {
                let [k, trim] = <[_; 2]>::map(query.try_into().ok().unwrap(), |x| x as u8);

                (trim, k, target)
            })
            .collect::<Vec<_>>();

        queries.sort_unstable_by_key(|&(trim, _, _)| trim);

        let mut nums = &mut [0; 100][..n];
        let mut buffer = &mut [0; 100][..n];
        let mut sorted_from = num_length;

        nums.iter_mut().zip(0..).for_each(|(target, i)| *target = i);

        for (trim, k, target) in queries {
            let required_sorted_from = num_length - usize::from(trim);

            (required_sorted_from..sorted_from).rev().for_each(|i| {
                Self::sort_by_index(&memory, num_length, i, nums, buffer);

                mem::swap(&mut nums, &mut buffer);
            });

            sorted_from = required_sorted_from;

            *target = i32::from(nums[usize::from(k) - 1]);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn smallest_trimmed_numbers(nums: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        Self::smallest_trimmed_numbers(nums, queries)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
