fn main() {
    let input = include_str!("./input2.txt");
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


const NUM_AS_STR: [(&str, u32); 9] = [ 
        ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
            ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
    ];

fn line_to_int(line: &str) -> u32 {

    let mut first_int: Option<(usize,u32)> = None;
    let mut second_int: Option<(usize,u32)> = None;
   
    for (idx, chr) in line.chars().into_iter().enumerate() {
        
        let chr_to_int: (usize, u32) = match chr.to_digit(10) {
            Some(num) => (idx,num),
            None => continue,
        };

        if first_int == None && second_int == None {
            first_int = Some(chr_to_int);
        }

        second_int = Some(chr_to_int);
    }

    for (st, num) in  NUM_AS_STR.into_iter(){
        let v: Vec<(usize, &str)> = line.match_indices(st).collect();

        for (us, _st2) in v.into_iter(){
  
            let _ = match first_int {
                Some(first) => {
                    if us < first.0{
                        first_int = Some( (us, num) );
                    }
                },
                None => (),
            };      
            
            let _ = match second_int {
                Some(second) => {
                    if us > second.0{
                        second_int = Some( (us, num) );
                    }
                },
                None => (),
            };      
        }
    }

    match (first_int, second_int){
        (Some(a), Some(b)) => 
            { 
                println!("a: {}, b: {}", a.1, b.1);
                (a.1 * 10) + b.1 
            },
        _ => 0
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen");
        assert_eq!(result,281);
    }

    #[test]
    fn case1(){
        let result = part2(
            "mdxdlh5six5nqfld9bqzxdqxfour"); // 50 + 4 = 54
        assert_eq!(result,54);
    }

}