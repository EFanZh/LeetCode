pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles
            .into_iter()
            .map(|tile| {
                let [start, end] = <[_; 2]>::map(tile.as_slice().try_into().ok().unwrap(), |x| x as u32);

                (start, end + 1)
            })
            .collect::<Vec<_>>();

        let carpet_len = carpet_len as u32;
        let mut start_index = 0;
        let mut covered = 0;
        let mut result = 0;

        tiles.sort_unstable_by_key(|&(start, _)| start);

        for &(right_start, right_end) in &tiles {
            let (mut left_start, mut left_end);

            loop {
                (left_start, left_end) = tiles[start_index];

                if left_end + carpet_len <= right_end {
                    covered -= left_end - left_start;
                    start_index += 1;
                } else {
                    break;
                }
            }

            covered += right_end - right_start;

            let candidate = covered - right_end.saturating_sub(left_start + carpet_len);

            if candidate == carpet_len {
                return candidate as _;
            }

            result = result.max(candidate);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        Self::maximum_white_tiles(tiles, carpet_len)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
