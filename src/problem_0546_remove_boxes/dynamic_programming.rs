pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

// https://leetcode.com/problems/remove-boxes/discuss/101310/Java-top-down-and-bottom-up-DP-solutions.

struct Cache {
    buffer: Box<[i32]>,
    n: usize,
    n_squared: usize,
}

impl Cache {
    fn new(n: usize) -> Self {
        Self {
            buffer: vec![0; n * n * (n - 1)].into_boxed_slice(),
            n,
            n_squared: n * n,
        }
    }

    fn get(&self, length: usize, start: usize, k: usize) -> i32 {
        match length {
            0 => 0,
            1 => ((k + 1) * (k + 1)) as _,
            _ => self.buffer[self.n_squared * (length - 2) + self.n * start + k],
        }
    }

    fn set(&mut self, length: usize, start: usize, k: usize, value: i32) {
        self.buffer[self.n_squared * (length - 2) + self.n * start + k] = value;
    }
}

impl Solution {
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let n = boxes.len();
        let mut cache = Cache::new(n);

        for length in 2..=n {
            for start in 0..=(n - length) {
                for k in 0..=start {
                    let mut max_points = (k as i32 + 1) * (k as i32 + 1) + cache.get(length - 1, start + 1, 0);
                    let (left_color, rest) = boxes[start..start + length].split_first().unwrap();

                    for (i, color) in rest.iter().enumerate() {
                        if color == left_color {
                            max_points = max_points
                                .max(cache.get(i, start + 1, 0) + cache.get(length - 1 - i, start + 1 + i, k + 1));
                        }
                    }

                    cache.set(length, start, k, max_points);
                }
            }
        }

        cache.get(n, 0, 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn remove_boxes(boxes: Vec<i32>) -> i32 {
        Self::remove_boxes(boxes)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
