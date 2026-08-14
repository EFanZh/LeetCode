pub mod greedy;

pub trait Solution {
    fn create_grid(m: i32, n: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(2, 3), (3, 3), (1, 4)];

        for (m, n) in test_cases {
            let mut iter = S::create_grid(m, n)
                .into_iter()
                .map(|s| {
                    let mut s = s.into_bytes();

                    for c in &mut s {
                        assert!(matches!(c, b'.' | b'#'));

                        *c = u8::from(*c == b'.');
                    }

                    s
                })
                .collect::<Vec<_>>()
                .into_iter();

            let mut prev = iter.next().unwrap();

            for mut current in iter {
                let mut left = 0;

                current.iter_mut().zip(prev).for_each(|(target, top)| {
                    *target *= (left + top).min(2);
                    left = *target;
                });

                prev = current;
            }

            assert_eq!(*prev.last().unwrap(), 1);
        }
    }
}
