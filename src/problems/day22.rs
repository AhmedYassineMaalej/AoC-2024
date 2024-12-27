use std::collections::HashMap;

fn next_secret(mut secret: i64) -> i64 {
    secret = prune(mix(secret << 6, secret));
    let secret = mix(secret, secret >> 5);
    prune(mix(secret << 11, secret))
}

fn prune(secret: i64) -> i64 {
    secret & 0b111_111_111_111_111_111_111_111
}

fn mix(value: i64, secret: i64) -> i64 {
    value ^ secret
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut secret: i64 = line.parse().unwrap();

        for _ in 0..2000 {
            secret = next_secret(secret);
        }

        sum += secret;
    }

    sum
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut changes_map: HashMap<Vec<i64>, i64> = HashMap::new();

    for line in input.lines() {
        let mut secret: i64 = line.parse().unwrap();

        let mut prices = Vec::with_capacity(2001);
        prices.push(secret % 10);

        for _ in 0..2000 {
            secret = next_secret(secret);
            prices.push(secret % 10);
        }

        let mut changes = Vec::with_capacity(2000);
        for window in prices.windows(2) {
            changes.push(window[1] - window[0]);
        }

        let mut temp_changes_map: HashMap<Vec<i64>, i64> = HashMap::new();
        for (idx, window) in changes.windows(4).enumerate() {
            let key = window.to_vec();
            if temp_changes_map.contains_key(&key) {
                continue;
            }
            temp_changes_map.insert(key, prices[idx + 4]);
        }

        for (key, value) in temp_changes_map.drain() {
            *changes_map.entry(key).or_default() += value;
        }
    }

    changes_map.into_values().max().unwrap()
}
