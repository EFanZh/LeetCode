pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as u32 as usize;

        let mut nums = nums
            .into_iter()
            .map(|x| x as u16)
            .filter(|&x| x <= (target as u16))
            .collect::<Box<_>>();

        nums.sort_unstable();

        let cache = &mut [i16::MIN; 1001][..=target];

        cache[0] = 0;

        let mut sum = 0;

        nums.iter().for_each(|&num| {
            let num = num as usize;

            (0..=sum.min(target - num)).rev().for_each(|i| {
                let cache_i = cache[i];
                let target = &mut cache[i + num];

                *target = (*target).max(cache_i + 1);
            });

            sum += num;
        });

        i32::from(i16::max(*cache.last().unwrap(), -1))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_of_longest_subsequence(nums: Vec<i32>, target: i32) -> i32 {
        Self::length_of_longest_subsequence(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
