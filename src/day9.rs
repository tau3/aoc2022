pub fn turn(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if head == tail {
        return tail;
    }

    if is_adjacent(head, tail) {
        return tail;
    }

    let (xt, yt) = tail;
    let (xh, yh) = head;

    if xt == xh {
        return (xt, (yh - yt) / (yh - yt).abs());
    }

    if yt == yh {
        return ((xh - xt) / (xh - xt).abs(), yt);
    }

    return ((xh - xt) / (xh - xt).abs(), (yh - yt) / (yh - yt).abs());
}

fn is_adjacent(head: (i32, i32), tail: (i32, i32)) -> bool {
    let (x, y) = head;
    let around = [
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x + 1, y),
        (x + 1, y - 1),
        (x, y - 1),
        (x - 1, y - 1),
        (x - 1, y),
    ];

    around.contains(&tail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn() {
        assert_eq!(turn((5, 5), (5, 5)), (5, 5));
        assert_eq!(turn((1, 0), (0, 0)), (0, 0));
        assert_eq!(turn((0, 5), (0, 2)), (0, 3));
        assert_eq!(turn((0, 0), (3, 0)), (2, 0));
        assert_eq!(turn((2, 3), (1, 1)), (2, 2));
        assert_eq!(turn((3, 2), (1, 1)), (2, 2));
    }
}
