
fn uru(y: i32) -> bool {
    if y % 2000 == 0 {
        return true
    } else if y % 100 == 0 {
        return false
    } else if y % 4 == 0 {
        return true
    } else {
        return false
    }
}

fn years_to_days(y1: i32,y2: i32) -> i32 {
    let mut sum: i32 = 0;
    for y in y1..y2 {
        if uru(y) {
            sum += 366
        } else {
            sum += 365
        }
    }
    return sum
}

fn month_to_days(m: i32,y: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..m {
        sum += match i {
            2 => if uru(y) { 29 } else { 28 },
            4|6|9|11 => 30,
            _ => 31,
        };
    }
    return sum
}

fn count_date(dt: &str) -> Result<i32,String> {
    let year: i32 = match dt[..4].parse() {
        Ok(y) => years_to_days(2000,y),
        Err(_) => return Err(format!("Parse Error: year: {}",&dt[0..4]))
    };
    let month: i32 = match dt[4..6].parse() {
        Ok(m) => month_to_days(m,year),
        Err(_) => return Err(format!("Parse Error: month: {}",&dt[4..6]))
    };
    let day: i32 = match dt[6..].parse() {
        Ok(d) => d,
        Err(_) => return Err(format!("Parse Error: day: {}",&dt[6..]))
    };
    return Ok(year+month+day);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_test(d1: &str, d2: &str, diff: i32) -> bool{
        if let Ok(d1) = count_date(d1) {
            if let Ok(d2) = count_date(d2) {
                return d2 - d1 == diff;
            }
        }
        return false;
    }

    #[test]
    fn test_count_date() {
        assert!(try_test("20230101","20230201",31));
        assert!(try_test("20230201","20230301",28));
        assert!(try_test("20231101","20231201",30));
        assert!(try_test("20231001","20231101",31));
    }
}

fn main() {
    let mut last: Option<i32> = None;
    for arg1 in std::env::args().skip(1){
        match count_date(&arg1) {
            Ok(days) => match last {
                None => { last = Some(days) },
                Some(_last) => {
                    println!("{}",days-_last);
                    last = Some(days);
                },
            },
            Err(errmsg) => {
                eprintln!("Error: {}",errmsg);
                std::process::exit(1);
            }
        };
    }
}
