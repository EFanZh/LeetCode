pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(unused_variables)] // Expected.
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = Vec::new();
        let mut i = 0;

        for value in target {
            loop {
                result.push(String::from("Push"));
                i += 1;

                if i == value {
                    break;
                }

                result.push(String::from("Pop"));
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        Self::build_array(target, n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
