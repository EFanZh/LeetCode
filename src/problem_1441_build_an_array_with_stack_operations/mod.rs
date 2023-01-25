pub mod iterative;

pub trait Solution {
    fn build_array(target: Vec<i32>, n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[1, 3] as &[_], 3), (&[1, 2, 3], 3), (&[1, 2], 4)];

        for (target, n) in test_cases {
            let result = S::build_array(target.to_vec(), n);
            let mut buffer = Vec::new();
            let mut iter = 1..=n;

            for operation in result {
                if operation == "Push" {
                    buffer.push(iter.next().unwrap());
                } else {
                    assert_eq!(operation, "Pop");

                    buffer.pop().unwrap();
                }
            }

            assert_eq!(buffer, target);
        }
    }
}
