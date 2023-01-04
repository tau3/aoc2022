fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut res = 0;
    for (i, v) in snafu.chars().rev().enumerate() {
        let v = match v {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unknown token {}", v),
        };
        res += 5_i64.pow(i as u32) * v;
    }
    res
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut res = String::from("");
    while decimal != 0 {
        let i = (decimal + 2) % 5;
        let v = match i {
            0 => "=",
            1 => "-",
            2 => "0",
            3 => "1",
            4 => "2",
            _ => panic!("unknown token {}", i),
        };
        res.push_str(v);
        decimal = (decimal + 2) / 5;
    }

    res.chars().rev().collect()
}

pub fn solve(input: &[&str]) -> String {
    let sum = input.iter().map(|line| snafu_to_decimal(line)).sum();
    decimal_to_snafu(sum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    #[test]
    fn test_with_real_data(){
        let input = util::read_real_data("day25");
        let input: Vec<&str> = input.iter().map(|line| line.as_str()).collect();
        assert_eq!(solve(&input), "asd");
    }

    #[test]
    fn test_solve() {
        let input = [
            "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
            "1=", "122",
        ];
        assert_eq!(solve(&input), "2=-1=0");
    }

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("12111"), 906);
        assert_eq!(snafu_to_decimal("2=0="), 198);
        assert_eq!(snafu_to_decimal("21"), 11);
        assert_eq!(snafu_to_decimal("2=01"), 201);
        assert_eq!(snafu_to_decimal("111"), 31);
        assert_eq!(snafu_to_decimal("20012"), 1257);
        assert_eq!(snafu_to_decimal("112"), 32);
        assert_eq!(snafu_to_decimal("1=-1="), 353);
        assert_eq!(snafu_to_decimal("1-12"), 107);
        assert_eq!(snafu_to_decimal("12"), 7);
        assert_eq!(snafu_to_decimal("1="), 3);
        assert_eq!(snafu_to_decimal("122"), 37);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(1747), "1=-0-2");
        assert_eq!(decimal_to_snafu(906), "12111");
        assert_eq!(decimal_to_snafu(198), "2=0=");
        assert_eq!(decimal_to_snafu(11), "21");
        assert_eq!(decimal_to_snafu(201), "2=01");
        assert_eq!(decimal_to_snafu(31), "111");
        assert_eq!(decimal_to_snafu(1257), "20012");
        assert_eq!(decimal_to_snafu(32), "112");
        assert_eq!(decimal_to_snafu(353), "1=-1=");
        assert_eq!(decimal_to_snafu(107), "1-12");
        assert_eq!(decimal_to_snafu(7), "12");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(37), "122");
    }
}
