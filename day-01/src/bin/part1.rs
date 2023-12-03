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


fn line_to_int(line: &str) -> u32 {

    let mut first_int: Option<u32> = None;
    let mut second_int: Option<u32> = None;

    for chr in line.chars() {
        
        let chr_to_int: u32 = match chr.to_digit(10) {
            Some(num) => num,
            None => continue,
        };

        if first_int == None && second_int == None {
            first_int = Some(chr_to_int);
        }

        second_int = Some(chr_to_int);
    }
    
    match (first_int, second_int){
        (Some(a), Some(b)) =>  (a * 10) + b,
        _ => 0
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result,142);
    }

}