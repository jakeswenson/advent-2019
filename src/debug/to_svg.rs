#![allow(dead_code)]
use std::collections::HashSet;

use crate::day03::{all_intersections, all_points, distances, find_closest};

use crate::day03::{Point, WirePath};

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
        .flat_map(|wire_path| all_points(wire_path))
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
        .map(|seg| seg.end())
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
        .map(|seg| seg.end())
        .fold(Data::new().move_to((0, 0)), |data, point| {
            data.line_to((point.x, point.y))
        });

    let second_path = Path::new()
        .set("stroke", "blue")
        .set("stroke-width", 1)
        .set("fill", "none")
        .set("d", data_second);

    let document = document.add(first_path).add(second_path);

    let document = all_intersections(&first, &second)
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

    let closest = find_closest((&first, &second), distances::manhattan).unwrap();

    let document = document.add(
        Circle::new()
            .set("cx", closest.x)
            .set("cy", closest.y)
            .set("r", 0.5)
            .set("fill", "red"),
    );

    svg::save("path.svg", &document).unwrap();
}
