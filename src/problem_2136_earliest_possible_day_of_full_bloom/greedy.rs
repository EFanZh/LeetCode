pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;

impl Solution {
    pub fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        let mut plant_time = plant_time.into_iter().map(|x| x as u32).collect::<Vec<_>>();
        let grow_time = grow_time.into_iter().map(|x| x as u32).collect::<Vec<_>>();

        for (plant_time, grow_time) in plant_time.iter_mut().zip(grow_time) {
            *plant_time |= grow_time << 16;
        }

        plant_time.sort_unstable_by_key(|&x| Reverse(x));

        let mut result = 0;
        let mut time = 0;

        for state in plant_time {
            let plant_time = state & ((1 << 16) - 1);
            let grow_time = state >> 16;

            time += plant_time;
            result = result.max(time + grow_time);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn earliest_full_bloom(plant_time: Vec<i32>, grow_time: Vec<i32>) -> i32 {
        Self::earliest_full_bloom(plant_time, grow_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
