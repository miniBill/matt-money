use rayon::prelude::*;

fn main() {
    let max_change = 100;
    let max_coin_denomination = 100;

    for coin_count in 2..9 {
        use std::time::Instant;
        let start = Instant::now();

        let (coins, total) = (IteratorState {
            max_coin_denomination,
            state: (1..=coin_count).collect::<Vec<_>>(),
        })
        .par_bridge()
        .map(|coins| {
            let count = count_coins(max_change, &coins);
            (coins, count)
        })
        .min_by(|(_, total_a), (_, total_b)| total_a.cmp(total_b))
        .expect("The list is not empty");

        println!(
            "The best result is an average of {} coins, found in {:.2?}",
            (total as f64) / (max_change as f64),
            start.elapsed()
        );

        println!("  {:?}", coins);
    }
}

struct IteratorState {
    max_coin_denomination: usize,
    state: Vec<usize>,
}

impl Iterator for IteratorState {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (1..self.state.len()).rev() {
            if self.state[i] < self.max_coin_denomination - 1 {
                let from = self.state[i];
                self.state[i] += 1;
                for j in (i + 1)..self.state.len() {
                    self.state[j] = from + j - i;
                }
                break;
            }
            if i == 1 {
                return None;
            }
        }
        Some(self.state.clone())
    }
}

fn count_coins(max_change: usize, coins: &Vec<usize>) -> usize {
    let mut counts: Vec<usize> = vec![0];

    for i in 1..max_change {
        let mut best = i;
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
            best = std::cmp::min(best, counts[i - *coin] + 1);
        }

        counts.push(best);
    }

    counts.iter().sum()
}
