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

        *self.table.get(idx).unwrap_or(&0)
    }

    fn max_subsquare(&self, only_three: bool) -> (isize, (isize, isize, isize)) {
        let mut max_score = 0;
        let mut max_size = 0;
        let mut max_coords = (0, 0);

        let sizes = if only_three { 3..4 } else { 1..300 };
        for y in 0..300 {
            for x in 0..300 {
                for size in sizes.clone() {
                    let (x_n, y_n) = (x + size - 1, y + size - 1);
                    if x_n >= 300 || y_n >= 300 {
                        continue;
                    }

                    let current = self.total((x, y), (x_n, y_n));
                    if current > max_score {
                        max_score = current;
                        max_size = size;
                        max_coords = (x, y);
                    }
                }
            }
        }

        (max_score, (max_coords.0, max_coords.1, max_size))
    }

    fn total(&self, (x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
        let a = self.get(x1 - 1, y1 - 1);
        let b = self.get(x1 - 1, y2);
        let c = self.get(x2, y1 - 1);
        let d = self.get(x2, y2);

        d - b - c + a
    }
}

pub fn run(input: Lines) -> Result<(), Error> {
    let mut input = input;
    let serial = input
        .next()
        .ok_or(format_err!("expected an input"))?
        .parse::<isize>()?;

    let grid = FuelGrid::new(serial);
    println!("I: {:?}", grid.max_subsquare(true));
    println!("II: {:?}", grid.max_subsquare(false));

    Ok(())
}
