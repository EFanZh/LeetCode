pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut result = vec![0; n as _];
        let (left, right) = result.split_at_mut((n - k) as _);
        let mut x = 1;

        for target in left {
            *target = x;
            x += 1;
        }

        for target in right.iter_mut().skip(1).step_by(2) {
            *target = x;
            x += 1;
        }

        x = n;

        for target in right.iter_mut().step_by(2) {
            *target = x;
            x -= 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn construct_array(n: i32, k: i32) -> Vec<i32> {
        Self::construct_array(n, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
