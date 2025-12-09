use std::io::{BufRead, Lines};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn surface(&self, other: &Tile) -> i64 {
        ((other.x - self.x).abs() + 1) * ((other.y - self.y).abs() + 1)
    }

    fn vector(&self, other: &Tile) -> (i64, i64) {
        ((other.x - self.x), (other.y - self.y))
    }
}

#[test]
fn check_surface() {
    let a = Tile { x: 1, y: 1 };

    assert_eq!(2, a.surface(&Tile { x: 2, y: 1 }));
    assert_eq!(3, a.surface(&Tile { x: 1, y: 3 }));

    assert_eq!(4, a.surface(&Tile { x: 2, y: 2 }));
    assert_eq!(12, a.surface(&Tile { x: 4, y: 3 }));
}

#[derive(Debug, PartialEq)]
struct ParseTileError;

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let positions: Vec<&str> = s.split(',').collect();

        if positions.len() != 2 {
            return Err(ParseTileError);
        }

        let x = positions[0].trim().parse().map_err(|_| ParseTileError)?;
        let y = positions[1].trim().parse().map_err(|_| ParseTileError)?;

        Ok(Tile { x, y })
    }
}

fn is_segments_intersect(
    segments: &[(i64, (i64, i64))],
    (v0, v1): (i64, i64),
    (u0, u1): (i64, i64),
) -> bool {
    let index = segments.binary_search_by_key(&(v0 - 1), |&(v, _)| v);
    let mut index = match index {
        Ok(i) => i + 1,
        Err(i) => i + 1,
    };

    while index < segments.len() {
        let (v, (a, b)) = segments[index];

        if v > v1 {
            break;
        }

        if v < v0 {
            continue;
        }

        if a > b {
            if b > u1 || a < u0 {
                // outside
            } else {
                return true;
            }
        } else if a < b {
            if b < u0 || a > u1 {
                // outside
            } else {
                return true;
            }
        } else {
            panic!("Not possible!");
        }

        index += 1;
    }

    false
}

fn resolve<T>(lines: Lines<T>) -> (i64, i64)
where
    T: BufRead,
{
    let (mut p1, mut p2) = (0, 0);
    let tiles: Vec<Tile> = lines
        .map(|line| Tile::from_str(&line.unwrap()).unwrap())
        .collect();
    let tiles_len = tiles.len();

    // finding external angle points
    let mut external_angle_tiles = vec![];
    let mut prev = tiles[tiles.len() - 1].vector(&tiles[0]);
    let mut rotation = 0;

    for i in 0..tiles_len {
        let (a, b) = (&tiles[i], &tiles[(i + 1) % tiles_len]);
        let v = a.vector(b);
        let product = prev.0 * v.1 - prev.1 * v.0;

        rotation += product.signum();

        let external_tile = if product > 0 {
            Tile {
                x: tiles[i].x + prev.0.signum() - v.0.signum(),
                y: tiles[i].y + prev.1.signum() - v.1.signum(),
            }
        } else {
            Tile {
                x: tiles[i].x - prev.0.signum() + v.0.signum(),
                y: tiles[i].y - prev.1.signum() + v.1.signum(),
            }
        };

        external_angle_tiles.push(external_tile);

        prev = v;
    }

    // Check that we're going in CW order. It has some implication on
    // where to find empty tiles regarding the vector direction.
    assert_eq!(rotation, 4);

    let mut verticals = vec![];
    let mut horizontals = vec![];

    for i in 0..tiles_len {
        let (a, b) = (
            &external_angle_tiles[i],
            &external_angle_tiles[(i + 1) % tiles_len],
        );

        if a.x == b.x {
            // vertical
            verticals.push((a.x, (a.y, b.y)));
        } else if a.y == b.y {
            // horizontal
            horizontals.push((a.y, (a.x, b.x)));
        } else {
            panic!("We have a diagonal!");
        }
    }

    verticals.sort_unstable_by_key(|&(v, _)| v);
    horizontals.sort_unstable_by_key(|&(v, _)| v);

    // remove mutability
    let verticals = verticals;
    let horizontals = horizontals;

    for i in 0..tiles_len - 1 {
        for j in i + 1..tiles_len {
            let (a, b) = (&tiles[i], &tiles[j]);
            let surface = a.surface(b);

            p1 = p1.max(surface);

            if surface > p2 {
                let xs = if a.x < b.x { (a.x, b.x) } else { (b.x, a.x) };
                let ys = if a.y < b.y { (a.y, b.y) } else { (b.y, a.y) };

                if is_segments_intersect(&verticals, xs, ys) {
                    continue;
                }

                if is_segments_intersect(&horizontals, ys, xs) {
                    continue;
                }

                p2 = surface;
            }
        }
    }

    (p1, p2)
}

#[test]
fn check() {
    const TEST: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    use std::io::Cursor;

    assert_eq!(resolve(Cursor::new(TEST).lines()), (50, 24));
}

fn resolve_string<T>(lines: Lines<T>) -> (String, String)
where
    T: BufRead,
{
    let solution = resolve(lines);
    (solution.0.to_string(), solution.1.to_string())
}

inventory::submit! { advent_2025::Day::new(file!(), resolve_string) }
