use std::io;

#[warn(unused_variables)]
#[warn(unused_mut)]

const PLAYER: char = 'X';
const COMPUTER: char = 'O';
const TIE: char = ' ';

fn get_y() -> i32 {
    let mut y_str = String::new();

    println!("Enter column from 1 to 3: ");
    io::stdin()
        .read_line(&mut y_str)
        .expect("Failed to read line!");

    let mut y: i32 = y_str.trim().parse()
        .expect("Failed to parse number!");
        
    if y > 3 || y < 1 {
        println!("Number is bigger than 3 or smaller than 1!");
        y = 0;
        get_y();
    }

    return y;
}

fn get_x() -> i32 {
    let mut x_str = String::new();

    println!("Enter row from 1 to 3: ");
    io::stdin()
        .read_line(&mut x_str)
        .expect("Failed to read line!");

    let mut x: i32 = x_str.trim().parse()
        .expect("Failed to parse number!");
    
    if x > 3 || x < 1 {
        println!("Number is bigger than 3 or smaller than 1!");
        x = 0;
        get_x();
    }
     
    return x;
}

pub fn player_move(table: &mut [[char; 3]; 3]) -> &mut [[char; 3]; 3] {
    let x: i32 = get_x();
    let y: i32 = get_y();

    let index_x: usize = (x - 1) as usize;
    let index_y: usize = (y - 1) as usize;

    if table[index_y][index_x] != ' ' {
        println!("This place is already taken by {}!", table[index_y][index_x]);
        player_move(table);
    }

    table[index_y][index_x] = PLAYER;

    return &mut table;
}

pub fn free_spaces(table: &mut [[char; 3]; 3]) -> i8 {
    let mut free_space: i8 = 9;

    for rows in 0..table.len() {
        for cols in 0..table[rows].len() {
            if table[rows][cols] != ' ' {
                free_space -= 1;
            }
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
        ['X', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', ' '],
    ];

    while winner == ' ' && free_spaces(&mut table) != 0 {
        print_table(&mut table);

        table = player_move(&mut table);
    }
}
