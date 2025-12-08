use std::io::{BufRead, Lines};

const SIZE: usize = if cfg!(test) { 15 } else { 141 };

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let mut p1 = 0;
    let mut beams = vec![0usize; SIZE];

    beams[SIZE / 2] = 1;

    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();

        // Start position
        let mut tmp = vec![0usize; SIZE];

        for (index, paths) in beams
            .into_iter()
            .enumerate()
            .filter(|(_, paths)| *paths != 0)
        {
            if line[index] == b'^' {
                if index > 0 && line[index - 1] != b'^' {
                    tmp[index - 1] += paths;
                }
                if index < SIZE && line[index + 1] != b'^' {
                    tmp[index + 1] += paths;
                }
                p1 += 1;
            } else {
                tmp[index] += paths;
            }
        }

        beams = tmp;
    }

    (p1, beams.into_iter().sum())
}

#[test]
fn check() {
    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (21, 40));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
