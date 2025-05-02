use rayon::prelude::*;

fn main() {
    let max_change = 500;

    for coin_count in 2..8 {
        let mut averages = (IteratorState {
            max_change,
            state: (1..=coin_count).collect::<Vec<_>>(),
        })
        .par_bridge()
        .map(|coins| {
            let count = count_coins(max_change, &coins);
            (coins, count)
        })
        .collect::<Vec<_>>();

        averages.par_sort_unstable_by(|(_, total_a), (_, total_b)| total_a.cmp(total_b));

        let best = if let Some((_, best)) = averages.get(0) {
            *best
        } else {
            continue;
        };

        println!(
            "The best result is an average of {} coins",
            (best as f64) / (max_change as f64)
        );

        for (coins, result) in averages.into_iter() {
            if result > best {
                break;
            }

            println!("  {:?}", coins);
        }
    }
}

struct IteratorState {
    max_change: usize,
    state: Vec<usize>,
}

impl Iterator for IteratorState {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (1..self.state.len()).rev() {
            if self.state[i] < self.max_change - 1 {
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
