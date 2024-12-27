use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut computers: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let (id1, id2) = line.split_once('-').unwrap();

        connections.insert((id1, id2));
        connections.insert((id2, id1));

        computers.insert(id1);
        computers.insert(id2);
    }

    let mut triangles: HashSet<Vec<&str>> = HashSet::new();
    for &(id1, id2) in &connections {
        if !id1.starts_with('t') {
            continue;
        }

        for id3 in &computers {
            if id3 == &id1 || id3 == &id2 {
                continue;
            }

            if connections.contains(&(id1, id3)) && connections.contains(&(id2, id3)) {
                let mut triangle = vec![id1, id2, id3];

                triangle.sort_unstable();

                triangles.insert(triangle);
            }
        }
    }

    triangles.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    let mut connections: HashSet<(&str, &str)> = HashSet::new();
    let mut computers: HashSet<&str> = HashSet::new();

    for line in input.lines() {
        let (id1, id2) = line.split_once('-').unwrap();

        connections.insert((id1, id2));
        connections.insert((id2, id1));

        computers.insert(id1);
        computers.insert(id2);
    }

    let mut computers: Vec<&str> = computers.into_iter().collect();
    computers.sort_unstable();
    let mut computers: VecDeque<&str> = computers.into_iter().collect();

    let mut max_groups: Vec<Vec<&str>> = Vec::new();

    while let Some(seed) = computers.pop_front() {
        let mut curr_group = vec![seed];

        let mut is_maximal = false;
        'outer: while !is_maximal {
            for (idx, computer) in computers.iter().enumerate() {
                if curr_group
                    .iter()
                    .all(|curr_computer| connections.contains(&(curr_computer, computer)))
                {
                    curr_group.push(computers.remove(idx).unwrap());
                    continue 'outer;
                }
            }

            is_maximal = true;
        }

        max_groups.push(curr_group);
    }

    let biggest_connected_network = max_groups.into_iter().max_by_key(Vec::len).unwrap();
    biggest_connected_network.into_iter().join(",")
}
