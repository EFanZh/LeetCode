pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let ring_size = ring.len() as i32;
        let mut indices = <[Vec<_>; 26]>::default();

        for (i, c) in (0..).zip(ring.into_bytes()) {
            indices[usize::from(c - b'a')].push(i);
        }

        let mut cache = vec![(0, 0)];
        let mut temp = Vec::new();

        for c in key.into_bytes() {
            temp.extend(indices[usize::from(c - b'a')].iter().map(|&i| {
                (
                    i,
                    cache
                        .iter()
                        .map(|&(j, value)| {
                            let (x, y) = if j < i { (j, i) } else { (i, j) };
                            let d = y - x;

                            value + d.min(ring_size - d)
                        })
                        .min()
                        .unwrap()
                        + 1,
                )
            }));

            mem::swap(&mut cache, &mut temp);

            temp.clear();
        }

        cache.into_iter().map(|(_, value)| value).min().unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn find_rotate_steps(ring: String, key: String) -> i32 {
        Self::find_rotate_steps(ring, key)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
