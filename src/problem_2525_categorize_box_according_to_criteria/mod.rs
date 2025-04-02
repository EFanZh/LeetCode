pub mod brute_force;

pub trait Solution {
    fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((1000, 35, 700, 300), "Heavy"),
            ((200, 50, 800, 50), "Neither"),
            ((92487, 6200, 58423, 40), "Bulky"),
            ((2909, 3968, 3272, 727), "Both"),
        ];

        for ((length, width, height, mass), expected) in test_cases {
            assert_eq!(S::categorize_box(length, width, height, mass), expected);
        }
    }
}
