fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    
    let mut result: u32 = 0;

    for line in input.lines() {
        result += line_to_int(line);
    }
    
    result
}


const RED_CUBES: (&str, i32) = ("red", 12);
const GREEN_CUBES: (&str, i32) = ("green", 13);
const BLUE_CUBES: (&str, i32) = ("blue", 14);


fn line_to_int(line: &str) -> u32 {

    // 0 = "Game x"
    // 1 = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let split_by_colon = line.split(":").collect::<Vec<_>>();

    let game = split_by_colon[1].split(";");

    let mut min_num_reds: u32 = 0;
    let mut min_num_greens: u32 = 0;
    let mut min_num_blues: u32 = 0;

    for rounds in game{
      
        let hands = rounds.trim().split(", ").collect::<Vec<_>>();

        for h in hands {
            let num_colour_name = h.split(" ").collect::<Vec<_>>();

            let num = num_colour_name[0].parse::<i32>().unwrap();
        
            if num_colour_name[1] == RED_CUBES.0 && num > min_num_reds as i32 {
                min_num_reds = num as u32;
            }
    
            if num_colour_name[1] == GREEN_CUBES.0 && num > min_num_greens as i32 {
                min_num_greens = num as u32;
            }
    
            if num_colour_name[1] == BLUE_CUBES.0 && num > min_num_blues as i32 {
                min_num_blues = num as u32;
            }
        }
    }

    min_num_reds * min_num_greens * min_num_blues
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
        let result = part2(
     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
     Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
     Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
     Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
     Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        
        assert_eq!(result,2286);
    }

}



fn _line_to_int_old(line: &str) -> u32 {

    let cube_list: Vec<(&str, i32)> = vec![
        RED_CUBES, GREEN_CUBES, BLUE_CUBES
    ];

    // 0 = "Game x"
    // 1 = "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    let split_by_colon = line.split(":").collect::<Vec<_>>();

    let binding = String::from(split_by_colon[0]);
    let game_name_trim = binding.trim();

    // 0 = "Game "
    // 1 = "x"
    let split_by_game = game_name_trim.split("Game ").collect::<Vec<_>>();

    let game_num = split_by_game[1].parse::<u32>().unwrap();

    println!("Game: {}", game_num);

    let binding = split_by_colon[1]
        .replace(";", ",");
    let rounds = binding
        .split(", ");


    let mut amount_reds: i32 = 0;
    let mut amount_greens: i32 = 0;
    let mut amount_blues: i32 = 0;

    
    for g in rounds{
        // println!("{}", g);

        let amount_colour = g.trim()
            .split(" ")
            .collect::<Vec<_>>();
        println!("{:?}", amount_colour);

        let num = amount_colour[0].parse::<i32>().unwrap();

        match amount_colour[1] {
            "red" => amount_reds += num,
            "green" => amount_greens += num,
            "blue" => amount_blues += num,
            _ => ()
        };
    }

    let mut r = true;
    let mut g = true;
    let mut b = true;

    for mut cubes in cube_list.into_iter() {
        match cubes.0 {
            "red" => {
                cubes.1 -= amount_reds;
                if cubes.1 < 0 { r = false  }
            },
            "green" => {
                cubes.1 -= amount_greens;
                if cubes.1 < 0 { g = false  }
            },
            "blue" => {
                cubes.1 -=  amount_blues;
                if cubes.1 < 0 { b = false  }
            },
            _ => ()
        };
    }

    match (r, g, b){
        (true, true, true) => {println!("Possible\n"); game_num},
        _ => {println!("NOT POSSIBLE\n"); 0}
    }
}