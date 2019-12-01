use std::str::FromStr;

fn module_masses() -> impl Iterator<Item = i64> {
    let modules_text = include_str!("resources/day01.txt");

    modules_text.lines().map(i64::from_str).map(|r| r.unwrap())
}

fn fuel_required(mass: i64) -> i64 {
    mass / 3 - 2
}

fn part_one() -> i64 {
    let sum = module_masses().map(fuel_required).sum();

    println!("Fuel for modules: {}", sum);
    return sum;
}

fn fuel(mass: i64) -> i64 {
    let fuel = fuel_required(mass);
    let mut total_fuel = fuel;
    let mut fuel_fuel = fuel_required(fuel);
    while fuel_fuel > 0 {
        total_fuel += fuel_fuel;
        fuel_fuel = fuel_required(fuel_fuel)
    }
    return total_fuel;
}

fn part_two() -> i64 {
    let sum = module_masses().map(fuel).sum();

    println!("Fuel for fuel + modules: {}", sum);
    return sum;
}

pub fn solve() {
    let total_mass: i64 = module_masses().sum();
    println!("Total Mass   : {}", total_mass);

    part_one();
    part_two();
}
