use std::collections::{HashSet, VecDeque};

use ndarray::Array2;

type Node = ((usize, usize), usize);

fn main() -> color_eyre::Result<()> {
    let file = include_str!("input");

    let ncols = file.lines().next().unwrap().len();
    let mut data = Vec::new();
    let mut nrows = 0;
    for line in file.lines() {
        data.extend(line.chars().map(|c| char_to_digit(&c)));
        nrows += 1;
    }
    let arr = Array2::from_shape_vec((nrows, ncols), data)?;

    let start = find_start(&arr).unwrap();

    let mut elements_to_check = VecDeque::from([start]);
    let mut move_count = 0;
    let mut visited = HashSet::new();
    visited.insert(start.0);
    let mut nodes_left_in_next_layer = 0;
    let mut nodes_left_in_this_layer = 1;

    while !elements_to_check.is_empty() {
        let next = elements_to_check.pop_front().unwrap();

        // is the next element the end?
        if next.1 == 27 {
            break;
        }

        // find valid moves next to this one.
        let next_elements = find_valid_moves(&arr, next);
        let next_elements = next_elements
            .iter()
            .filter(|element| !visited.contains(&element.0))
            .map(|(index, element)| (*index, *element))
            .collect::<Vec<Node>>();

        nodes_left_in_next_layer += next_elements.len();
        elements_to_check.extend(next_elements);

        nodes_left_in_this_layer -= 1;
        if nodes_left_in_this_layer == 0 {
            nodes_left_in_this_layer = nodes_left_in_next_layer;
            nodes_left_in_next_layer = 0;
            move_count += 1;
        }
        visited.insert(next.0);
    }

    println!("{move_count:?}");
    Ok(())
}

fn char_to_digit(c: &char) -> usize {
    match *c as u8 {
        b'a'..=b'z' => 1 + (*c as u8 - b'a') as usize,
        b'S' => 0,
        b'E' => 27,
        _ => unreachable!(),
    }
}

fn find_start(array: &Array2<usize>) -> Option<Node> {
    array
        .indexed_iter()
        .find(|(_, element)| **element == 0)
        .map(|(index, element)| (index, *element))
}

/// find_valid_moves returns the valid moves from the current character to one
/// that is either above by 1, equal or any amount below.
fn find_valid_moves(array: &Array2<usize>, current_element: Node) -> Vec<Node> {
    let mut neighbours = vec![];
    let index = current_element.0;
    let (above, below, right, left) = (
        (index.0, index.1 + 1),
        (index.0, index.1.checked_sub(1)),
        (index.0 + 1, index.1),
        (index.0.checked_sub(1), index.1),
    );

    let above_element = array.get(above);
    let below_element = if let (x, Some(y)) = below {
        array.get((x, y))
    } else {
        None
    };
    let right_element = array.get(right);
    let left_element = if let (Some(x), y) = left {
        array.get((x, y))
    } else {
        None
    };

    if let Some(value) = above_element {
        if *value <= current_element.1 + 1 {
            neighbours.push((above, *value))
        }
    }
    if let Some(value) = below_element {
        if *value <= current_element.1 + 1 {
            neighbours.push(((below.0, below.1.unwrap()), *value))
        }
    }
    if let Some(value) = right_element {
        if *value <= current_element.1 + 1 {
            neighbours.push((right, *value))
        }
    }
    if let Some(value) = left_element {
        if *value <= current_element.1 + 1 {
            neighbours.push(((left.0.unwrap(), left.1), *value))
        }
    }

    neighbours
}

#[cfg(test)]
mod test_valid_moves {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_finds_valid_moves() {
        let array = array![[1, 2], [2, 3],];

        assert_eq!(
            find_valid_moves(&array, ((0, 0), 1)),
            vec![((0, 1), 2), ((1, 0), 2)]
        );
    }

    #[test]
    fn test_finds_valid_moves_2() {
        let array = array![[1, 1], [3, 3],];

        assert_eq!(find_valid_moves(&array, ((0, 0), 1)), vec![((0, 1), 1)]);
    }

    #[test]
    fn test_finds_valid_moves_3() {
        let array = array![[1, 1], [3, 3],];

        assert_eq!(
            find_valid_moves(&array, ((1, 1), 3)),
            vec![((1, 0), 3), ((0, 1), 1)]
        );
    }

    #[test]
    fn test_finds_valid_moves_4() {
        let array = array![[1, 1], [1, 1],];

        assert_eq!(
            find_valid_moves(&array, ((0, 1), 1)),
            vec![((0, 0), 1), ((1, 1), 1)]
        );
    }
}
