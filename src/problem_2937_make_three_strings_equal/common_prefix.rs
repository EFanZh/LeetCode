pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let common_prefix_length = s1
            .bytes()
            .zip(s2.bytes())
            .zip(s3.bytes())
            .take_while(|((x, y), z)| x == y && x == z)
            .count();

        if common_prefix_length == 0 {
            -1
        } else {
            ((s1.len() + s2.len() + s3.len()) - common_prefix_length * 3) as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        Self::find_minimum_operations(s1, s2, s3)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
