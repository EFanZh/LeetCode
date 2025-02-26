pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Stats {
    total_views: u32,
    most_popular_video: String,
    most_popular_video_views: u32,
}

impl Solution {
    pub fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        let mut videos_by_creators = HashMap::<String, Stats>::new();

        creators
            .into_iter()
            .zip(ids)
            .zip(views)
            .for_each(|((creator, id), view)| {
                let view = view as u32;

                match videos_by_creators.entry(creator) {
                    Entry::Occupied(occupied_entry) => {
                        let stats = occupied_entry.into_mut();

                        stats.total_views += view;

                        match view.cmp(&stats.most_popular_video_views) {
                            Ordering::Less => {}
                            Ordering::Equal => {
                                if id < stats.most_popular_video {
                                    stats.most_popular_video = id;
                                }
                            }
                            Ordering::Greater => {
                                stats.most_popular_video = id;
                                stats.most_popular_video_views = view;
                            }
                        }
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(Stats {
                            total_views: view,
                            most_popular_video: id,
                            most_popular_video_views: view,
                        });
                    }
                }
            });

        let mut result = Vec::new();
        let mut max_total_views = 0;

        for (creator, stats) in videos_by_creators {
            match stats.total_views.cmp(&max_total_views) {
                Ordering::Less => continue,
                Ordering::Equal => {}
                Ordering::Greater => {
                    max_total_views = stats.total_views;
                    result.clear();
                }
            }

            result.push((creator, stats.most_popular_video));
        }

        result.into_iter().map(|(creator, id)| vec![creator, id]).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn most_popular_creator(creators: Vec<String>, ids: Vec<String>, views: Vec<i32>) -> Vec<Vec<String>> {
        Self::most_popular_creator(creators, ids, views)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
