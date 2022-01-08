#[warn(unused_variables)]
#[warn(unused_mut)]

const PLAYER: char = 'X';
const COMPUTER: char = 'O';
const TIE: char = ' ';

pub fn free_spaces(table: &mut [[char; 3]; 3]) -> i8 {
    let free_space: i8 = 9;

    for row in 1..3 {
        for col in 1..3 {
            println!("{} {}", row, col);
        }
    }

    return free_space;
}

pub fn print_table(table: &mut [[char; 3]; 3]) {
    println!("-------------
| {} | {} | {} |
-------------
| {} | {} | {} | 
-------------
| {} | {} | {} |
-------------", 
    table[0][0],
    table[0][1],
    table[0][2],
    table[1][0],
    table[1][1],
    table[1][2],
    table[2][0],
    table[2][1],
    table[2][2],
    )
}

pub fn main() {
    let mut winner: char = ' ';
    let mut table = [
        [' ', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', ' '],
    ];

    print_table(&mut table);

    while winner == ' ' && free_spaces(&mut table) != 0 {

    }
}
