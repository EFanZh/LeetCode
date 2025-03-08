pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        let n = n as u32 as usize;

        let mut rides = rides
            .into_iter()
            .map(|ride| {
                let [start, end, tip] = ride.try_into().ok().unwrap();

                (end as u32 - 1, start as u32 - 1, tip as u32)
            })
            .collect::<Vec<_>>();

        rides.sort_unstable_by_key(|&(end, _, _)| end);

        let mut cache = vec![0_u64; n].into_boxed_slice();
        let mut prev_position = 0;
        let mut prev_earning = 0_u64;

        for (end, start, tip) in rides {
            if end != prev_position {
                cache[prev_position as usize..end as usize].fill(prev_earning);
            }

            prev_earning = prev_earning.max(cache[start as usize] + u64::from(end - start + tip));
            prev_position = end;
        }

        prev_earning as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_taxi_earnings(n: i32, rides: Vec<Vec<i32>>) -> i64 {
        Self::max_taxi_earnings(n, rides)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
