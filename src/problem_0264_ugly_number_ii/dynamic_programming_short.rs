pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as _;
        let mut cache = vec![0; n];

        cache[0] = 1;

        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        for i in 1..n {
            let v2 = cache[i2] * 2;
            let v3 = cache[i3] * 3;
            let v5 = cache[i5] * 5;
            let v = v2.min(v3).min(v5);

            if v == v2 {
                i2 += 1;
            }

            if v == v3 {
                i3 += 1;
            }

            if v == v5 {
                i5 += 1;
            }

            cache[i] = v;
        }

        cache[n - 1]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nth_ugly_number(n: i32) -> i32 {
        Self::nth_ugly_number(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
