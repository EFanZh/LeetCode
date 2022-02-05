pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let (n, goal, k) = (n as usize, goal as usize, k as usize);
        let mut result = 0;
        let mut cache = vec![0_u32; n];

        *cache.last_mut().unwrap() = 1;

        for _ in 0..goal {
            let mut unique = n;
            let mut next = &mut result;

            for current in &mut cache {
                let new = u64::from(*current) * (n + 1 - unique) as u64;
                let old = u64::from(*next) * unique.saturating_sub(k) as u64;

                *next = ((new + old) % 1_000_000_007) as _;
                unique -= 1;
                next = current;
            }

            *next = 0;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        Self::num_music_playlists(n, goal, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
