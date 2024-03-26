use crate::game::Grid;

pub fn print_grid(grid: &Grid) {
    // Print header
    print!("   ");
    for x in 0..grid.size {
        print!("{:>2}", index_to_char(x));
    }
    println!();

    for y in 0..grid.size {
        print!("{:>2} ", y + 1);
        for x in 0..grid.size {
            let square = &grid.squares[x][y];
            match (square.ship, square.hit) {
                (Some(_ship), false) => print!(" O"),
                (Some(_ship), true) => print!(" X"),
                (None, false) => print!(" ."),
                (None, true) => print!(" x"),
            }
        }
        println!();
    }
}

fn index_to_char(i: usize) -> char {
    (65u8 + i as u8) as char
}
