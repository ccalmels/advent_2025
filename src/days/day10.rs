use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufRead, Lines};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Button {
    mask: u32,
    leds: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct ParseButtonError;

impl FromStr for Button {
    type Err = ParseButtonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        let len = bytes.len();

        if bytes[0] != b'(' || bytes[len - 1] != b')' {
            return Err(ParseButtonError);
        }

        let mut leds = vec![];
        let mut mask = 0;

        for &c in &bytes[1..len - 1] {
            match c {
                b',' => {}
                b'0'..=b'9' => {
                    let index = (c - b'0') as usize;

                    leds.push(index);
                    mask |= 1 << index;
                }
                _ => return Err(ParseButtonError),
            }
        }

        Ok(Button { mask, leds })
    }
}

#[test]
fn check_parse_button() {
    let s = "(1,2,3,5)";

    assert_eq!(
        Button::from_str(&s),
        Ok(Button {
            mask: 0b101110,
            leds: vec![1, 2, 3, 5]
        })
    );

    let s = "(1,2,a,5)";

    assert_eq!(Button::from_str(&s), Err(ParseButtonError));
}

type Combinations = Vec<Vec<usize>>;

fn find_combinations(
    buttons: &[Button],
    index: usize,
    leds: u32,
    objective: u32,
) -> Option<Combinations> {
    if index < buttons.len() {
        let mut ret = find_combinations(buttons, index + 1, leds, objective).unwrap_or_default();
        let mask = buttons[index].mask;
        let leds = (leds & !mask) | ((leds & mask) ^ mask);

        if let Some(mut v) = find_combinations(buttons, index + 1, leds, objective) {
            v.iter_mut().for_each(|combi| combi.push(index));

            ret.append(&mut v);
        }

        if ret.is_empty() {
            None
        } else {
            Some(ret)
        }
    } else if leds == objective {
        Some(vec![vec![]])
    } else {
        None
    }
}

#[test]
fn check_find_combinations() {
    let buttons =
        ["(3)", "(1,3)", "(2)", "(2,3)", "(0,2)", "(0,1)"].map(|s| Button::from_str(&s).unwrap());

    assert_eq!(
        find_combinations(&buttons, 0, 0, 0b110),
        Some(vec![
            vec![5, 4],
            vec![3, 1],
            vec![5, 4, 3, 2, 0],
            vec![2, 1, 0]
        ])
    );
}

fn find_combinations_cached(
    button: &[Button],
    objective: u32,
    cache: &mut HashMap<u32, Option<Combinations>>,
) -> Option<Combinations> {
    if let Some(v) = cache.get(&objective) {
        v.clone()
    } else {
        let ret = find_combinations(button, 0, 0, objective);

        cache.insert(objective, ret.clone());

        ret
    }
}

// The part 2 of this day was really hard for me. I tried to
// bruteforce things but at the end it was too long. Thanks to this
// post on Reddit:
// https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/
// I was able to complete this day in 35ms.
fn find_joules(
    buttons: &[Button],
    joules: &[u32],
    cache: &mut HashMap<u32, Option<Combinations>>,
) -> Option<usize> {
    if joules.iter().all(|&j| j == 0) {
        return Some(0);
    }

    let leds: u32 = joules
        .iter()
        .rev()
        .fold(0, |acc, joule| 2 * acc + if joule % 2 == 0 { 0 } else { 1 });

    find_combinations_cached(buttons, leds, cache).map(|v| {
        v.into_iter()
            .filter_map(|combi| {
                let mut j = Vec::from(joules);

                for &button in &combi {
                    for &led in &buttons[button].leds {
                        if j[led] > 0 {
                            j[led] -= 1;
                        } else {
                            return None;
                        }
                    }
                }

                for joule in &mut j {
                    assert_eq!(*joule % 2, 0);
                    *joule /= 2;
                }

                find_joules(buttons, &j, cache).map(|m| combi.len() + 2 * m)
            })
            .min()
    })?
}

#[test]
fn check_find_joules() {
    let buttons =
        ["(3)", "(1,3)", "(2)", "(2,3)", "(0,2)", "(0,1)"].map(|s| Button::from_str(&s).unwrap());
    let joules = vec![3, 5, 4, 7];

    let mut cache = HashMap::new();

    assert_eq!(Some(10), find_joules(&buttons, &joules, &mut cache));
}

fn resolve<T>(lines: Lines<T>) -> (usize, usize)
where
    T: BufRead,
{
    let machines = lines
        .map(|line| {
            let line = line.unwrap();
            let words = line.split_whitespace().collect::<Vec<_>>();

            let leds: u32 = words[0].as_bytes().iter().rev().fold(0, |acc, &b| match b {
                b'.' => acc * 2,
                b'#' => acc * 2 + 1,
                _ => acc,
            });
            let buttons: Vec<Button> = words[1..words.len() - 1]
                .iter()
                .map(|s| Button::from_str(s).unwrap())
                .collect();
            let mut joltages: Vec<u32> = vec![0];

            for &c in words[words.len() - 1].as_bytes() {
                match c {
                    b'0'..=b'9' => {
                        *joltages.last_mut().unwrap() =
                            10 * joltages.last().unwrap() + (c - b'0') as u32;
                    }
                    b',' => {
                        joltages.push(0);
                    }
                    _ => {}
                }
            }

            (leds, buttons, joltages)
        })
        .collect::<Vec<_>>();

    machines
        .into_par_iter()
        .fold(
            || (0, 0),
            |(p1, p2), (leds, buttons, joltages)| {
                let mut cache = HashMap::new();
                let combis = find_combinations_cached(&buttons, leds, &mut cache).unwrap();

                (
                    p1 + combis.iter().map(|v| v.len()).min().unwrap(),
                    p2 + find_joules(&buttons, &joltages, &mut cache).unwrap(),
                )
            },
        )
        .reduce(|| (0, 0), |(a1, a2), (b1, b2)| (a1 + b1, a2 + b2))
}

#[test]
fn check() {
    const TEST: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (7, 33));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
