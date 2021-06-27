pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as _;
        let mut cache = Vec::with_capacity(n);
        let mut indices = vec![0; primes.len()];
        let mut prev = 1;

        cache.push(1);

        while cache.len() != n {
            let mut v = i32::MAX;

            for (index, &prime) in indices.iter_mut().zip(&primes) {
                let new_value = cache[*index] * prime;

                if new_value == prev {
                    *index += 1;

                    v = v.min(cache[*index] * prime);
                } else {
                    v = v.min(new_value);
                }
            }

            cache.push(v);
            prev = v;
        }

        prev
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        Self::nth_super_ugly_number(n, primes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
