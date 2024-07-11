pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn make_range([start, end]: [i32; 2]) -> u64 {
        ((1 << end) - 1) ^ ((1 << (start - 1)) - 1)
    }

    pub fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        let mut covered = 0_u64;

        for range in ranges {
            covered |= Self::make_range(range.try_into().ok().unwrap());
        }

        let expected = Self::make_range([left, right]);

        covered & expected == expected
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_covered(ranges: Vec<Vec<i32>>, left: i32, right: i32) -> bool {
        Self::is_covered(ranges, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
