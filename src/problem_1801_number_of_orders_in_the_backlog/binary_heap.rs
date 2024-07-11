pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

struct Order {
    price: u32,
    amount: u32,
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Order {}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl Solution {
    pub fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        let mut buy = BinaryHeap::<Order>::new();
        let mut sell = BinaryHeap::<Reverse<Order>>::new();

        'outer: for order in orders {
            let [price, amount, order_type]: [_; 3] = order.try_into().ok().unwrap();
            let price = price as u32;
            let mut amount = amount as u32;

            if order_type == 0 {
                while let Some(mut peek) = sell.peek_mut() {
                    let top = &peek.0;

                    if top.price <= price {
                        if top.amount < amount {
                            amount -= top.amount;
                            PeekMut::pop(peek);
                        } else {
                            if top.amount == amount {
                                PeekMut::pop(peek);
                            } else {
                                peek.0.amount -= amount;
                            }

                            continue 'outer;
                        }
                    } else {
                        break;
                    }
                }

                buy.push(Order { price, amount });
            } else {
                while let Some(mut peek) = buy.peek_mut() {
                    let top = &*peek;

                    if top.price >= price {
                        if top.amount < amount {
                            amount -= top.amount;
                            PeekMut::pop(peek);
                        } else {
                            if top.amount == amount {
                                PeekMut::pop(peek);
                            } else {
                                peek.amount -= amount;
                            }

                            continue 'outer;
                        }
                    } else {
                        break;
                    }
                }

                sell.push(Reverse(Order { price, amount }));
            }
        }

        let add = |x: u32, y: u32| {
            let z = x + y;

            z.checked_sub(1_000_000_007).unwrap_or(z)
        };

        let buy_orders = buy.iter().fold(0, |sum, order| add(sum, order.amount));
        let all_orders = sell.iter().fold(buy_orders, |sum, order| add(sum, order.0.amount));

        all_orders as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_number_of_backlog_orders(orders: Vec<Vec<i32>>) -> i32 {
        Self::get_number_of_backlog_orders(orders)
    }
}

#[cfg(test)]
mod tests {
    use super::Order;

    #[test]
    fn test_order_partial_eq() {
        assert!(Order { price: 2, amount: 3 } == Order { price: 2, amount: 5 });
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
