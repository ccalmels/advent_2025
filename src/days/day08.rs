use std::io::{BufRead, Lines};
use std::str::FromStr;

const PAIRS: usize = if cfg!(test) { 10 } else { 1000 };

#[derive(Debug, PartialEq)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

impl Junction {
    fn square_distance(&self, other: &Junction) -> i64 {
        (other.x - self.x) * (other.x - self.x)
            + (other.y - self.y) * (other.y - self.y)
            + (other.z - self.z) * (other.z - self.z)
    }
}

#[derive(Debug, PartialEq)]
struct ParseJunctionError;

impl FromStr for Junction {
    type Err = ParseJunctionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<&str> = s.split(',').collect();

        if positions.len() != 3 {
            return Err(ParseJunctionError);
        }

        let x = positions[0]
            .trim()
            .parse()
            .map_err(|_| ParseJunctionError)?;
        let y = positions[1]
            .trim()
            .parse()
            .map_err(|_| ParseJunctionError)?;
        let z = positions[2]
            .trim()
            .parse()
            .map_err(|_| ParseJunctionError)?;

        Ok(Junction { x, y, z })
    }
}

#[test]
fn check_junction() {
    assert_eq!(
        Junction::from_str("1,2,3"),
        Ok(Junction { x: 1, y: 2, z: 3 })
    );
    assert_eq!(Junction::from_str("1,2"), Err(ParseJunctionError));
    assert_eq!(
        Junction::from_str("1, 2, 3"),
        Ok(Junction { x: 1, y: 2, z: 3 })
    );
}

fn resolve<T>(lines: Lines<T>) -> (usize, i64)
where
    T: BufRead,
{
    let (mut p1, mut p2) = (vec![], 0);
    let junctions: Vec<Junction> = lines
        .map(|line| Junction::from_str(&line.unwrap()).unwrap())
        .collect();

    let size = junctions.len();
    let mut connections = vec![];
    let mut circuits = (0..size).map(|e| vec![e]).collect::<Vec<_>>();

    for i in 0..size - 1 {
        for j in i + 1..size {
            let distance = junctions[i].square_distance(&junctions[j]);

            connections.push((distance, i, j));
        }
    }

    connections.sort_unstable_by_key(|&(d, _, _)| d);

    for (n, &(_, i, j)) in connections.iter().enumerate() {
        let first_index = circuits
            .iter()
            .position(|e| e.contains(&i) || e.contains(&j))
            .unwrap();
        let second_index = circuits[first_index + 1..]
            .iter()
            .position(|e| e.contains(&i) || e.contains(&j));

        if let Some(second_index) = second_index {
            let second_index = second_index + first_index + 1;
            let to_merge = circuits.swap_remove(second_index);

            if circuits.len() == 1 {
                p2 = junctions[i].x * junctions[j].x;
                break;
            }

            circuits[first_index].extend(to_merge);
        }

        if n == PAIRS - 1 {
            p1 = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
        }
    }

    p1.sort_unstable();

    (p1.into_iter().rev().take(3).product(), p2)
}

#[test]
fn check() {
    const TEST: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (40, 25272));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
