use rayon::prelude::*;

type Coin = u16;

fn main() {
    let max_change = 500;
    let max_coin_denomination = 200;
    let max_coin_count = 9;

    for coin_count in 2..=max_coin_count {
        use std::time::Instant;
        let start = Instant::now();

        if let Some((coins, total)) = (IteratorState {
            max_coin_denomination,
            state: (0..coin_count).collect::<Vec<_>>(),
        })
        .par_bridge()
        .map(|coins| {
            let count = count_coins(max_change, &coins);
            (coins, count)
        })
        .min_by_key(|(_, total)| *total)
        {
            println!(
                "The best result for {coin_count} coins is an average of {} coins, found in {:.2?}",
                (total as f64) / (max_change as f64),
                start.elapsed()
            );

            println!("  {:?}", coins);
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
            if self.state[i] < self.max_coin_denomination + 1 - ((i - self.state.len()) as Coin) {
                let from = self.state[i];
                self.state[i] += 1;
                for j in (i + 1)..self.state.len() {
                    let next = from + ((j - i) as Coin);
                    self.state[j] = next;
                }
                return Some(self.state.clone());
            }
        }
        return None;
    }
}

fn count_coins(max_change: Coin, coins: &Vec<Coin>) -> usize {
    let mut counts: Vec<usize> = Vec::with_capacity(max_change as usize);
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
