pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        let mut height = height;
        let mut prev = 0;
        let mut i = -1;

        height.retain_mut(|slot| {
            i += 1;

            let keep = prev > threshold;

            prev = *slot;

            if keep {
                *slot = i;
            }

            keep
        });

        height
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn stable_mountains(height: Vec<i32>, threshold: i32) -> Vec<i32> {
        Self::stable_mountains(height, threshold)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
