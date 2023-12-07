
fn main() {
    let input = include_str!("input1.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
enum EngineToken{
    Symbol(String),
    Number(u32),
    Dot,
}

fn part1(input: &str) -> u32 {
    
    let mut v: Vec<Vec<EngineToken>> = vec![];

    for line in input.lines() {
        v.push( line_to_vec(line) );
    }

    let result = solver(v);

    result
}

static POS_COMBINATIONS: [(i32, i32); 8] = [
            (1, 0), (-1, 0), (0, -1), (0, 1),
            (-1, -1), (1,1), (1,-1),(-1,1),
];


fn positions_to_check
(
    r: usize, 
    c: usize, 
    max_r: usize,
    max_c: usize,
) 
-> Vec<(usize, usize)>{
    
    let mut v: Vec<(usize, usize)> = vec![];
    
    for (x, y)  in POS_COMBINATIONS{
        let res_c: i32 = y + (c as i32);
        let res_r: i32 = x + (r as i32);

        if 
            (res_c >= 0 && res_c <= max_c as i32) 
        && 
            (res_r >= 0 && res_r <= max_r as i32)
        {
            v.push( (res_r as usize, res_c as usize) );
        }
    } 

    v
}


fn solver(grid: Vec<Vec<EngineToken>>) -> u32 {

    let max_row_idx = grid.len()-1;
    let max_toke_idx = grid.get(0).unwrap().len()-1;
    
    let mut toke_row: Vec<(u32, usize)> = vec![];

    let mut result: u32 = 0;

    for (r, row) in grid.iter().enumerate(){
                
        for (c, toke) in row.iter().enumerate() {
            
            let mut _pos_check_v: Vec<(usize, usize)> = vec![];

            match toke {
               
                EngineToken::Number(_) => {
                    _pos_check_v = positions_to_check(r,c,max_row_idx,max_toke_idx);
                },
                
                _ => continue,
            }
            
            for (x, y) in _pos_check_v.iter() {
                match grid[*x][*y] {
                    EngineToken::Symbol(_) => {
                        match toke{
                            EngineToken::Number(t_n) => {
                                if !toke_row.contains(&(*t_n, r)){
                                    result += t_n;
                                    toke_row.push((*t_n, r));
                                }
                            },
                            _ => ()
                        }
                    },
                    _ => continue
                }
            } 
        }
    }
    
    
    result
}


fn line_to_vec(line: &str) -> Vec<EngineToken> {
    
    let mut v: Vec<EngineToken> = vec![];
    
    for c in line.trim_start().chars(){
           
           match c.to_digit(10){
               Some(n) => {
                   v.push(EngineToken::Number(n));
               },
               
               None => {
                   match c {
                       '.' => {
                           v.push(EngineToken::Dot);
                       },
                       
                       _ => {
                           v.push(EngineToken::Symbol(String::from(c)));
                       }
                   }
               }
           } 
           
        }

    let mut pow: u32 = 1;
    let mut num = 0;

    let mut idx_val_re: Vec<(u32, Vec<usize>)> = vec![(0, vec![])];
    let mut curr_idx = 0;

    for (idx, tok) in v.iter_mut().enumerate().rev(){

        match tok {
            
            EngineToken::Number(n) => {
               
                num += *n * pow;

                let mut temp_v: Vec<usize> = idx_val_re[curr_idx].1.to_owned();
                temp_v.push(idx);
                idx_val_re[curr_idx] = (num, temp_v);

                pow = pow * 10;
            },
            _ => {
                if num != 0{
                    curr_idx += 1;
                    idx_val_re.push((0, vec![]));
                }
                
                num = 0;
                pow = 1;
            }
        }
    }

    for (val, idx_v) in idx_val_re {
        for i in idx_v {
            v[i] = EngineToken::Number(val);
        }
    }
  
       
    v
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn base_case(){
       
        let result = part1(
        "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..");

        assert_eq!(result, 4361);
    }

    
    #[test]
    fn actual_result(){
        let input = include_str!("input1.txt");
        let output = part1(input);
        assert_eq!(output, 532331);
    }


}