fn solve(input: Vector<String>) {
    let input = input.into();
    let (initial, moves) = parse(input);
    let stacks = parse_initial(initial);

    for move_ : moves{
	let move_ = parse_move(move);
	apply_move(stacks, move_);
    }


}

fn headers(stacks: [VecDeque<String>; POSITIONS.size()]){
    let mut result = String::new();
    for stack in stacks {
	let item = stack.pop_front();
	result.push(item);
    }
    result
}

fn parse(mut input: VecDeque<String>)-> (Vec<String>, Vec<String>){
    let mut initial = Vec::new();
    let mut moves = Vec::new();

    let target = &initial;

    while input.pop_front() == Some(item) {
	if(!item.empty()){
	    initial.push(item);
	}
	else{
	break;}
    }

    (initial, input)
}

const POSITIONS = [1,5,9,13,17,21,25,29,33];

fn parse_initial(initial: &Vec<String>)-> [VecDeque<char>; POSITIONS.size()]{
    let result = [];
    for line in initial {
	for i in POSITIONS{
	    if line[i] != ' '{
		result[i].push_front(line[i]);
	    }
	}
	
	let tokens =line.split(" ").collect();
	// 1, 5, 9, 13, 17
    }

    result
}

fn parse_move(move: &String) -> (u32, u32, u32){
    let mut tokens  =move.split(" ");
    let amount = tokens.nth(1);
    let from = tokens.nth(1);
    let to = tokens.nth(1);
    (amount, from, to)
}

fn apply_move(mut stacks: &[VecDeque<char>; POSITIONS.size()], (amount, from, to): (u32,u32,u32)) {
    let source = stacks[from-1];
    let target = stacks[to-1];
    for i in 0..amount{
	let item = source.pop_front().unwrap();
	target.push_front(item);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn test_solve(){
	let input = vec![
"    [D]",    
	    "[N] [C]",
	    "[Z] [M] [P]",
	    " 1   2   3 ",
	    "",
	    "move 1 from 2 to 1",
	    "move 3 from 1 to 3",
	    "move 2 from 2 to 1",
	    "move 1 from 1 to 2"];
	assert_eq!(solve(&input), "CMZ");
    }
}
