use request::Error;
use std::str::Lines;

const WIDTH: isize = 300;

fn index_at(x: isize, y: isize) -> usize {
    (y * WIDTH + x) as usize
}

fn power_at(serial: isize, x: isize, y: isize) -> isize {
    let rack_id = x + 10;
    let base = y * rack_id;
    let scaled = (base + serial) * rack_id;

    ((scaled / 100) % 10) - 5
}

#[derive(Debug)]
struct FuelGrid {
    table: Vec<isize>,
}

impl FuelGrid {
    fn new(serial: isize) -> Self {
        let mut table = vec![0_isize; 90_000];

        for y in 0..300 {
            for x in 0..300 {
                let idx = index_at(x, y);

                println!("{:?}", table.get(index_at(x - 1, y)).unwrap_or(&0));

                table[idx] = table.get(index_at(x - 1, y)).unwrap_or(&0)
                    + table.get(index_at(x, y - 1)).unwrap_or(&0)
                    - table.get(index_at(x - 1, y - 1)).unwrap_or(&0)
                    + power_at(serial, x as isize, y as isize);
            }
        }

        FuelGrid { table }
    }

    fn get(&self, x: isize, y: isize) -> isize {
        let idx = index_at(x, y);
        if idx < 0 || idx > 90_000 {
            // OUT OF BOUNDS
            0
        } else {
            *self.table.get(idx).unwrap()
        }
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
    let mut input = input;
    let serial = input
        .next()
        .ok_or(format_err!("expected an input"))?
        .parse::<isize>()?;

    let grid = FuelGrid::new(serial);

    //    println!("{:?}", grid);

    Ok(())
}

/*
const WIDTH: isize = 300;

type SubGridCost = Vec<(isize, isize, (isize, isize))>;

enum Square {
    Fixed,  // 3x3
    Custom, // NxN where 1 ≤ N ≤ 300
}

fn index(row: isize, col: isize) -> usize {
    ((row - 1) * WIDTH + (col - 1)) as usize
}

fn get_neighbors(row: isize, col: isize, size: isize) -> Vec<(isize, isize)> {
    let mut neighbors = Vec::new();
    for r in 0..size {
        for c in 0..size {
            neighbors.push((col + c, row + r))
        }
    }

    neighbors
}

#[derive(Debug)]
struct FuelGrid(Vec<isize>);
impl FuelGrid {
    fn new(serial: isize) -> Self {
        let mut grid = vec![0; 90_000];

        for row in 1..301 {
            for col in 1..301 {
                let rack_id = col + 10;
                let base_power = row * rack_id;
                let scaled_power = (base_power + serial) * rack_id;

                let power = ((scaled_power / 100) % 10) - 5;
                println!("{:?}", index(row, col));
                grid[index(row, col)] = power
                    + *grid.get(index(row - 1, col)).unwrap_or(&0)
                    + *grid.get(index(row, col - 1)).unwrap_or(&0)
                    + *grid.get(index(row - 1, col - 1)).unwrap_or(&0);
            }
        }

        FuelGrid(grid)
    }

    fn subgrids(&self, square: Square) -> SubGridCost {
        let square_sizes = match square {
            Square::Fixed => vec![3],
            Square::Custom => (1..301).collect::<Vec<_>>(),
        };

        let mut subgrids = Vec::new();
        for row in 1..301 {
            for col in 1..301 {
                let scored: Vec<(isize, isize, (isize, isize))> = square_sizes
                    .iter()
                    .map(|s| {
                        let neighbors = get_neighbors(row, col, *s);
                        let score = neighbors.iter().fold(0, |mut score, &(row, col)| {
                            if let Some(power_level) = self.get(col, row) {
                                score += power_level;
                            }
                            score
                        });

                        let (row, col) = neighbors[0];

                        (score, *s, (col, row))
                    })
                    .collect();
                subgrids.push(scored);
            }
        }

        subgrids.into_iter().flat_map(|v| v).collect()
    }

    fn sorted_subgrids(&self, square: Square) -> SubGridCost {
        let mut subgrids = self.subgrids(square);

        subgrids.sort_by_key(|&(score, _, _)| score);
        subgrids
    }

    fn get(&self, x: isize, y: isize) -> Option<&isize> {
        self.0.get(index(y, x))
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
    let mut input = input;
    let serial = input
        .next()
        .ok_or(format_err!("expected an input"))?
        .parse::<isize>()?;

    let grid = FuelGrid::new(serial);
    let subgrids = grid.sorted_subgrids(Square::Fixed);

    println!("{:?}", subgrids.len());
    println!("{:?}", subgrids[subgrids.len() - 1]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{index, index_to_coords, FuelGrid, Square};

    #[test]
    fn test_round_trip_index() {
        assert_eq!((64, 243), index_to_coords(index(243, 64)));
    }

    #[test]
    fn test_expected_power_levels() {
        let tests = vec![
            (57, (122, 79), -5),
            (39, (217, 196), 0),
            (71, (101, 153), 4),
        ];

        for (serial, (col, row), expected) in tests {
            let grid = FuelGrid::new(serial);
            assert_eq!(Some(&(expected as isize)), grid.get(col, row))
        }
    }

    #[test]
    fn test_grid() {
        let tests = vec![(18, 29), (42, 30)];

        for (serial, expected_power) in tests {
            let grid = FuelGrid::new(serial);
            let subgrids = grid.sorted_subgrids(Square::Fixed);

            let (actual, _, _) = subgrids[90_000 - 1];

            assert_eq!(expected_power, actual);
        }
    }

    fn test_sized_grid() {
        let tests = vec![(18, 113, (90, 269, 16)), (42, 119, (232, 251, 12))];

        for (serial, expected_power, candidate) in tests {
            let grid = FuelGrid::new(serial);
            let subgrids = grid.sorted_subgrids(Square::Custom);

            let (actual, _, _) = subgrids[90_000 - 1];

            assert_eq!(expected_power, actual);
        }
    }

}
*/
