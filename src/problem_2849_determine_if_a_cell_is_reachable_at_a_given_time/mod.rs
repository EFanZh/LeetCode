pub mod mathematical;

pub trait Solution {
    fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ((2, 4, 7, 7, 6), true),
            ((3, 1, 7, 3, 3), false),
            ((1, 2, 1, 2, 1), false),
        ];

        for ((sx, sy, fx, fy, t), expected) in test_cases {
            assert_eq!(S::is_reachable_at_time(sx, sy, fx, fy, t), expected);
        }
    }
}
