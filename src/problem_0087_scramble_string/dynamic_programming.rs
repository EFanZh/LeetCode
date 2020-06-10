pub struct Solution {}

impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        if s1.is_empty() {
            true
        } else {
            let s1 = s1.into_bytes();
            let s2 = s2.into_bytes();
            let n = s1.len();
            let n_squared = n * n;
            let mut cache = vec![false; n_squared * n];
            let index = move |length, i, j| n_squared * (length - 1) + n * i + j;

            for i in 0..n {
                for j in 0..n {
                    cache[n * i + j] = s1[i] == s2[j];
                }
            }

            for length in 2..=n {
                for i in 0..=n - length {
                    for j in 0..=n - length {
                        cache[index(length, i, j)] = (1..length).any(|k| {
                            (cache[index(k, i, j)] && cache[index(length - k, i + k, j + k)])
                                || (cache[index(k, i, j + (length - k))] && cache[index(length - k, i + k, j)])
                        });
                    }
                }
            }

            cache[index(n, 0, 0)]
        }
    }
}

impl super::Solution for Solution {
    fn is_scramble(s1: String, s2: String) -> bool {
        Self::is_scramble(s1, s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
