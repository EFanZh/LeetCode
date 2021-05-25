pub struct Solution;

use std::collections::binary_heap::PeekMut;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;

struct Twitter {
    time: u32,
    followees: HashMap<i32, HashSet<i32>>,
    tweets: HashMap<i32, Vec<(u32, i32)>>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            followees: HashMap::new(),
            tweets: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let time = self.time;

        self.time += 1;

        match self.tweets.entry(user_id) {
            Entry::Occupied(entry) => entry.into_mut().push((time, tweet_id)),
            Entry::Vacant(entry) => {
                entry.insert(vec![(time, tweet_id)]);

                self.followees
                    .entry(user_id)
                    .or_insert_with(|| iter::once(user_id).collect());
            }
        }
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(followees) = self.followees.get(&user_id) {
            let mut queue = followees
                .iter()
                .filter_map(|followee_id| self.tweets.get(followee_id).and_then(|tweets| tweets.split_last()))
                .collect::<BinaryHeap<_>>();

            for _ in 0..10 {
                if let Some(mut top) = queue.peek_mut() {
                    result.push(top.0 .1);

                    if let Some(new_top) = top.1.split_last() {
                        *top = new_top;
                    } else {
                        PeekMut::pop(top);
                    }
                } else {
                    break;
                }
            }
        }

        result
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followees
            .entry(follower_id)
            .and_modify(|followees| {
                followees.insert(followee_id);
            })
            .or_insert_with(|| [follower_id, followee_id].iter().copied().collect());
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            if let Some(followees) = self.followees.get_mut(&follower_id) {
                followees.remove(&followee_id);
            }
        }
    }
}

impl super::Twitter for Twitter {
    fn new() -> Self {
        Self::new()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.post_tweet(user_id, tweet_id);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.get_news_feed(user_id)
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follow(follower_id, followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        self.unfollow(follower_id, followee_id);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Twitter>();
    }
}
