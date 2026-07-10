pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn first_matching_index(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let half = n / 2;
        let (left, right) = s.split_at(half);

        left.iter()
            .zip(right.iter().rev())
            .position(|(&x, &y)| x == y)
            .map_or_else(|| if n.is_multiple_of(2) { -1 } else { half as _ }, |i| i as _)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn first_matching_index(s: String) -> i32 {
        Self::first_matching_index(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
