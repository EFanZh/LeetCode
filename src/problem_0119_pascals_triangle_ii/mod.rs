pub mod iterative;

pub trait Solution {
    fn get_row(row_index: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(0, &[1] as &[_]), (3, &[1, 3, 3, 1]), (4, &[1, 4, 6, 4, 1])];

        for (num_rows, expected) in test_cases {
            assert_eq!(S::get_row(num_rows), expected);
        }
    }
}
