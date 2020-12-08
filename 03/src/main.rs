use std::fs;

fn find_answer(rows: &Vec<Vec<char>>, y_inc: usize, x_inc: usize) -> u32 {
    let height = rows.len();
    let width = rows[0].len();

    let mut count: u32 = 0;

    let mut y = y_inc;
    let mut x = x_inc;
    while y < height {
        if rows[y][x] == '#' {
            count += 1;
        }

        y += y_inc;
        x += x_inc;
        if x >= width {
            x = x % width;
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let input: String = String::from_utf8_lossy(&fs::read("./input.txt")?).parse()?;

    let rows: Vec<Vec<char>> = input
        .lines()
        .map(|line: &str|  
            line.chars().collect()
        )
        .collect();
    
    println!("The answer to part 1 is {}", find_answer(&rows, 1, 3));

    let slopes= [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let total: u32 = slopes
        .iter()
        .map(|(x_inc, y_inc)| -> u32 {
            find_answer(&rows, *y_inc, *x_inc)
        })
        .fold(1, |a, b| a * b);
    
    println!("The answer to part 2 is {}", total);

    Ok(())
}