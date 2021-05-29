// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::binary_heap::PeekMut;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::iter;

struct User {
    followees: HashSet<i32>,
    tweets: Vec<(u32, i32)>,
}

pub struct Twitter {
    time: u32,
    users: HashMap<i32, User>,
}

impl Twitter {
    fn new() -> Self {
        Self {
            time: 0,
            users: HashMap::new(),
        }
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        let time = self.time;

        self.time += 1;

        self.users
            .entry(user_id)
            .and_modify(|user| user.tweets.push((time, tweet_id)))
            .or_insert_with(|| User {
                followees: iter::once(user_id).collect(),
                tweets: vec![(time, tweet_id)],
            });
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(user) = self.users.get(&user_id) {
            let mut queue = user
                .followees
                .iter()
                .filter_map(|followee_id| {
                    self.users
                        .get(followee_id)
                        .and_then(|followee| followee.tweets.split_last())
                })
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
        self.users
            .entry(follower_id)
            .and_modify(|user| {
                user.followees.insert(followee_id);
            })
            .or_insert_with(|| User {
                followees: [follower_id, followee_id].iter().copied().collect(),
                tweets: Vec::new(),
            });
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if follower_id != followee_id {
            if let Some(user) = self.users.get_mut(&follower_id) {
                user.followees.remove(&followee_id);
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
