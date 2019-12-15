use std::collections::hash_map::OccupiedEntry;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
struct Edge {
  center: String,
  moon: String,
}

fn parse_orbits(orbits_text: &str) -> Vec<Edge> {
  orbits_text
    .lines()
    .map(|l| {
      let mut split = l.splitn(2, ')');
      Edge {
        center: split.next().unwrap().to_string(),
        moon: split.next().unwrap().to_string(),
      }
    })
    .collect()
}

fn count_orbits(orbits: &[Edge]) -> usize {
  let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

  for edge in orbits {
    graph
      .entry(&edge.center)
      .and_modify(|v| v.push(&edge.moon))
      .or_insert(vec![&edge.moon]);
  }

  let mut queue = VecDeque::from(vec![("COM", 0)]);
  let mut indirect_orbits = 0;

  while let Some((center, count)) = queue.pop_front() {
    for &moon in graph.get(center).unwrap_or(&Vec::new()) {
      queue.push_back((moon, count + 1));
      indirect_orbits += count;
    }
  }

  let direct_orbits_count = orbits.len();

  indirect_orbits + direct_orbits_count
}

fn transfer_count(orbits: &[Edge], to: &str, from: &str) -> usize {
  let mut parents: HashMap<&str, &str> = HashMap::with_capacity(orbits.len());

  for orbit in orbits {
    parents.insert(&orbit.moon, &orbit.center);
  }

  let mut queue = VecDeque::from(vec![(to, 0), (from, 0)]);

  let mut path_distance: HashMap<&str, usize> = HashMap::with_capacity(orbits.len());

  while let Some((node, current_distance)) = queue.pop_front() {
    let parent = parents.get(node).unwrap();
    //    println!("{}->{}({})", node, parent, current_distance);

    if let Some(&distance) = path_distance.get(parent) {
      return current_distance + distance;
    }

    path_distance.insert(parent, current_distance);

    let distance = current_distance + 1;
    queue.push_back((parent, distance));
  }

  unreachable!("No path between Nodes")
}

fn part1(orbits: &[Edge]) {
  let all_orbits = count_orbits(orbits);
  println!("Orbits: {}", all_orbits);
}

fn part2(orbits: &[Edge]) {
  let distance = transfer_count(orbits, "SAN", "YOU");

  println!("Distance: {}", distance);
}

pub fn solve() {
  let orbits = parse_orbits(include_str!("resources/day06.txt"));
  part1(&orbits);
  part2(&orbits);
}

#[cfg(test)]
mod tests {
  use crate::day06::{count_orbits, parse_orbits, transfer_count};

  #[test]
  fn test_part1_rules() {
    let orbits = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    let orbits = parse_orbits(orbits);
    assert_eq!(count_orbits(&orbits), 42)
  }

  #[test]
  fn test_part2_rules() {
    let orbits = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    let orbits = parse_orbits(orbits);
    let distance = transfer_count(&orbits, "YOU", "SAN");
    println!("{:?}", distance);
    assert_eq!(distance, 4);
  }
}
