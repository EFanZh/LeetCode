pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::mem;

impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids
            .into_iter()
            .map(|cuboids| {
                let [width, length, height] = cuboids.try_into().ok().unwrap();
                let mut result = (width as u8, length as u8, height as u8, 0_u16);

                if result.1 < result.0 {
                    mem::swap(&mut result.0, &mut result.1);
                }

                if result.2 < result.1 {
                    mem::swap(&mut result.1, &mut result.2);
                }

                if result.1 < result.0 {
                    mem::swap(&mut result.0, &mut result.1);
                }

                result
            })
            .collect::<Box<_>>();

        cuboids.sort_unstable_by_key(|&(x, y, z, _)| Reverse((x, y, z)));

        let mut result = 0;

        for i in 0..cuboids.len() {
            let (current, left_cache) = cuboids[..=i].split_last_mut().unwrap();
            let mut max_height = 0;

            for left in left_cache {
                if current.1 <= left.1 && current.2 <= left.2 {
                    max_height = max_height.max(left.3);
                }
            }

            current.3 = max_height + u16::from(current.2);
            result = result.max(current.3);
        }

        i32::from(result)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        Self::max_height(cuboids)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
