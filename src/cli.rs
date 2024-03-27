use crate::game::Grid;

pub fn print_grid(grid: &Grid) {
    // Print header
    print!("   ");
    for x in 0..grid.size {
        print!("{:>2}", index_to_char(x));
    }
    println!();

    for (y, row) in grid.to_string().lines().enumerate() {
        print!("{:>2} ", y + 1);
        for ch in row.chars() {
            print!(" {}", ch);
        }
        println!();
    }
}

fn index_to_char(i: usize) -> char {
    (65u8 + i as u8) as char
}
