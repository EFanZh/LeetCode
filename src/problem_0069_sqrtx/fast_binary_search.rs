pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as u32;
        let mut left = 0;
        let mut count = x.min(46340) + 1;

        while count != 1 {
            let half = count / 2;
            let middle = left + half;

            if middle * middle <= x {
                left = middle;
            }

            count -= half;
        }

        left as _
    }
}

impl super::Solution for Solution {
    fn my_sqrt(x: i32) -> i32 {
        Self::my_sqrt(x)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
