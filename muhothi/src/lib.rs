pub fn execute_bf(s :&String) -> Result<(), &str> {
    let mut buf = [0usize; 30000];

    let mut count = 0;
    let mut i :usize = 0;
    let mut note :usize = 0;
    while count != s.len() {
        match s.chars().nth(count).unwrap() {
            '+' => buf[i] += 1,
            '-' => buf[i] -= 1,
            '>' => i += 1,
            '<' => i -= 1,
            '.' => print!("{}", buf[i] as u8 as char),
            ',' => return Err("未実装"),
            '[' => if buf[i] == 0 { 
                count = note // TODO countを]へジャンプする
            },
            ']' => return Err("未実装"),
            _  => continue, 
        };
        count += 1;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::{execute_bf};
    #[test]
    fn test_bf() {
        let s = "+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.>++++++++++.".to_string();
        match execute_bf(&s) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}