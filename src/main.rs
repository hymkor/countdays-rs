fn is_uruu(y: i32) -> bool {
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
        if is_uruu(y) {
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
            2 => if is_uruu(y) { 29 } else { 28 },
            4|6|9|11 => 30,
            _ => 31,
        };
    }
    return sum
}

fn count_days(y: i32,m: i32,d: i32) -> i32 {
    return years_to_days(2000,y) + month_to_days(m,y) + d
}

fn yyyymmdd_to_days(dt: &str) -> Result<i32,std::num::ParseIntError> {
    let year: i32 = dt[..4].parse()?;
    let month: i32 = dt[4..6].parse()?;
    let day: i32 = dt[6..].parse()?;
    return Ok(count_days(year,month,day));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn try_test(d1: &str, d2: &str, diff: i32) -> bool{
        if let Ok(_d1) = yyyymmdd_to_days(d1) {
            if let Ok(_d2) = yyyymmdd_to_days(d2) {
                return _d2 - _d1 == diff;
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
        match yyyymmdd_to_days(&arg1) {
            Ok(_days) => {
                if let Some(_last) = last {
                    println!("{}",_days-_last);
                }
                last = Some(_days)
            }
            Err(errmsg) => {
                eprintln!("Error: {}",errmsg);
                std::process::exit(1);
            }
        };
    }
}
