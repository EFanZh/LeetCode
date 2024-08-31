// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};

struct MovieState {
    unrented: BTreeSet<(u32, u32)>,
    prices: HashMap<u32, u32>,
}

pub struct MovieRentingSystem {
    unrented: HashMap<u32, MovieState>,
    rented: BTreeSet<(u32, u32, u32)>,
}

impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        let _ = n;
        let mut unrented = HashMap::<u32, MovieState>::new();

        for entry in entries {
            let [shop, movie, price] = <[_; 3]>::map(entry.try_into().ok().unwrap(), |x| x as u32);

            match unrented.entry(movie) {
                Entry::Occupied(entry) => {
                    let state = entry.into_mut();

                    state.unrented.insert((price, shop));
                    state.prices.insert(shop, price);
                }
                Entry::Vacant(entry) => {
                    entry.insert(MovieState {
                        unrented: BTreeSet::from([(price, shop)]),
                        prices: HashMap::from([(shop, price)]),
                    });
                }
            }
        }

        Self {
            unrented,
            rented: BTreeSet::new(),
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        let mut result = Vec::new();

        if let Some(state) = self.unrented.get(&(movie as _)) {
            result.extend(state.unrented.iter().take(5).map(|&(_, shop)| shop as i32));
        }

        result
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let (shop, movie) = (shop as u32, movie as u32);
        let state = self.unrented.get_mut(&movie).unwrap();
        let price = state.prices[&shop];

        state.unrented.remove(&(price, shop));
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let (shop, movie) = (shop as u32, movie as u32);
        let state = self.unrented.get_mut(&movie).unwrap();
        let price = state.prices[&shop];

        state.unrented.insert((price, shop));
        self.rented.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop as _, movie as _])
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::MovieRentingSystem for MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        Self::new(n, entries)
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.search(movie)
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        self.rent(shop, movie);
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        self.drop(shop, movie);
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.report()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::MovieRentingSystem>();
    }
}
