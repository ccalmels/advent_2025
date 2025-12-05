use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 10 } else { 140 };
const S: i32 = SIZE as i32;

fn get_forklift(grid: &[[u8; SIZE]; SIZE]) -> Vec<(usize, usize)> {
    let mut ret = vec![];

    for y in 0..S {
        for x in 0..S {
            if grid[y as usize][x as usize] != b'@' {
                continue;
            }

            let mut adjacents = 0;

            for (dx, dy) in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ] {
                if y + dy < 0 || y + dy >= S {
                    continue;
                }
                if x + dx < 0 || x + dx >= S {
                    continue;
                }

                if grid[(y + dy) as usize][(x + dx) as usize] == b'@' {
                    adjacents += 1;
                }
            }

            if adjacents < 4 {
                ret.push((x as usize, y as usize));
            }
        }
    }

    ret
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut grid = [[0; SIZE]; SIZE];

    for (y, line) in lines.enumerate() {
        let line = line.unwrap();

        for (x, &c) in line.as_bytes().iter().enumerate() {
            grid[y][x] = c;
        }
    }

    let to_remove = get_forklift(&grid);
    let p1 = to_remove.len();

    for (x, y) in to_remove.into_iter() {
        grid[y][x] = b'.';
    }

    let mut p2 = p1;

    loop {
        let to_remove = get_forklift(&grid);

        if to_remove.is_empty() {
            break;
        }

        p2 += to_remove.len();

        for (x, y) in to_remove.into_iter() {
            grid[y][x] = b'.';
        }
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (13, 43));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
