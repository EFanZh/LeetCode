pub mod mathematical;

pub trait Solution {
    fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [((1, 1, 3, 5), true), ((1, 1, 2, 2), false), ((1, 1, 1, 1), true)];

        for ((sx, sy, tx, ty), expected) in test_cases {
            assert_eq!(S::reaching_points(sx, sy, tx, ty), expected);
        }
    }
}
