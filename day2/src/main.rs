#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Forward,
}

fn string_to_direction(direction: &str) -> Direction {
    match direction {
        "forward" => Direction::Forward,
        "up" => Direction::Up,
        "down" => Direction::Down,
        _ => panic!("Invalid direction provided: {}", direction)
    }
}

fn direction_finder(input: Vec<&str>) -> i32 {
    let mut depth = 0;
    let mut horz_distance = 0;
    for line in input {
        let split: Vec<&str> = line.split(" ").collect();
        println!("{:?}", split);
        let direction = string_to_direction(split[0]);
        let distance: i32 = split[1].to_owned().parse().unwrap();
        println!("{:?}, {:?}", direction, distance);
        match direction {
            Direction::Forward => horz_distance += distance,
            Direction::Up => depth -= distance,
            Direction::Down => depth += distance,
        }
    }
    println!("Horizontal distance: {:?}", horz_distance);
    println!("Vertical distance: {:?}", depth);
    let result = horz_distance * depth;
    println!("Total distance: {:?}", result);
    return result;
}

fn main() {
    let _ = direction_finder(vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2"
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(direction_finder(vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2"
        ]), 150);
    }

    #[test]
    #[should_panic(expected = "Invalid direction provided: backward")]
    fn test_invalid_input() {
        direction_finder(vec!["forward 5", "backward 7"]);
    }
}