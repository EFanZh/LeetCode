pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{HashMap, VecDeque};
use std::mem;

impl Solution {
    fn get_key(s: &str) -> u64 {
        let mut bytes = [0; 8];

        for (target, c) in bytes.iter_mut().zip(s.bytes()) {
            *target = c;
        }

        u64::from_be_bytes(bytes)
    }

    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let mut watched_videos = watched_videos;
        let n = watched_videos.len();
        let id = id as usize;
        let mut queue = VecDeque::from([id]);
        let mut visited = vec![false; n];

        visited[id] = true;

        for _ in 0..level {
            for _ in 0..queue.len() {
                let current = queue.pop_front().unwrap();

                for &next in &friends[current] {
                    let next = next as usize;

                    if let visited_value @ false = &mut visited[next] {
                        *visited_value = true;

                        queue.push_back(next);
                    }
                }
            }

            if queue.is_empty() {
                break;
            }
        }

        let mut frequencies = HashMap::<_, u16>::new();
        let mut result = Vec::new();

        for friend in queue {
            for video in mem::take(&mut watched_videos[friend]) {
                frequencies
                    .entry(Self::get_key(&video))
                    .and_modify(|count| *count += 1)
                    .or_insert_with(|| {
                        result.push(video);

                        1
                    });
            }
        }

        result.sort_unstable_by_key(|video| {
            let key = Self::get_key(video);

            (frequencies[&key], key)
        });

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        Self::watched_videos_by_friends(watched_videos, friends, id, level)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
