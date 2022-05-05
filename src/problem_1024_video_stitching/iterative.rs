pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::convert::TryFrom;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut clips = clips
            .into_iter()
            .map(|clip| {
                let [start, end] = <[_; 2]>::try_from(clip).unwrap();

                (start as u32, end as u32)
            })
            .collect::<Vec<_>>();

        let time = time as u32;

        clips.sort_unstable_by_key(|&(start, _)| start);

        let mut result = 1;
        let mut iter = clips.into_iter();
        let mut prev_end = 0;
        let mut clip = iter.next().unwrap();

        loop {
            if clip.0 <= prev_end {
                let mut max_end = clip.1;

                loop {
                    if max_end < time {
                        if let Some(next_clip) = iter.next() {
                            if next_clip.0 <= prev_end {
                                max_end = max_end.max(next_clip.1);
                            } else {
                                prev_end = max_end;
                                clip = next_clip;

                                break;
                            }
                        } else {
                            return -1;
                        }
                    } else {
                        return result;
                    }
                }

                result += 1;
            } else {
                return -1;
            }
        }
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
