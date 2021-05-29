use super::Solution as _;

// ------------------------------------------------------ snip ------------------------------------------------------ //

pub struct Solution {
    bad: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut start = 1;
        let mut count = n;

        while count != 0 {
            let half = count / 2;
            let middle = start + half;

            if self.isBadVersion(middle) {
                count = half;
            } else {
                start = middle + 1;
                count -= half + 1;
            }
        }

        start
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn new(bad: i32) -> Self {
        Self { bad }
    }

    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }

    fn first_bad_version(&self, n: i32) -> i32 {
        self.first_bad_version(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
