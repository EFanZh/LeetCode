pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryInto;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        let k = k as usize;

        if k < points.len() {
            let i = points
                .select_nth_unstable_by_key(k, |p| {
                    let [x, y]: [_; 2] = p.as_slice().try_into().unwrap();

                    x * x + y * y
                })
                .0
                .len();

            points.truncate(i);
        }

        points
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        Self::k_closest(points, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
