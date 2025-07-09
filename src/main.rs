use rayon::prelude::*;

type Coin = u16;

fn main() {
    for n in 5..=20 {
        let max_change = n;
        let max_coin_denomination = n;
        let max_coin_count = n;

        for coin_count in 2..=max_coin_count {
        // for coin_count in 5..=5 {
            use std::time::Instant;
            let start = Instant::now();

            if let Some((mut solution, total)) = (IteratorState {
                max_coin_denomination,
                state: (0..coin_count).collect::<Vec<_>>(),
            })
            .par_bridge()
            .map(|coins| {
                let count = count_coins(max_change, &coins);
                (coins, count)
            })
            .fold(
                || (vec![], (max_change * max_change) as usize),
                |(mut acc, best), (coins, total)| match total.cmp(&best) {
                    std::cmp::Ordering::Less => (vec![coins], total),
                    std::cmp::Ordering::Equal => {
                        acc.push(coins);
                        (acc, best)
                    }
                    std::cmp::Ordering::Greater => (acc, best),
                },
            )
            .reduce_with(|(mut left, left_best), (mut right, right_best)| {
                match left_best.cmp(&right_best) {
                    std::cmp::Ordering::Less => (left, left_best),
                    std::cmp::Ordering::Equal => {
                        left.append(&mut right);
                        (left, left_best)
                    }
                    std::cmp::Ordering::Greater => (right, right_best),
                }
            })
            // .min_by_key(|(_, total)| *total)
            {
                let average: f64 = (total as f64) / (max_change as f64);
                println!(
                    "max_change {max_change:3} coin_count {coin_count} solutions count {:3} time {:3.2?} average {average}",
                    solution.len(),
                    start.elapsed()
                );

                solution.sort();

                for coins in solution.into_iter().take(5) {
                    for (i, coin) in coins.iter().enumerate() {
                        if i == 0 {
                            print!("  ");
                        }
                        print!("{coin:3}");
                    }
                    println!();
                }
                // println!("  {:?}", solution);
            }
        }
    }
}

struct IteratorState {
    max_coin_denomination: Coin,
    state: Vec<Coin>,
}

impl Iterator for IteratorState {
    type Item = Vec<Coin>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_empty() {
            return None;
        }

        if self.state[0] == 0 {
            self.state = (1..=(self.state.len() as Coin)).collect::<Vec<_>>();
            return Some(self.state.clone());
        }

        for i in (1..self.state.len()).rev() {
            if self.state[i] < self.max_coin_denomination + 1 - (self.state.len() - i) as Coin {
                let from = self.state[i];
                self.state[i] += 1;
                for j in (i + 1)..self.state.len() {
                    let next = from + ((j - i) as Coin);
                    self.state[j] = next;
                }
                // println!("{:?}", self.state);
                return Some(self.state.clone());
            }
        }
        return None;
    }
}

fn count_coins(max_change: Coin, coins: &Vec<Coin>) -> usize {
    let mut counts: Vec<usize> = Vec::with_capacity(1 + max_change as usize);
    counts.push(0);

    for i in 1..=max_change {
        let mut best = i as usize;
        for coin in coins.into_iter() {
            if i == *coin {
                best = 1;
                break;
            }
            if *coin > i {
                break;
            }
            if i - *coin == 4 && counts.len() == 4 {
                println!("{:?}", coins);
            }
            best = std::cmp::min(best, counts[(i - *coin) as usize] + 1);
        }

        counts.push(best);
    }

    counts.iter().sum()
}
