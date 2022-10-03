#[derive(Debug, PartialEq)]
enum DepthChange {
    None,
    Increased,
    Decreased,
}

fn diff_to_depth(left: i32, right: i32) -> DepthChange {
    let diff = left - right;
    if diff == 0 {
        DepthChange::None
    } else if diff > 0 {
        DepthChange::Decreased
    } else {
        DepthChange::Increased
    }
}

fn sonar_sweep(diffs: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut results:Vec<DepthChange> = vec![];
    while i < diffs.len() -1 {
        let current = diffs[i];
        let next = diffs[i + 1];
        let result = diff_to_depth(current, next);
        println!("{} to {} is a change of: {:?}", current, next, result);
        results.push(result);
        i += 1;
    }
    let amount_increased: usize = results.iter().filter(|&n| *n == DepthChange::Increased).count();
    println!("{} change(s) were an increase!", amount_increased);
    return amount_increased as i32;
}

fn main() {
    // The numbers as provided in the advent-of-code day 1 description
    sonar_sweep(vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_numbers_inc() {
        assert_eq!(sonar_sweep(vec![1, 2, 3]), 2);
    }

    #[test]
    fn test_3_numbers_dec() {
        assert_eq!(sonar_sweep(vec![3, 2, 1]), 0);
    }
}