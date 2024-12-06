// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::convert;

fn encode_name(name: &str) -> u64 {
    let mut result = 0;
    let mut offset = 64;

    for c in name.bytes() {
        offset -= 5;
        result |= u64::from(c & 0b_11111) << offset;
    }

    result
}

fn decode_name(mut name: u64, buffer: String) -> String {
    let mut result = buffer.into_bytes();

    result.clear();

    loop {
        result.push((name >> 59) as u8 | 0b_0110_0000);

        name <<= 5;

        if name == 0 {
            break;
        }
    }

    String::from_utf8(result).unwrap()
}

fn get_stat(stats: &[(u32, u64, u16)], index: u16) -> (u32, Reverse<u64>) {
    let (rating, food_name, _) = stats[usize::from(index)];

    (rating, Reverse(food_name))
}

fn heap_sift_down(
    heap: &mut [u16],
    stats: &mut [(u32, u64, u16)],
    stat_index: usize,
    rating: u32,
    food_name: u64,
    mut heap_index: usize,
) {
    let stat = (rating, Reverse(food_name));

    loop {
        let left_child_heap_index = heap_index * 2 + 1;

        if let Some(&left_child_stat_index) = heap.get(left_child_heap_index) {
            let right_child_heap_index = left_child_heap_index + 1;

            let child_heap_index = if let Some(&right_child_stat_index) = heap.get(right_child_heap_index) {
                if get_stat(stats, right_child_stat_index) > get_stat(stats, left_child_stat_index) {
                    right_child_heap_index
                } else {
                    left_child_heap_index
                }
            } else {
                left_child_heap_index
            };

            let child_stat_index = heap[child_heap_index];

            if get_stat(stats, child_stat_index) > stat {
                heap[heap_index] = child_stat_index;
                stats[usize::from(child_stat_index)].2 = heap_index as _;
                heap_index = child_heap_index;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    heap[heap_index] = stat_index as _;
    stats[stat_index].2 = heap_index as _;
}

fn heap_sift_up(
    heap: &mut [u16],
    stats: &mut [(u32, u64, u16)],
    stat_index: usize,
    rating: u32,
    food_name: u64,
    mut heap_index: usize,
) {
    let stat = (rating, Reverse(food_name));

    loop {
        let parent_heap_index = heap_index.wrapping_sub(1) / 2;

        if let Some(&parent_stat_index) = heap.get(parent_heap_index) {
            let parent_stat = get_stat(stats, parent_stat_index);

            if stat > parent_stat {
                heap[heap_index] = parent_stat_index;
                stats[usize::from(parent_stat_index)].2 = heap_index as _;
                heap_index = parent_heap_index;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    heap[heap_index] = stat_index as _;
    stats[stat_index].2 = heap_index as _;
}

fn heap_build(heap: &mut Box<[u16]>, stats: &mut [(u32, u64, u16)]) {
    let n = stats.len();

    *heap = (0..n as _).collect();

    for i in (0..n / 2).rev() {
        let (rating, food_name, heap_index) = stats[i];

        heap_sift_down(heap, stats, i, rating, food_name, usize::from(heap_index));
    }
}

fn heap_update(heap: &mut [u16], stats: &mut [(u32, u64, u16)], stat_index: u16, rating: u32) {
    let stat_index = usize::from(stat_index);
    let stat = &mut stats[stat_index];

    if rating != stat.0 {
        let (old_rating, food_name, heap_index) = *stat;
        let heap_index = usize::from(heap_index);

        stat.0 = rating;

        if rating < old_rating {
            heap_sift_down(heap, stats, stat_index, rating, food_name, heap_index);
        } else {
            heap_sift_up(heap, stats, stat_index, rating, food_name, heap_index);
        }
    }
}

struct Ratings {
    heap: Box<[u16]>,
    stats: Vec<(u32, u64, u16)>,
}

pub struct FoodRatings {
    food_to_cuisine: HashMap<u64, (u64, u16)>,
    cuisine_ratings: HashMap<u64, Ratings>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut cuisine_ratings = HashMap::<_, Ratings>::new();

        foods
            .into_iter()
            .zip(cuisines)
            .zip(ratings)
            .for_each(|((food, cuisine), rating)| {
                let food = encode_name(&convert::identity(food));
                let cuisine = encode_name(&convert::identity(cuisine));

                let stat_index = match cuisine_ratings.entry(cuisine) {
                    Entry::Occupied(occupied_entry) => {
                        let stats = &mut occupied_entry.into_mut().stats;
                        let stat_index = stats.len() as _;

                        stats.push((rating as _, food, stat_index));

                        stat_index
                    }
                    Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(Ratings {
                            heap: Box::new([]),
                            stats: vec![(rating as _, food, 0)],
                        });

                        0
                    }
                };

                food_to_cuisine.insert(food, (cuisine, stat_index));
            });

        for ratings in cuisine_ratings.values_mut() {
            heap_build(&mut ratings.heap, &mut ratings.stats);
        }

        Self {
            food_to_cuisine,
            cuisine_ratings,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food = encode_name(&convert::identity(food));
        let new_rating = new_rating as u32;
        let (cuisine, stat_index) = self.food_to_cuisine[&food];
        let ratings = self.cuisine_ratings.get_mut(&cuisine).unwrap();

        heap_update(&mut ratings.heap, &mut ratings.stats, stat_index, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let encoded_cuisine = encode_name(&cuisine);
        let ratings = &self.cuisine_ratings[&encoded_cuisine];
        let stat_index = ratings.heap[0];
        let food_name = ratings.stats[usize::from(stat_index)].1;

        decode_name(food_name, cuisine)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::FoodRatings for FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        Self::new(foods, cuisines, ratings)
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        self.change_rating(food, new_rating);
    }

    fn highest_rated(&self, cuisine: String) -> String {
        self.highest_rated(cuisine)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::FoodRatings>();
    }
}
