pub mod binary_heap;
pub mod binary_heap_2;

pub trait Twitter {
    fn new() -> Self;
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32);
    fn get_news_feed(&self, user_id: i32) -> Vec<i32>;
    fn follow(&mut self, follower_id: i32, followee_id: i32);
    fn unfollow(&mut self, follower_id: i32, followee_id: i32);
}

#[cfg(test)]
mod tests {
    use super::Twitter;

    #[derive(Clone, Copy)]
    enum Operation<'a> {
        PostTweet(i32, i32),
        GetNewsFeed(i32, &'a [i32]),
        Follow(i32, i32),
        Unfollow(i32, i32),
    }

    #[allow(clippy::similar_names)]
    pub fn run<T: Twitter>() {
        use Operation::{Follow, GetNewsFeed, PostTweet, Unfollow};

        let test_cases = [
            &[
                PostTweet(1, 5),
                GetNewsFeed(1, &[5]),
                Follow(1, 2),
                PostTweet(2, 6),
                GetNewsFeed(1, &[6, 5]),
                Unfollow(1, 2),
                GetNewsFeed(1, &[5]),
            ] as &[_],
            &[PostTweet(1, 5), Follow(1, 1), GetNewsFeed(1, &[5])],
            &[PostTweet(1, 5), Unfollow(1, 1), GetNewsFeed(1, &[5])],
        ];

        for operations in test_cases.iter().copied() {
            let mut twitter = T::new();

            for operation in operations {
                match *operation {
                    PostTweet(user_id, tweet_id) => twitter.post_tweet(user_id, tweet_id),
                    GetNewsFeed(user_id, expected) => assert_eq!(twitter.get_news_feed(user_id), expected),
                    Follow(follower_id, followee_id) => twitter.follow(follower_id, followee_id),
                    Unfollow(follower_id, followee_id) => twitter.unfollow(follower_id, followee_id),
                }
            }
        }
    }
}
