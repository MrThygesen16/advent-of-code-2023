fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    
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

    let binding = String::from(split_by_colon[0]);
    let game_name_trim = binding.trim();

    // 0 = "Game "
    // 1 = "x"
    let split_by_game = game_name_trim.split("Game ")
        .collect::<Vec<_>>();

    let game_num = split_by_game[1].parse::<u32>().unwrap();

    let game = split_by_colon[1].split(";");

    let mut result: Option<u32> = Some(game_num);

    for rounds in game{
      
        let hands = rounds.trim().split(", ").collect::<Vec<_>>();

        for h in hands {
            let num_colour_name = h.split(" ").collect::<Vec<_>>();

            let num = num_colour_name[0].parse::<i32>().unwrap();
        
            if num_colour_name[1] == RED_CUBES.0 && num > RED_CUBES.1 {
                result = None;           
                continue;    
            }
    
            if num_colour_name[1] == GREEN_CUBES.0 && num > GREEN_CUBES.1 {
                result = None;
                continue;
            }
    
            if num_colour_name[1] == BLUE_CUBES.0 && num > BLUE_CUBES.1 {
                result = None;
                continue;
            }
        }
    }

    match result {
        Some(n) => n,
        None => 0
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
        let result = part1(
     "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        
        assert_eq!(result,8);
    }

}