pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let (bottom, top) = (bottom as u32, top as u32);
        let mut special = special.into_iter().map(|floor| floor as u32).collect::<Vec<_>>();

        special.sort_unstable();

        let mut prev = bottom - 1;
        let mut result = 0;

        for floor in special {
            result = result.max(floor - prev);
            prev = floor;
        }

        (result - 1).max(top - prev) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        Self::max_consecutive(bottom, top, special)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
