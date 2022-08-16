pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

struct Cache {
    data: Box<[i32]>,
    columns: usize,
}

impl Cache {
    fn new(n: usize) -> Self {
        let columns = n - 1;

        Self {
            data: vec![0; columns * columns].into_boxed_slice(),
            columns,
        }
    }

    fn get(&self, length: usize, start: usize) -> i32 {
        self.data[self.columns * (length - 1) + start]
    }

    fn set(&mut self, length: usize, start: usize, value: i32) {
        self.data[self.columns * (length - 1) + start] = value;
    }
}

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut cache = Cache::new(n);

        for length in 2..n {
            for (start, (left, right)) in values.iter().zip(&values[length..]).enumerate() {
                let left_right = left * right;
                let mut score = i32::MAX;

                for (left_length, middle) in (1..).zip(&values[start + 1..start + length]) {
                    score = score.min(
                        left_right * middle
                            + cache.get(left_length, start)
                            + cache.get(length - left_length, start + left_length),
                    );
                }

                cache.set(length, start, score);
            }
        }

        cache.get(n - 1, 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_score_triangulation(values: Vec<i32>) -> i32 {
        Self::min_score_triangulation(values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
