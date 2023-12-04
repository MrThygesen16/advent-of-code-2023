fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}


#[derive(Debug)]
enum Character{
    Number(u32),
    Symbol(String),
}

#[derive(Debug)]
struct Token {
    chr: Character,
    row: usize,
    col: usize,
}

#[derive(Debug)]
enum EngineToken{
    T(Token),
    Dot,
}

fn part1(input: &str) -> u32 {
    
    let result: u32 = 0;

    let mut v: Vec<Vec<EngineToken>> = vec![];

    for (row, line) in input.lines().enumerate().to_owned() {
        v.push( line_to_vec(line, row) );
    }

    for toke in v {
        println!("{:?}", toke);
        println!();
    }
    
    result
}


fn line_to_vec(line: &str, row_num: usize) -> Vec<EngineToken> {

    let mut v: Vec<EngineToken> = vec![];

    for (col_num, chr) in line.chars().enumerate() {
        
        match chr.to_digit(10) {
            Some(num) => {               
                let t = EngineToken::T(
                    Token {
                        chr: Character::Number(num),
                        col: col_num,
                        row: row_num,
                    }
                );

                v.push(t);
            },

            None => match chr {
           
                '.' => {
                    v.push(EngineToken::Dot);
                },

                _ => {
                    // println!("{}", chr);

                    let t = EngineToken::T(
                        Token {
                            chr: Character::Symbol(chr.to_string()),
                            col: col_num,
                            row: row_num,
                        }
                    );

                    // println!("{:?}", chr);

                    v.push(t);
                }

            },
        };
 
    }
    
    v
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
        let input = include_str!("default_test_case.txt");
        let result = part1(input);

        assert_eq!(result, 4361);
    }

}