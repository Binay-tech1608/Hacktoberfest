use std::io;
use std::io::Write;

const EMPTY: char = ' ';
const WALL: char = '#';
const PLAYER: char = 'P';
const BOX: char = 'B';
const TARGET: char = 'T';

fn main() {
    let level = vec![
        "#####",
        "# B #",
        "#PT #",
        "#####",
    ];

    let mut player_x = 2;
    let mut player_y = 2;

    loop {
        print_level(&level, player_x, player_y);
        if is_victory(&level) {
            println!("Congratulations! You won!");
            break;
        }

        let mut input = String::new();
        print!("Enter a move (W/A/S/D): ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let direction = input.trim().to_uppercase();
        match direction.as_str() {
            "W" => move_player(&mut player_y, &mut player_x, -1, 0, &mut level),
            "S" => move_player(&mut player_y, &mut player_x, 1, 0, &mut level),
            "A" => move_player(&mut player_y, &mut player_x, 0, -1, &mut level),
            "D" => move_player(&mut player_y, &mut player_x, 0, 1, &mut level),
            _ => println!("Invalid input! Use W/A/S/D to move."),
        }
    }
}

fn print_level(level: &Vec<&str>, player_x: usize, player_y: usize) {
    for (y, row) in level.iter().enumerate() {
        for (x, tile) in row.chars().enumerate() {
            if x == player_x && y == player_y {
                print!("{}", PLAYER);
            } else {
                print!("{}", match tile {
                    ' ' => EMPTY,
                    '#' => WALL,
                    'B' => BOX,
                    'T' => TARGET,
                    _ => tile,
                });
            }
        }
        println!();
    }
}

fn move_player(player_y: &mut usize, player_x: &mut usize, dy: i32, dx: i32, level: &mut Vec<&str>) {
    let new_y = (*player_y as i32 + dy) as usize;
    let new_x = (*player_x as i32 + dx) as usize;
    let current_tile = level[*player_y].chars().nth(*player_x).unwrap();
    let next_tile = level[new_y].chars().nth(new_x).unwrap();

    if next_tile == EMPTY {
        level[*player_y] = replace_char_at(&level[*player_y], *player_x, EMPTY);
        level[new_y] = replace_char_at(&level[new_y], new_x, PLAYER);
        *player_y = new_y;
        *player_x = new_x;
    } else if next_tile == BOX {
        let box_next_y = (new_y as i32 + dy) as usize;
        let box_next_x = (new_x as i32 + dx) as usize;
        if level[box_next_y].chars().nth(box_next_x).unwrap() == EMPTY {
            level[*player_y] = replace_char_at(&level[*player_y], *player_x, EMPTY);
            level[new_y] = replace_char_at(&level[new_y], new_x, PLAYER);
            level[box_next_y] = replace_char_at(&level[box_next_y], box_next_x, BOX);
            *player_y = new_y;
            *player_x = new_x;
        }
    }
}

fn replace_char_at(s: &str, index: usize, replacement: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if i == index { replacement } else { c })
        .collect()
}

fn is_victory(level: &Vec<&str>) -> bool {
    level.iter().all(|row| !row.contains(BOX.to_string()))
}
