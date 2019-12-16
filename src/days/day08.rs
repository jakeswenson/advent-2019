fn parse_layers(image: Vec<i32>, width: usize, height: usize) -> Vec<Vec<i32>> {
    image
        .chunks(width * height)
        .map(|c| c.iter().cloned().collect())
        .collect()
}

fn parse(input: &str) -> Vec<i32> {
    input
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c as i32 - '0' as i32)
        .collect()
}

fn merge_layers(base: Vec<i32>, layer: &Vec<i32>) -> Vec<i32> {
    base.iter()
        .zip(layer)
        .map(|(&b, &l)| match l {
            2 => b,
            v => v,
        })
        .collect()
}

fn merge_image(image_layers: Vec<Vec<i32>>) -> Vec<i32> {
    let base = image_layers.first().cloned().unwrap();
    image_layers.iter().skip(1).fold(base, merge_layers)
}

fn part1() -> usize {
    let input = include_str!("resources/day08.txt");
    let layers = parse_layers(parse(input), 25, 6);
    let layer = layers
        .iter()
        .min_by_key(|l| l.iter().filter(|&&i| i == 0).count())
        .unwrap();
    let (ones, twos): (Vec<i32>, Vec<i32>) = layer
        .iter()
        .filter(|&&i| i == 1 || i == 2)
        .partition(|&&i| i % 2 == 0);
    ones.len() * twos.len()
}

fn part2() {
    let input = include_str!("resources/day08.txt");
    let mut layers = parse_layers(parse(input), 25, 6);
    layers.reverse();
    let image = merge_image(layers);
    for row in image.chunks(25) {
        for &v in row {
            print!(
                "{}",
                match v {
                    0 => ' ',
                    1 => '*',
                    _ => unreachable!(),
                }
            );
        }
        println!();
    }
}

pub fn solve() {
    println!("Digits Between: {}", part1());
    part2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_rules() {
        let layers = parse_layers(parse("123456789012"), 3, 2);
        assert_eq!(layers, vec![vec![1, 2, 3, 4, 5, 6], vec![7, 8, 9, 0, 1, 2]])
    }

    #[test]
    fn test_part2_rules() {}
}
