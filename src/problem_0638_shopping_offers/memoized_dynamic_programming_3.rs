pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;

impl Solution {
    fn check_offer(offer: u32, needs: u32) -> bool {
        offer <= needs
            && (offer & 0x_000f_0000) <= (needs & 0x_000f_0000)
            && (offer & 0x_0000_f000) <= (needs & 0x_0000_f000)
            && (offer & 0x_0000_0f00) <= (needs & 0x_0000_0f00)
            && (offer & 0x_0000_00f0) <= (needs & 0x_0000_00f0)
            && (offer & 0x_0000_000f) <= (needs & 0x_0000_000f)
    }

    fn helper(prices: &[i32], offers: &[(u32, i32)], needs: u32, cache: &mut HashMap<u32, i32>) -> i32 {
        if needs == 0 {
            0
        } else if let Some(&result) = cache.get(&needs) {
            result
        } else {
            let mut result = prices
                .iter()
                .enumerate()
                .map(|(i, price)| price * ((needs >> (4 * i)) & 0b_1111) as i32)
                .sum();

            for &(offer, price) in offers {
                if price < result && Self::check_offer(offer, needs) {
                    result = result.min(price + Self::helper(prices, offers, needs - offer, cache));
                }
            }

            cache.insert(needs, result);

            result
        }
    }

    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        let mut offers = Vec::with_capacity(special.len());

        for offer_description in special {
            let (&price, offer) = offer_description.split_last().unwrap();

            let offer = offer
                .iter()
                .enumerate()
                .map(|(i, &count)| (count as u32) << (4 * i))
                .sum();

            offers.push((offer, price));
        }

        let max_states = needs.iter().product::<i32>() as _;

        let needs = needs
            .iter()
            .enumerate()
            .map(|(i, &count)| (count as u32) << (4 * i))
            .sum();

        Self::helper(&price, &offers, needs, &mut HashMap::with_capacity(max_states))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        Self::shopping_offers(price, special, needs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
