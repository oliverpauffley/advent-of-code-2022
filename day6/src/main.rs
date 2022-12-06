fn main() {
    let input = include_str!("input.txt");

    // part 1
    for window in input
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .enumerate()
    {
        if is_unique(window.1) {
            println!("found packet at: {}", window.0 + window.1.len());
            break;
        }
    }

    for window in input
        .lines()
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .enumerate()
    {
        if is_unique(window.1) {
            println!("found message at: {}", window.0 + window.1.len());
            break;
        }
    }
}

fn is_unique(set: &[char]) -> bool {
    (1..set.len()).all(|i| !set[i..].contains(&set[i - 1]))
}
