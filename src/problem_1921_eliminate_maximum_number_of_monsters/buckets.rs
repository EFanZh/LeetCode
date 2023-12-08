pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut buckets = vec![0_u32; dist.len()].into_boxed_slice();

        for (distance, speed) in dist.into_iter().zip(speed) {
            if let Some(count) = buckets.get_mut(((distance - 1) as u32 / speed as u32) as usize) {
                *count += 1;
            }
        }

        let mut need_to_kill = 0;

        for (can_kill, &count) in (1..).zip(&*buckets) {
            need_to_kill += count;

            if need_to_kill > can_kill {
                return can_kill as _;
            }
        }

        buckets.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        Self::eliminate_maximum(dist, speed)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
