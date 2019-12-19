use crate::{
    util::{Direction, Point2us},
    Solution,
};
use pathfinding::directed::bfs::bfs;

pub fn solve(input: String) -> Solution<usize, usize> {
    let (mut map, keys) = parse_input(input);
    let p1 = solve_part1(&map, keys);
    transform_map(&mut map);
    let p2 = solve_part2(&mut map, keys);
    Solution::new(p1, p2)
} // 151.24s

fn solve_part1(map: &[Vec<char>], all_keys: u32) -> usize {
    let start_pos = get_entrances(map)[0];
    let start = Cell::new(start_pos, 0);
    let path = bfs(
        &start,
        |Cell { pos, keys }| {
            get_neighbors(pos)
                .iter()
                .filter_map(|neighbor| match map[neighbor.y][neighbor.x] {
                    '#' => None,
                    '.' | '@' => Some(Cell::new(*neighbor, *keys)),
                    key if key.is_ascii_lowercase() => {
                        let pos = key as u8 - 97;
                        if (keys >> pos) & 1 == 1 {
                            Some(Cell::new(*neighbor, *keys))
                        } else {
                            Some(Cell::new(*neighbor, keys + (1 << pos)))
                        }
                    }
                    door if door.is_ascii_uppercase() => {
                        let pos = door as u8 - 65;
                        if (keys >> pos) & 1 == 1 {
                            Some(Cell::new(*neighbor, *keys))
                        } else {
                            None
                        }
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        },
        |Cell { keys, .. }| *keys == all_keys,
    )
    .unwrap();
    path.len() - 1
}

fn solve_part2(map: &mut Vec<Vec<char>>, all_keys: u32) -> usize {
    let start_poss = get_entrances(&map);
    let start = Cells::new(start_poss, 0, None);
    let path = bfs(
        &start,
        |Cells { poss, keys, active }| {
            let mut successors: Vec<Cells> = Vec::new();
            for (vault, pos) in poss.iter().enumerate() {
                if let Some(bot) = active {
                    if *bot != vault {
                        continue;
                    }
                }
                for neighbor in get_neighbors(pos) {
                    let mut status = Some(vault);
                    let mut keys = *keys;
                    match map[neighbor.y][neighbor.x] {
                        '#' => continue,
                        key if key.is_ascii_lowercase() => {
                            let pos = key as u8 - 97;
                            if (keys >> pos) & 1 == 0 {
                                keys += 1 << pos;
                                status = None;
                            }
                        }
                        door if door.is_ascii_uppercase() => {
                            if (keys >> (door as u8 - 65)) & 1 == 0 {
                                continue;
                            }
                        }
                        _ => {}
                    }
                    let mut new_poss = poss.clone();
                    new_poss[vault] = neighbor;
                    successors.push(Cells::new(new_poss, keys, status));
                }
            }
            successors
        },
        |Cells { keys, .. }| *keys == all_keys,
    )
    .unwrap();
    path.len() - 1
}

fn parse_input(input: String) -> (Vec<Vec<char>>, u32) {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    let all_keys = input
        .chars()
        .filter(char::is_ascii_lowercase)
        .fold(0, |keys, key| keys + (1 << (key as u8 - 97)));
    (map, all_keys)
}

fn transform_map(map: &mut Vec<Vec<char>>) {
    let start = get_entrances(&map)[0];
    map[start.y][start.x] = '#';
    map[start.y - 1][start.x] = '#';
    map[start.y][start.x - 1] = '#';
    map[start.y + 1][start.x] = '#';
    map[start.y + 1][start.x + 1] = '@';
    map[start.y - 1][start.x + 1] = '@';
    map[start.y + 1][start.x - 1] = '@';
    map[start.y - 1][start.x - 1] = '@';
}

fn get_neighbors(pos: &Point2us) -> Vec<Point2us> {
    let mut neighbors = Vec::new();
    for direction in Direction::iter() {
        let nx = match direction {
            Direction::W => pos.x - 1,
            Direction::E => pos.x + 1,
            _ => pos.x,
        };
        let ny = match direction {
            Direction::N => pos.y - 1,
            Direction::S => pos.y + 1,
            _ => pos.y,
        };
        neighbors.push(Point2us::new(nx, ny));
    }
    neighbors
}

fn get_entrances(map: &[Vec<char>]) -> Vec<Point2us> {
    let mut entrances = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '@' {
                entrances.push(Point2us::new(x, y));
            }
        }
    }
    entrances
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Cells {
    poss: Vec<Point2us>,
    keys: u32,
    active: Option<usize>,
}

impl Cells {
    fn new(poss: Vec<Point2us>, keys: u32, status: Option<usize>) -> Self {
        Self {
            poss,
            keys,
            active: status,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
struct Cell {
    pos: Point2us,
    keys: u32,
}

impl Cell {
    fn new(pos: Point2us, keys: u32) -> Self {
        Self { pos, keys }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test18() {
        let input =
            "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################"
            .to_string();
        let (map, keys) = parse_input(input);
        assert_eq!(solve_part1(&map, keys), 86);
        let input =
            "########################\n#@..............ac.GI.b#\n###d#e#f################\n###A#B#C################\n###g#h#i################\n########################"
            .to_string();
        let (map, keys) = parse_input(input);
        assert_eq!(solve_part1(&map, keys), 81);
        let input =
            "#############\n#g#f.D#..h#l#\n#F###e#E###.#\n#dCba@#@BcIJ#\n#############\n#nK.L@#@G...#\n#M###N#H###.#\n#o#m..#i#jk.#\n#############"
            .to_string();
        let (mut map, keys) = parse_input(input);
        assert_eq!(solve_part2(&mut map, keys), 72);
        crate::util::tests::test_full_problem(18, solve, 4420, 2128);
    }
}
