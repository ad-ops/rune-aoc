// Comments added for every line changed from scripts/aoc_2022_1_1.rn

// pub async fn main() {
fn main() -> anyhow::Result<()> {
    // let file = fs::read_to_string("data/aoc_2022_1_1.txt").await?;
    let file = std::fs::read_to_string("data/aoc_2022_1_1.txt")?;

    // let max_calories = 0;
    let mut max_calories = 0;
    // let current_calories = 0;
    let mut current_calories = 0;
    for line in file.split("\n") {
        if line.len() == 0 {
            if max_calories < current_calories {
                max_calories = current_calories;
            }
            current_calories = 0;
            continue;
        }
        // let calorie = std::i64::parse(line)?;
        let calorie = line.parse::<i64>()?;
        current_calories += calorie;
    }

    println!("max_calories: {}", max_calories);
    // max_calories
    Ok(())
}