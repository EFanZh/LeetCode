pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut tiles = tiles
            .into_iter()
            .map(|tile| {
                let [start, end]: [_; 2] = tile.as_slice().try_into().ok().unwrap();

                (start, end + 1)
            })
            .collect::<Vec<_>>();

        let mut start_index = 0;
        let mut start_position = i32::MIN;
        let mut covered = 0;
        let mut result = 0;

        tiles.sort_unstable_by_key(|&(start, _)| start);

        for &(right_start, right_end) in &tiles {
            let (mut left_start, mut left_end);

            loop {
                (left_start, left_end) = tiles[start_index];

                if start_position < left_start {
                    if left_start + carpet_len < right_start {
                        start_position = left_start;
                    } else {
                        break;
                    }
                }

                if left_end + carpet_len < right_start {
                    covered -= left_end - start_position;
                    start_position = left_end;
                    start_index += 1;
                } else {
                    covered -= right_start - carpet_len - start_position;

                    break;
                }
            }

            start_position = right_start - carpet_len;

            loop {
                if start_position < left_start {
                    if left_start + carpet_len < right_end {
                        covered += left_start - start_position;
                    } else {
                        covered += right_end - carpet_len - start_position;

                        break;
                    }
                }

                if left_end + carpet_len < right_end {
                    start_position = left_end;
                    start_index += 1;
                    (left_start, left_end) = tiles[start_index];
                } else {
                    break;
                }
            }

            if covered == carpet_len {
                result = covered;

                break;
            }

            start_position = right_end - carpet_len;
            result = result.max(covered);
        }

        result
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
