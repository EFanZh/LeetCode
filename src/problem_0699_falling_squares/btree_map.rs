pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::BTreeMap;
use std::convert::TryInto;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::with_capacity(positions.len());
        let mut skyline = BTreeMap::new();
        let mut max_height = 0;
        let mut to_remove = Vec::new();

        for position in positions {
            let [left, length]: [i32; 2] = position.as_slice().try_into().unwrap();
            let right = left + length;
            let mut iter = skyline.range(..=right).map(|(&x, &h)| (x, h)).rev();

            let height = if let Some((last_x, last_height)) = iter.next() {
                if last_x > left {
                    let mut base = 0;

                    for (x, h) in iter {
                        base = base.max(h);

                        if x <= left {
                            break;
                        }

                        to_remove.push(x);
                    }

                    for x in to_remove.drain(..) {
                        skyline.remove(&x);
                    }

                    if last_x == right {
                        let height = base + length;

                        skyline.insert(left, height);

                        height
                    } else {
                        let height = base.max(last_height) + length;

                        skyline.remove(&last_x);
                        skyline.insert(left, height);
                        skyline.insert(right, last_height);

                        height
                    }
                } else {
                    let height = last_height + length;

                    skyline.insert(left, height);
                    skyline.insert(right, last_height);

                    height
                }
            } else {
                skyline.insert(left, length);
                skyline.insert(right, 0);

                length
            };

            max_height = max_height.max(height);

            result.push(max_height);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        Self::falling_squares(positions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
