pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    const MODULUS: u32 = 1_000_000_007;

    pub fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        let mut cache = next_visit;

        cache.pop();

        let mut sum = 0;

        for i in 0..cache.len() {
            let prev = cache[i] as u32 as usize;

            sum += sum + 2 + Self::MODULUS;
            sum -= cache.get(prev.wrapping_sub(1)).copied().unwrap_or(0) as u32;

            sum = sum.checked_sub(Self::MODULUS).unwrap_or(sum);
            sum = sum.checked_sub(Self::MODULUS).unwrap_or(sum);
            sum = sum.checked_sub(Self::MODULUS).unwrap_or(sum);

            cache[i] = sum as _;
        }

        sum as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_day_been_in_all_rooms(next_visit: Vec<i32>) -> i32 {
        Self::first_day_been_in_all_rooms(next_visit)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
