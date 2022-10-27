pub mod mathematical;

pub trait Solution {
    fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((16, 7), &[1, 6] as &[_]),
            ((17, 4), &[]),
            ((4, 17), &[]),
            ((2_385_088, 164_763), &[]),
        ];

        for ((tomato_slices, cheese_slices), expected) in test_cases {
            assert_eq!(S::num_of_burgers(tomato_slices, cheese_slices), expected);
        }
    }
}
