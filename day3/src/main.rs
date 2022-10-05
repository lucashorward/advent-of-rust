fn split_string(str: &str) -> Vec<String> {
    str.split("")
        .filter(|n: &&str| *n != "")
        .map(|n| String::from(n))
        .collect()
}

struct MostAndLeast {
    most_used: String,
    least_used: String,
}

// I was lazy and assumed the only options are 0 or 1
fn most_least_used(input: String) -> MostAndLeast {
    let zero_count = input.matches("0").count();
    let one_count = input.matches("1").count();
    if zero_count > one_count {
        MostAndLeast {
            most_used: String::from("0"),
            least_used: String::from("1"),
        }
    } else {
        MostAndLeast {
            most_used: String::from("1"),
            least_used: String::from("0"),
        }
    }
}

// Yes, this is overengineered
fn get_power_consumption(input: Vec<&str>) -> i32 {
    // I rebuild the binary strings vertically, so I have a string corresponding to the first number of each original string
    let mut resliced: Vec<String> = split_string(input[0]);
    let mut counter = 1;
    while counter < input.len() {
        let slice: Vec<String> = split_string(input[counter]);
        let mut i = 0;
        while i < slice.len() {
            resliced[i].push_str(slice[i].as_str());
            i += 1;
        }
        counter += 1;
    }
    // From each vertical slice, we must now determine the most and least used character
    let mut binary_gamma = String::from("");
    let mut binary_epsilon = String::from("");
    for a in resliced {
        let res = most_least_used(a);
        binary_gamma.push_str(&res.most_used);
        binary_epsilon.push_str(&res.least_used);
    }
    let decimal_gamma = i32::from_str_radix(binary_gamma.as_str(), 2).unwrap();
    let decimal_epsilon = i32::from_str_radix(binary_epsilon.as_str(), 2).unwrap();
    let result = decimal_gamma * decimal_epsilon;
    println!("Full result is: {}", result);
    return result;
}

fn main() {
    let _ = get_power_consumption(vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal() {
        assert_eq!(
            get_power_consumption(vec![
                "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
                "11001", "00010", "01010",
            ]),
            198
        );
    }
}
