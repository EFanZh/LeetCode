pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut clips = clips
            .into_iter()
            .map(|clip| {
                let [start, end] = <[_; 2]>::try_from(clip).ok().unwrap();

                (start as u32, end as u32)
            })
            .collect::<Vec<_>>();

        let time = time as u32;

        clips.sort_unstable_by_key(|&(start, _)| start);

        let mut max_end_1 = 0;
        let mut max_end_2 = 0;
        let mut result = 0;

        for (start, end) in clips {
            if start > max_end_2 {
                return -1;
            } else if end >= time {
                return result + if start <= max_end_1 { 1 } else { 2 };
            }

            if start > max_end_1 {
                max_end_1 = max_end_2;
                result += 1;
            }

            max_end_2 = max_end_2.max(end);
        }

        -1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        Self::video_stitching(clips, time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
