use std::collections::VecDeque;

pub fn solve(mut data: &mut VecDeque<i32>) -> i32 {
    let mut result = 0;
    let order: Vec<i32> = data.iter().copied().collect();
    for i in 1..=3005 {
        let current = order[(i-1) % order.len()];
	// println!("current {}", current);
        permute(&mut data, current);
        if [1000, 2000, 3000].contains(&i) {
            let index = index_of(&data, 0) + 1;
	    let kek = data[index];
	    println!("ith: {}", kek);
            result += data[index];
        }
    }
    result
}

fn index_of(data: &VecDeque<i32>, current: i32) -> usize {
    let index = data.iter().position(|&x| x == current).unwrap();
    index
}

fn permute(data: &mut VecDeque<i32>, current: i32) {
    if current == 0 {
	return;
    }
    
    let pos = index_of(data, current);
    // println!("permute {} on pos {}", current, pos);
    data.remove(pos);

    let mut pos = pos as i32 + current;
    if pos < 0 {
        pos = data.len() as i32 + pos;
    } else if pos > data.len() as i32 {
        pos = pos - data.len() as i32;
    }
    let mut pos = pos as usize;
    if pos == 0 {
	pos = data.len();
    }
    // println!("new pos: {}", pos);
    data.insert(pos, current);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        let mut data: VecDeque<i32> = VecDeque::from([1, 2, -3, 3, -2, 0, 4]);
        permute(&mut data, 1);
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![2, 1, -3, 3, -2, 0, 4]);

        permute(&mut data, 2);
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, -3, 2, 3, -2, 0, 4]);

        permute(&mut data, -3); // 1, -3, 2, 3, -2, 0, 4
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, 2, 3, -2, -3, 0, 4]);

        permute(&mut data, 3);
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, 2, -2, -3, 0, 3, 4]);

        permute(&mut data, -2); // 1, 2, -3, 0, 3, 4, -2
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, 2, -3, 0, 3, 4, -2]);

     	permute(&mut data, 0);
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, 2, -3, 0, 3, 4, -2]);

       	permute(&mut data, 4);
        let actual: Vec<i32> = data.iter().copied().collect();
        assert_eq!(actual, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_solve() {
	let mut data: VecDeque<i32> = VecDeque::from([1, 2, -3, 3, -2, 0, 4]);
	assert_eq!(solve(&mut data), 3);
    }
}
