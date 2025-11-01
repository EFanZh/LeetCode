pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        const PRIMES: [u64; 2] = [0x_2820_8A20_A08A_28AC, 0x_8002_28A2_0208_8288];

        let mut iter = nums.iter().enumerate().filter_map(|(i, &num)| {
            let num = num as u8;

            PRIMES
                .get(usize::from(num / 64))
                .is_some_and(|&bits| bits & (1 << (num % 64)) != 0)
                .then_some(i)
        });

        let first = iter.next().unwrap();
        let last = iter.next_back().unwrap_or(first);

        (last - first) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        Self::maximum_prime_difference(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
