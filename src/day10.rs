use std::collections::{HashMap, HashSet};

use crate::day03::all_points;
use crate::point::{Point, Slope};

struct Map {
  objects: Vec<Vec<bool>>,
}

impl Map {
  fn from(map_string: &str) -> Map {
    Map {
      objects: map_string
        .trim()
        .lines()
        .map(|l| {
          l.trim()
            .chars()
            .map(|c| match c {
              '.' => false,
              '#' => true,
              _ => unreachable!(),
            })
            .collect()
        })
        .collect(),
    }
  }

  fn object_at(&self, point: &Point) -> bool {
    if point.x as usize > self.objects.len() {
      return false;
    }
    return self.objects[point.y as usize][point.x as usize];
  }

  fn all_points(&self) -> Vec<Point> {
    let x_max = self.objects.get(0).map(|row| row.len()).unwrap_or(0);
    let y_max = self.objects.len();

    (0..y_max)
      .flat_map(|y| (0..x_max).map(move |x| Point::of(x as i32, y as i32)))
      .collect()
  }

  fn objects(&self) -> HashSet<Point> {
    self
      .all_points()
      .iter()
      .cloned()
      .filter(|p| self.object_at(p))
      .clone()
      .collect()
  }
}

struct BestPoint {
  point: Point,
  count: i32,
  results: HashMap<Point, i32>,
}

fn find_best(map: &Map) -> BestPoint {
  let objects = map.objects();

  let mut hash: HashMap<Point, i32> = HashMap::new();

  objects.iter().copied().for_each(|starting: Point| {
    objects.iter().copied().for_each(|other| {
      let mut path = starting.walk_to(&other);
      while let Some(point) = path.step() {
        if map.object_at(&point) && point != starting {
          if point == other {
            hash.entry(starting).and_modify(|v| *v += 1).or_insert(1);
          }
          break;
        }
      }
    });
  });

  let mut v: Vec<(i32, Point)> = hash.iter().map(|(&k, &v)| (v, k)).collect();
  v.sort_by_key(|(count, point)| -count);

  let (count, point) = v.first().unwrap().clone();
  BestPoint {
    count: count.clone(),
    point: point.clone(),
    results: hash,
  }
}

fn part1() {
  let map = Map::from(include_str!("resources/day10.txt"));
  let best_point = find_best(&map);
  println!("{} - #{}", best_point.point, best_point.count);
}

fn laser(center: &Point, map: &Map) -> Vec<Point> {
  let mut result: Vec<Slope> = map
    .objects()
    .iter()
    .filter(|&p| p != center)
    .map(|f| center.slope_to(f))
    .collect::<HashSet<Slope>>()
    .iter()
    .cloned()
    .collect();

  result.sort_by(|a, b| {
    let a = a.as_vector();
    let b = b.as_vector();
    let a_mag = a.magnitude();
    let b_mag = b.magnitude();
    let a = (a.angle_with_0_down() * b_mag,);
    let b = (b.angle_with_0_down(), b.magnitude());

    a.partial_cmp(&b).unwrap()
  });

  return result.iter().map(|slope| center.add(slope)).collect();
}

fn part2() {}

pub fn solve() {
  part1();
  part2();
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::point::{Slope, Vector};
  use std::collections::VecDeque;

  #[test]
  fn test_part1_small() {
    let map = Map::from(
      "\
#.
.#",
    );

    assert!(map.object_at(&Point::of(0, 0)));
    assert!(map.object_at(&Point::of(1, 1)));

    let BestPoint {
      count: _,
      results,
      point,
    } = find_best(&map);
    assert_eq!(point, Point::of(0, 0));

    println!("{:?}", results);
  }

  #[test]
  fn test_part1_rules() {
    let map = Map::from(
      "\
.#..#
.....
#####
....#
...##",
    );

    let BestPoint {
      count,
      results,
      point,
    } = find_best(&map);
    println!("Count {}, point {}", count, point);
    println!("{:?}", results.get(&Point::of(3, 4)));
    println!("{:?}", results);

    assert_eq!(point, Point::of(3, 4));
  }

  #[test]
  fn test_part2_rules() {
    let map = "
.#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    let map = Map::from(map);
    let center = Point::of(11, 13);

    println!("{:?}", result);
    let mut iter = result.iter().cloned();

    let point = center.add(iter.next().unwrap());
    assert_eq!(point, Point::of(11, 12));

    let point = center.add(iter.next().unwrap());
    assert_eq!(point, Point::of(12, 1));
  }

  #[test]
  fn test_part2_simple() {
    let map = "
.#.
###
";
    let map = Map::from(map);
    let center = Point::of(1, 1);

    let points = laser(&center, &map);
    println!("{:?}", points);
    let mut iter = result.iter().cloned();

    let point = center.add(iter.next().unwrap());
    assert_eq!(point, Point::of(11, 12));

    let point = center.add(iter.next().unwrap());
    assert_eq!(point, Point::of(12, 1));
  }
}
