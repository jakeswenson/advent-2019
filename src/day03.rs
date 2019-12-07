use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Debug, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        return Point { x, y };
    }

    fn distance_from_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
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

pub type WirePath = Vec<PathSegment>;

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

fn all_points(path: &WirePath) -> HashSet<Point> {
    path.iter()
        .flat_map(|seg| seg.range.start.step_to(&seg.range.end).expect("Cant step"))
        .map(|p| p.point)
        .filter(|&p| p != Point::default())
        .collect()
}

fn all_intersections(path1: WirePath, path2: WirePath) -> Vec<Point> {
    let path1_points = all_points(&path1);
    let path2_points = all_points(&path2);

    path1_points.intersection(&path2_points).cloned().collect()
}

fn closest_to_origin(mut points: Vec<Point>) -> Option<Point> {
    points.sort_by_key(|p| p.distance_from_origin());
    points.first().cloned()
}

fn find_closest(input: &str) -> Option<Point> {
    let (first, second) = parse_paths(input);
    closest_to_origin(all_intersections(first, second))
}

pub fn solve() {
    let paths_text = include_str!("resources/day03.txt");
    let result = find_closest(paths_text).expect("No closest Point");
    println!("Distance: {}", result.distance_from_origin());
    let (first, second) = parse_paths(paths_text);
    // to_svg::make_svg(first, second);
}

mod to_svg {
    use std::collections::HashSet;

    use crate::day03::find_closest;

    use super::{PathSegment, Point, WirePath};

    struct Bounds {
        min_x: i32,
        min_y: i32,
        max_x: i32,
        max_y: i32,
    }

    impl Bounds {
        fn expand(&self, by: i32) -> Bounds {
            Bounds {
                max_y: by + self.max_y,
                max_x: by + self.max_x,
                min_y: self.min_y - by,
                min_x: self.min_x - by,
            }
        }
    }

    fn get_bounds(paths: &Vec<WirePath>) -> Bounds {
        let points: HashSet<Point> = paths
            .iter()
            .flat_map(|wire_path| super::all_points(wire_path))
            .collect();

        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();

        Bounds {
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }

    pub fn make_svg(first: WirePath, second: WirePath) {
        use svg::node::element::path::Data;
        use svg::node::element::{Circle, Path, Text};
        use svg::node::Node;
        use svg::Document;

        let bounds = get_bounds(&vec![first.clone(), second.clone()]);
        let view_port = bounds.expand(2);

        let document = Document::new().set(
            "viewBox",
            (
                view_port.min_x,
                view_port.min_y,
                view_port.max_x,
                view_port.max_y,
            ),
        );

        let document = (view_port.min_x..=view_port.max_x).fold(document, |document, x| {
            (view_port.min_y..=view_port.max_y).fold(document, |document, y| {
                if x % 50 == 0 && y % 50 == 0 {
                    document.add(
                        Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", 0.25)
                            .set("fill", "black"),
                    )
                } else {
                    document
                }
            })
        });

        let data_first: Data = first
            .iter()
            .map(|seg| seg.range.end.point)
            .fold(Data::new().move_to((0, 0)), |data, point| {
                data.line_to((point.x, point.y))
            });

        let first_path = Path::new()
            .set("stroke", "green")
            .set("stroke-width", 1)
            .set("fill", "none")
            .set("d", data_first);

        let data_second: Data = second
            .iter()
            .map(|seg| seg.range.end.point)
            .fold(Data::new().move_to((0, 0)), |data, point| {
                data.line_to((point.x, point.y))
            });

        let second_path = Path::new()
            .set("stroke", "blue")
            .set("stroke-width", 1)
            .set("fill", "none")
            .set("d", data_second);

        let document = document.add(first_path).add(second_path);

        let document = super::all_intersections(first.clone(), second.clone())
            .iter()
            .fold(document, |document, p| {
                document
                    .add(
                        Circle::new()
                            .set("cx", p.x)
                            .set("cy", p.y)
                            .set("r", 0.4)
                            .set("fill", "purple"),
                    )
                    .add({
                        let mut text = Text::new().set("x", p.x).set("y", p.y);
                        text.append(svg::node::Text::new(format!("{},{}", p.x, p.y)));
                        text
                    })
            });

        let closest = super::closest_to_origin(super::all_intersections(first, second)).unwrap();

        let document = document.add(
            Circle::new()
                .set("cx", closest.x)
                .set("cy", closest.y)
                .set("r", 0.5)
                .set("fill", "red"),
        );

        svg::save("path.svg", &document).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demo() {
        let first_path = parse_wire_path("R8,U5,L5,D3");
        let second_path = parse_wire_path("U7,R6,D4,L4");

        to_svg::make_svg(first_path.clone(), second_path.clone());
        println!("{:?}", all_points(&first_path));
        let intersections = all_intersections(first_path, second_path);
        println!("{:?}", intersections);
        assert_eq!(closest_to_origin(intersections), Some(Point::new(3, 3)))
    }

    #[test]
    fn test_examples() {
        let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

        let (first, second) = parse_paths(input);
        to_svg::make_svg(first, second);

        assert_eq!(
            find_closest(input).map(|p| p.distance_from_origin()),
            Some(159)
        );

        let input = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

        assert_eq!(
            find_closest(input).map(|p| p.distance_from_origin()),
            Some(135)
        )
    }
}
