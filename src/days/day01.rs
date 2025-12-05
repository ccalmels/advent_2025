use std::io::{BufRead, Lines};

const DIAL_SIZE: i32 = 100;

fn resolve<T>(lines: Lines<T>) -> (i32, i32)
where
    T: BufRead,
{
    let (_, p1, p2) = lines.fold((50, 0, 0), |(dial, acc1, acc2), line| {
        let line = line.unwrap();
        let bytes = line.as_bytes();
        let clicks = bytes
            .iter()
            .skip(1)
            .fold(0, |acc, b| acc * 10 + (b - b'0') as i32);

        let newdial = if bytes[0] == b'L' {
            dial - clicks
        } else {
            dial + clicks
        };

        let (div, rem) = (newdial.div_euclid(DIAL_SIZE), newdial.rem_euclid(DIAL_SIZE));
        let (p1, p2) = (
            (rem == 0) as i32, // check that we're on 0
            div.abs()
                + if bytes[0] == b'L' {
                    (rem == 0) as i32 // we're going back to 0, add a rotation not in div
                        - (dial == 0) as i32 // we're starting from 0, but div wrongly count a rotation
                } else {
                    0
                },
        );

        (rem, acc1 + p1, acc2 + p2)
    });

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (3, 6));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
