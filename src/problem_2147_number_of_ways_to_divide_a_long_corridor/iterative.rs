pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut iter = corridor.bytes();
        let is_seat = |x| x == b'S';

        if iter.any(is_seat) {
            let mut result = 1;

            while iter.any(is_seat) {
                if let Some(i) = iter.position(is_seat) {
                    result = (result * (i as u64 + 1)) % 1_000_000_007;
                } else {
                    return result as _;
                }
            }
        }

        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn number_of_ways(corridor: String) -> i32 {
        Self::number_of_ways(corridor)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
