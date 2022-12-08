use anyhow::Result;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn day1(input: &dyn AsRef<[&str]>) -> Result<(u32, u32)> {
    let mut x = Vec::new();
    let mut current = Vec::new();
    for line in input.as_ref() {
        if line.is_empty() {
            x.push(current);
            current = Vec::new();
            continue;
        }
        let calories: u32 = line.parse()?;
        current.push(calories);
    }
    return Ok((0, 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_day1() {
        let actual = day1(&[
            "1000", "2000", "3000", "", "4000", "5000", "6000", "", "7000", "8000", "9000", "10000",
        ]);
        assert_eq!(actual.unwrap(), (4, 24000));
    }
}
