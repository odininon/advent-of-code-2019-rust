use crate::utilities;
use std::io::{BufReader, Error, BufRead};
use std::fs::File;

#[cfg(test)]
mod tests {
    use crate::day1::{calculate_fuel, calculate_compound_fuel};

    #[test]
    fn known_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn known_compound_calculate_fuel() {
        assert_eq!(calculate_compound_fuel(12), 2);
        assert_eq!(calculate_compound_fuel(1969), 966);
        assert_eq!(calculate_compound_fuel(100756), 50346);
    }
}

pub fn solve(input_file: &str) -> Result<(), Error> {
    println!("-------Day 1-------");
    let masses = get_puzzle_input(utilities::read_file(input_file)?)?;
    let fuel_weight = calculate_fuel_for_masses(&masses, &calculate_fuel);
    println!("Needed fuel:\t\t{}", fuel_weight);
    let compound_fuel_weight = calculate_fuel_for_masses(&masses, &calculate_compound_fuel);
    println!("Total needed fuel:\t{}", compound_fuel_weight);

    Ok(())
}

fn calculate_fuel_for_masses(masses: &Vec<i32>, f: &dyn Fn(i32) -> i32) -> i32 {
    let mut fuel_weight = 0;
    for mass in masses {
        fuel_weight += f(*mass);
    }
    fuel_weight
}

fn get_puzzle_input(input_stream: BufReader<File>) -> Result<Vec<i32>, Error> {
    let mut masses = vec![];
    for line in input_stream.lines() {
        masses.push(line?.parse().unwrap());
    }
    Ok(masses)
}

fn calculate_compound_fuel(mass: i32) -> i32 {
    fn work(working_mass: i32, accumulator: i32) -> i32 {
        let fuel_weight = calculate_fuel(working_mass);
        if fuel_weight <= 0 {
            return accumulator;
        } else {
            return work(fuel_weight, accumulator + fuel_weight);
        }
    }
    work(mass, 0)
}

fn calculate_fuel(mass: i32) -> i32 {
    return mass / 3 - 2;
}