fn main() {
    // load input file
    let file = include_str!("../input");

    let mut elves: Vec<Vec<i32>> = Vec::new();
    // parse each line as a calorie count and sum into groups of elves
    let mut current_elf = 0;
    elves.push(Vec::new());
    for line in file.lines() {
        match line {
            "" => {
                current_elf += 1;
                elves.push(Vec::new());
            }
            _ => {
                let calorie = line.parse::<i32>().unwrap();
                elves[current_elf].push(calorie);
            }
        }
    }

    let mut calorie_count: Vec<i32> = elves.iter().map(|c| c.iter().sum::<i32>()).collect();

    print!("{:?}", calorie_count.iter().max());

    // sort to find the top 3
    calorie_count.sort_by(|a, b| b.cmp(a));

    let _ = calorie_count.split_off(3);

    dbg!(&calorie_count);
    println!("{:?}", calorie_count.iter().sum::<i32>());
}
