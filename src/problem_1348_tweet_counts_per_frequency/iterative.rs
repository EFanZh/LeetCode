// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::{BTreeMap, HashMap};
use std::num::NonZeroU32;

pub struct TweetCounts {
    tweets: HashMap<Box<str>, BTreeMap<u32, u32>>,
}

impl TweetCounts {
    fn new() -> Self {
        Self { tweets: HashMap::new() }
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        let time = time as u32;

        self.tweets
            .entry(tweet_name.into_boxed_str())
            .and_modify(|timeline| {
                timeline.entry(time).and_modify(|count| *count += 1).or_insert(1);
            })
            .or_insert_with(|| BTreeMap::from([(time, 1)]));
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        let start_time = start_time as u32;
        let end_time = end_time as u32;

        let interval = NonZeroU32::new(match freq.as_str() {
            "day" => 86_400,
            "hour" => 3_600,
            _ => 60,
        })
        .unwrap();

        drop(freq);

        let timeline = self.tweets.get(tweet_name.as_str());

        drop(tweet_name);

        let mut result = vec![0; ((end_time - start_time + interval.get()) / interval) as _];

        if let Some(timeline) = timeline {
            for (&time, &count) in timeline.range(start_time..=end_time) {
                result[((time - start_time) / interval) as usize] += count as i32;
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::TweetCounts for TweetCounts {
    fn new() -> Self {
        Self::new()
    }

    fn record_tweet(&mut self, tweet_name: String, time: i32) {
        self.record_tweet(tweet_name, time);
    }

    fn get_tweet_counts_per_frequency(
        &self,
        freq: String,
        tweet_name: String,
        start_time: i32,
        end_time: i32,
    ) -> Vec<i32> {
        self.get_tweet_counts_per_frequency(freq, tweet_name, start_time, end_time)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::TweetCounts>();
    }
}
