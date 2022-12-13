use ndarray::Array2;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let file = include_str!("input.txt");

    let ncols = file.lines().next().unwrap().len();
    let mut data = Vec::new();
    let mut nrows = 0;
    for line in file.lines() {
        // Compute `row` and append it to `data`; this is a trivial example:
        data.extend(line.chars().map(|c| c.to_digit(10).unwrap()));
        nrows += 1;
    }
    let arr = Array2::from_shape_vec((nrows, ncols), data)?;

    let _ = find_visable_trees(arr);

    Ok(())
}

/// A tree is visible if all of the other trees
/// between it and an edge of the grid are shorter than it.
/// Only consider trees in the same row or column;
/// that is, only look up, down, left, or right from any given tree.
fn find_visable_trees(arr: Array2<u32>) -> u32 {
    let rows = arr.rows();
    let cols = arr.columns();

    let all_lines = rows.into_iter().chain(cols.into_iter());

    for pos in all_lines {
        println!("{pos:?}")
    }

    1
}

#[cfg(test)]
mod test_trees {
    use ndarray::array;

    use super::*;

    #[test]
    fn test_find_visable_trees() {
        let test_case = array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0]
        ];

        assert_eq!(find_visable_trees(test_case), 21)
    }
}
