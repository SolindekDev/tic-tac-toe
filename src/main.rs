use std::io;
use std::process;
use rand::prelude::*;

#[warn(unused_variables)]
#[warn(unused_mut)]

const PLAYER: char = 'X';
const COMPUTER: char = 'O';
const TIE: char = 'T';

fn get_x() -> i32 {
    let mut x_str = String::new();

    println!("Enter column from 1 to 3: ");
    io::stdin()
        .read_line(&mut x_str)
        .expect("Failed to read line!");

    match x_str.trim().parse::<i32>() {
        Ok(x) => {
            if x > 3 || x < 1 {
                println!("Number is bigger than 3 or smaller than 1!");
                get_x();
            }
        
            return x;
        },
        Err(e) => {
            println!("You must to provide a valid number");
            get_y()
        }
    }
}

fn get_y() -> i32 {
    let mut y_str = String::new();

    println!("Enter row from 1 to 3: ");
    io::stdin()
        .read_line(&mut y_str)
        .expect("Failed to read line!");

    match y_str.trim().parse::<i32>() {
        Ok(y) => {
            if y > 3 || y < 1 {
                println!("Number is bigger than 3 or smaller than 1!");
                get_y();
            }
        
            return y;
        },
        Err(e) => {
            println!("You must to provide a valid number");
            get_y()
        }
    }
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

    return table;
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
    println!(" 1    2   3
-------------
| {} | {} | {} | 1
-------------
| {} | {} | {} | 2
-------------
| {} | {} | {} | 3
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

pub fn check_winner(table: &mut [[char; 3]; 3], space: i8) -> char {
    /*
        Checks is a winner in rows

        -------------
        | X | X | X | <--- Rows
        -------------
        |   |   |   |
        -------------
        |   |   |    |
        -------------
    */
    if table[0][0] == table[0][1] && table[0][0] == table[0][2] {
        return table[0][0];
    }

    if table[1][0] == table[1][1] && table[1][0] == table[1][2] {
        return table[1][0];
    }

    if table[2][0] == table[2][1] && table[2][0] == table[2][2] {
        return table[2][0];
    }

     /*
        Checks is a winner in columns

        -------------
        | X |   |   |
        -------------
        | X |   |   |
        -------------
        | X |   |    |
        -------------
         /\
        Columns
    */
    if table[0][0] == table[1][0] && table[0][0] == table[2][0] {
        return table[0][0];
    }

    if table[0][1] == table[1][1] && table[0][1] == table[2][1] {
        return table[0][1];
    }

    if table[0][2] == table[1][2] && table[0][2] == table[2][2] {
        println!("winned");
        return table[0][2];
    }

    /*
        Cheks is a winner in crosswise direction

        -------------
        | X |   |   |
        -------------
        |   | X |   |
        -------------
        |   |   | X | <--- crosswise direction
        -------------
    */

    if table[0][0] == table[1][1] && table[0][0] == table[2][2] {
        return table[0][0];
    }

    if table[2][0] == table[1][1] && table[2][0] == table[0][2] {
        return table[2][0];
    }

    if space == 0 {
        return TIE;
    }

    return ' ';
}

pub fn random_y() -> i16 {
    let mut rng = thread_rng();
    let random: i16 = rng.gen_range(1..4); 

    return random;
}

pub fn random_x() -> i16 {
    let mut rng = thread_rng();
    let random: i16 = rng.gen_range(1..4);

    return random;
}

pub fn random_brain() -> i16 {
    let mut rng = thread_rng();
    let random: i16 = rng.gen_range(1..3);

    return random;
}


pub fn computer_move(table: &mut [[char; 3]; 3]) {
    let mut x: i16 = random_x()-1;
    let mut y: i16 = random_y()-1;

    if table[x as usize][y as usize] == ' ' {
        table[x as usize][y as usize] = COMPUTER;
        return;
    } else {
        while table[x as usize][y as usize] != ' ' {
            x = random_x()-1;
            y = random_y()-1;

            if table[x as usize][y as usize] == ' ' {
                table[x as usize][y as usize] = COMPUTER;
                return;
            } else {
                continue;
            }
        }
    }
}

pub fn main() {
    let mut winner: char = ' ';
    let mut table = [
        [' ', ' ', ' '],
        [' ', ' ', ' '],
        [' ', ' ', ' '],
    ];

    while winner == ' ' && free_spaces(&mut table) != 0 {
        print_table(&mut table);

        player_move(&mut table);

        let sp: i8 = free_spaces(&mut table);
        winner = check_winner(&mut table, sp);

        if winner == TIE {
            print_table(&mut table);
            println!("TiE! Nobody wins because there no space left!");
            process::exit(0x0100);
        } else if winner == COMPUTER {
            print_table(&mut table);
            println!("CoMpUtEr WiNs!");
            process::exit(0x0100);
        } else if winner == PLAYER {
            print_table(&mut table);
            println!("YoU WiN!");
            process::exit(0x0100);
        }

        computer_move(&mut table);

        let sp: i8 = free_spaces(&mut table);
        winner = check_winner(&mut table, sp);

        if winner == TIE {
            print_table(&mut table);
            println!("TiE! Nobody wins because there no space left!");
            process::exit(0x0100);
        } else if winner == COMPUTER {
            print_table(&mut table);
            println!("CoMpUtEr WiNs!");
            process::exit(0x0100);
        } else if winner == PLAYER {
            print_table(&mut table);
            println!("YoU WiN!");
            process::exit(0x0100);
        }
    }

    if winner == TIE {
        print_table(&mut table);
        println!("TiE! Nobody wins because there no space left!");
        process::exit(0x0100);
    } else if winner == COMPUTER {
        print_table(&mut table);
        println!("CoMpUtEr WiNs!");
        process::exit(0x0100);
    } else if winner == PLAYER {
        print_table(&mut table);
        println!("YoU WiN!");
        process::exit(0x0100);
    }
}
