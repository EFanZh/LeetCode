pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct QueueItem {
    distance: u16,
    length: u16,
    left_finger: u8,
    right_finger: u8,
}

impl Eq for QueueItem {}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == Ordering::Equal
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&other.distance, &self.distance)
    }
}

impl Solution {
    fn distance(from: u8, to: u8) -> u16 {
        let from_row = from / 6;
        let from_column = from % 6;
        let to_row = to / 6;
        let to_column = to % 6;

        u16::from(from_row.abs_diff(to_row) + from_column.abs_diff(to_column))
    }

    pub fn minimum_distance(word: String) -> i32 {
        let word = word.into_bytes();
        let mut distances = vec![u16::MAX; 26 * word.len()];
        let mut queue = BinaryHeap::new();

        let mut item = QueueItem {
            distance: 0,
            length: 1,
            left_finger: u8::MAX,
            right_finger: word[0] - b'A',
        };

        while let Some(&next_character) = word.get(usize::from(item.length)) {
            let next_length = item.length + 1;
            let next_character = next_character - b'A';

            for (next_distance, next_left) in [
                (
                    if item.left_finger == u8::MAX {
                        item.distance
                    } else {
                        item.distance + Self::distance(item.left_finger, next_character)
                    },
                    item.right_finger,
                ),
                (
                    item.distance + Self::distance(item.right_finger, next_character),
                    item.left_finger,
                ),
            ] {
                if next_left == u8::MAX || {
                    let current_distance = &mut distances[26 * usize::from(item.length) + usize::from(next_left)];

                    if next_distance < *current_distance {
                        *current_distance = next_distance;

                        true
                    } else {
                        false
                    }
                } {
                    queue.push(QueueItem {
                        distance: next_distance,
                        length: next_length,
                        left_finger: next_left,
                        right_finger: next_character,
                    });
                }
            }

            loop {
                let next = queue.pop().unwrap();

                if next.left_finger == u8::MAX
                    || next.distance == distances[26 * (usize::from(next.length) - 1) + usize::from(next.left_finger)]
                {
                    item = next;

                    break;
                }
            }
        }

        i32::from(item.distance)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_distance(word: String) -> i32 {
        Self::minimum_distance(word)
    }
}

#[cfg(test)]
mod tests {
    use super::QueueItem;

    #[test]
    fn test_queue_item_partial_eq() {
        assert!(
            QueueItem {
                distance: 2,
                length: 3,
                left_finger: 5,
                right_finger: 7,
            } == QueueItem {
                distance: 2,
                length: 11,
                left_finger: 13,
                right_finger: 17,
            },
        );

        assert!(
            QueueItem {
                distance: 2,
                length: 3,
                left_finger: 5,
                right_finger: 7,
            } != QueueItem {
                distance: 11,
                length: 3,
                left_finger: 5,
                right_finger: 7,
            },
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
