use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Point { x, y };
    }

    fn left(&self, amount: i32) -> Self {
        Point {
            x: self.x - amount,
            y: self.y,
        }
    }
    fn right(&self, amount: i32) -> Self {
        Point {
            x: self.x + amount,
            y: self.y,
        }
    }
    fn up(&self, amount: i32) -> Self {
        Point {
            x: self.x,
            y: self.y + amount,
        }
    }
    fn down(&self, amount: i32) -> Self {
        Point {
            x: self.x,
            y: self.y - amount,
        }
    }
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct DirectionalPoint {
    point: Point,
    direction: Direction,
}

impl DirectionalPoint {
    fn step(&self, n: i32) -> Self {
        match self.direction {
            Direction::Up => DirectionalPoint {
                point: self.point.up(n),
                direction: self.direction,
            },
            Direction::Down => DirectionalPoint {
                point: self.point.down(n),
                direction: self.direction,
            },
            Direction::Left => DirectionalPoint {
                point: self.point.left(n),
                direction: self.direction,
            },
            Direction::Right => DirectionalPoint {
                point: self.point.right(n),
                direction: self.direction,
            },
        }
    }

    fn step_to(&self, end: &Self) -> Option<Vec<Self>> {
        let steps_between: Option<usize> = match self.direction {
            Direction::Down | Direction::Up => {
                if self.point.x == end.point.x {
                    Some((end.point.y - self.point.y).abs() as usize)
                } else {
                    None
                }
            }
            Direction::Left | Direction::Right => {
                if self.point.y == end.point.y {
                    Some((end.point.x - self.point.x).abs() as usize)
                } else {
                    None
                }
            }
        };

        steps_between.map(|steps| (0..steps).map(|step| self.step(step as i32)).collect())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PathSegment {
    range: Range<DirectionalPoint>,
}

impl PathSegment {
    pub fn points(&self) -> Vec<DirectionalPoint> {
        self.range
            .start
            .step_to(&self.range.end)
            .expect("Cant step")
    }

    pub fn end(&self) -> Point {
        self.range.end.point.clone()
    }
}

pub type WirePath = Vec<PathSegment>;

pub mod distances {
    use crate::day03::{Point, WirePath};
    use std::collections::HashMap;

    pub fn manhattan(point: &Point) -> i32 {
        point.x.abs() + point.y.abs()
    }

    pub fn wire_paths(first: &WirePath, second: &WirePath) -> impl Fn(&Point) -> i32 {
        fn points(path: &WirePath) -> Vec<Point> {
            path.iter()
                .flat_map(|seg| seg.points())
                .map(|p| p.point)
                .collect()
        }

        let mut map: HashMap<Point, i32> = HashMap::new();
        points(first)
            .iter()
            .enumerate()
            .chain(points(second).iter().enumerate())
            .for_each(|(idx, &point)| {
                map.entry(point)
                    .and_modify(|e| *e += idx as i32)
                    .or_insert(idx as i32);
            });

        move |point| map.get(point).expect("Can't find key").clone()
    }
}

fn parse_wire_path(input: &str) -> WirePath {
    struct State {
        last_point: Point,
        segments: WirePath,
    }
    impl Default for State {
        fn default() -> Self {
            State {
                last_point: Point::default(),
                segments: vec![],
            }
        }
    }

    input
        .split(',')
        .fold(State::default(), |mut state: State, instruction: &str| {
            let (direction, distance) = instruction.split_at(1);
            let distance = i32::from_str(distance).expect("Can't Parse Distance");
            let last_point = state.last_point;
            let new_point = match direction {
                "U" => DirectionalPoint {
                    direction: Direction::Up,
                    point: Point {
                        y: last_point.y + distance,
                        ..last_point
                    },
                },
                "D" => DirectionalPoint {
                    direction: Direction::Down,
                    point: Point {
                        y: last_point.y - distance,
                        ..last_point
                    },
                },
                "R" => DirectionalPoint {
                    direction: Direction::Right,
                    point: Point {
                        x: last_point.x + distance,
                        ..last_point
                    },
                },
                "L" => DirectionalPoint {
                    direction: Direction::Left,
                    point: Point {
                        x: last_point.x - distance,
                        ..last_point
                    },
                },
                direction => panic!("Unknown direction {}", direction),
            };
            state.segments.push(PathSegment {
                range: DirectionalPoint {
                    point: last_point,
                    direction: new_point.direction,
                }..new_point,
            });

            return State {
                last_point: new_point.point,
                ..state
            };
        })
        .segments
}

fn parse_paths(input: &str) -> (WirePath, WirePath) {
    let lines: Vec<WirePath> = input.lines().map(parse_wire_path).collect();
    assert_eq!(lines.len(), 2);
    (lines[0].clone(), lines[1].clone())
}

pub fn all_points(path: &WirePath) -> HashSet<Point> {
    path.iter()
        .flat_map(|seg| seg.range.start.step_to(&seg.range.end).expect("Cant step"))
        .map(|p| p.point)
        .filter(|&p| p != Point::default())
        .collect()
}

pub fn all_intersections(path1: &WirePath, path2: &WirePath) -> Vec<Point> {
    let path1_points = all_points(path1);
    let path2_points = all_points(path2);

    path1_points.intersection(&path2_points).cloned().collect()
}

pub fn find_closest(
    (first, second): (&WirePath, &WirePath),
    distance: impl Fn(&Point) -> i32,
) -> Option<Point> {
    let mut points = all_intersections(&first, &second);

    points.sort_by_key(distance);
    points.first().cloned()
}

fn part1(input: &str) {
    let (first, second) = parse_paths(input);

    let result = find_closest((&first, &second), distances::manhattan).expect("No closest Point");
    println!("Distance: {}", distances::manhattan(&result));
}

fn part2(input: &str) {
    let (first, second) = parse_paths(input);
    let distance_to = distances::wire_paths(&first, &second);

    let result = find_closest((&first, &second), distance_to).expect("No closest Point");

    let distance_to = distances::wire_paths(&first, &second);

    println!("Distance: {}", distance_to(&result));
}

pub fn solve() {
    let paths_text = include_str!("resources/day03.txt");
    part1(paths_text);
    part2(paths_text)
    // let (first, second) = parse_paths(paths_text);
    // to_svg::make_svg(first, second);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::to_svg;

    #[test]
    fn test_demo() {
        let first_path = parse_wire_path("R8,U5,L5,D3");
        let second_path = parse_wire_path("U7,R6,D4,L4");

        to_svg::make_svg(first_path.clone(), second_path.clone());
        println!("{:?}", all_points(&first_path));
        let intersections = all_intersections(&first_path, &second_path);
        println!("{:?}", intersections);
        assert_eq!(
            super::find_closest((&first_path, &second_path), super::distances::manhattan),
            Some(Point::new(3, 3))
        )
    }

    #[test]
    fn test_demo_part2() {
        let first_path = parse_wire_path("R8,U5,L5,D3");
        let second_path = parse_wire_path("U7,R6,D4,L4");

        println!("{:?}", all_points(&first_path));
        let intersections = all_intersections(&first_path, &second_path);
        let distance_to = distances::wire_paths(&first_path, &second_path);
        let closest_point = find_closest((&first_path, &second_path), distance_to).unwrap();
        assert_eq!(closest_point, Point::new(6, 5));

        let distance_to = distances::wire_paths(&first_path, &second_path);
        assert_eq!(distance_to(&closest_point), 30)
    }

    #[test]
    fn test_examples() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

        let (first, second) = parse_paths(input);

        assert_eq!(
            distances::manhattan(&find_closest((&first, &second), distances::manhattan).unwrap()),
            159
        );
        to_svg::make_svg(first, second);

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
        let (first, second) = parse_paths(input);

        assert_eq!(
            distances::manhattan(&find_closest((&first, &second), distances::manhattan).unwrap()),
            135
        )
    }
}
