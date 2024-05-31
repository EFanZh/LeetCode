// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};

struct Item<K> {
    price: K,
    timestamp: i32,
}

impl<K> PartialEq for Item<K>
where
    K: Ord,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl<K> Eq for Item<K> where K: Ord {}

impl<K> PartialOrd for Item<K>
where
    K: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K> Ord for Item<K>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

pub struct StockPrice {
    prices: HashMap<i32, i32>,
    max_prices: BinaryHeap<Item<i32>>,
    min_prices: BinaryHeap<Item<Reverse<i32>>>,
    current_time: i32,
    current_price: i32,
}

impl StockPrice {
    fn new() -> Self {
        Self {
            prices: HashMap::new(),
            max_prices: BinaryHeap::new(),
            min_prices: BinaryHeap::new(),
            current_time: 0,
            current_price: 0,
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.prices.insert(timestamp, price);

        self.max_prices.push(Item { price, timestamp });

        self.min_prices.push(Item {
            price: Reverse(price),
            timestamp,
        });

        if timestamp >= self.current_time {
            self.current_time = timestamp;
            self.current_price = price;
        }
    }

    fn current(&self) -> i32 {
        self.current_price
    }

    fn maximum(&mut self) -> i32 {
        loop {
            let top = self.max_prices.peek().unwrap();

            if top.price == self.prices[&top.timestamp] {
                return top.price;
            }

            self.max_prices.pop();
        }
    }

    fn minimum(&mut self) -> i32 {
        loop {
            let top = self.min_prices.peek().unwrap();

            if top.price.0 == self.prices[&top.timestamp] {
                return top.price.0;
            }

            self.min_prices.pop();
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::StockPrice for StockPrice {
    fn new() -> Self {
        Self::new()
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.update(timestamp, price);
    }

    fn current(&self) -> i32 {
        self.current()
    }

    fn maximum(&mut self) -> i32 {
        self.maximum()
    }

    fn minimum(&mut self) -> i32 {
        self.minimum()
    }
}

#[cfg(test)]
mod tests {
    use super::Item;
    use std::cmp::Reverse;

    #[test]
    fn test_item_partial_eq() {
        assert!(Item { price: 2, timestamp: 3 } == Item { price: 2, timestamp: 5 });

        assert!(
            Item {
                price: Reverse(2),
                timestamp: 3
            } == Item {
                price: Reverse(2),
                timestamp: 5
            }
        );
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::StockPrice>();
    }
}
