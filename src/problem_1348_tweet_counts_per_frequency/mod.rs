pub mod iterative;

pub trait TweetCounts {
    fn new() -> Self;

    fn record_tweet(&mut self, tweet_name: String, time: i32);

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::TweetCounts;

    enum Operation {
        RecordTweet(&'static str, i32),
        GetTweetCountsPerFrequency((&'static str, &'static str, i32, i32), &'static [i32]),
    }

    pub fn run<T: TweetCounts>() {
        let test_cases = [
            &[
                Operation::RecordTweet("tweet3", 0),
                Operation::RecordTweet("tweet3", 60),
                Operation::RecordTweet("tweet3", 10),
                Operation::GetTweetCountsPerFrequency(("minute", "tweet3", 0, 59), &[2]),
                Operation::GetTweetCountsPerFrequency(("minute", "tweet3", 0, 60), &[2, 1]),
                Operation::RecordTweet("tweet3", 120),
                Operation::GetTweetCountsPerFrequency(("hour", "tweet3", 0, 210), &[4]),
            ] as &[_],
            &[
                Operation::RecordTweet("tweet0", 94),
                Operation::RecordTweet("tweet1", 44),
                Operation::RecordTweet("tweet2", 73),
                Operation::RecordTweet("tweet3", 21),
                Operation::RecordTweet("tweet4", 54),
                Operation::RecordTweet("tweet1", 44),
                Operation::GetTweetCountsPerFrequency(("day", "tweet2", 44, 7504), &[1]),
                Operation::GetTweetCountsPerFrequency(("hour", "tweet3", 73, 7911), &[0, 0, 0]),
            ],
            &[
                Operation::RecordTweet("tweet0", 7),
                Operation::GetTweetCountsPerFrequency(("hour", "tweet1", 1, 36), &[0]),
            ],
        ];

        for operations in test_cases {
            let mut tweet_counts = T::new();

            for operation in operations {
                match *operation {
                    Operation::RecordTweet(tweet_name, time) => tweet_counts.record_tweet(tweet_name.to_string(), time),
                    Operation::GetTweetCountsPerFrequency((freq, tweet_name, start_time, end_time), expected) => {
                        assert_eq!(
                            tweet_counts.get_tweet_counts_per_frequency(
                                freq.to_string(),
                                tweet_name.to_string(),
                                start_time,
                                end_time,
                            ),
                            expected,
                        );
                    }
                }
            }
        }
    }
}
