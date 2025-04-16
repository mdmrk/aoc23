use std::collections::HashMap;

use crate::days::Day;

pub struct Day010;

static DIRS: &[((isize, isize), &str); 4] = &[
    ((-1, 0), "-LF"), // left
    ((1, 0), "-J7"),  // right
    ((0, 1), "|LJ"),  // down
    ((0, -1), "|F7"), // up
];

static GROUND: char = '.';
type Pos = (isize, isize);

fn dfs(
    tiles: &Vec<Vec<char>>,
    init_pos: &Pos,
    visited: &mut HashMap<Pos, bool>,
    pos: Pos,
) -> isize {
    print!("n: {:?}", pos);
    let mut buf = "".to_string();
    std::io::stdin().read_line(&mut buf).unwrap();

    if let Some(t) = visited.get(&pos) {
        if pos == *init_pos {
            return 1;
        }
        return 1;
    }
    visited.insert((pos.0, pos.1), true);

    for dir in DIRS {
        if dir.1.contains(tiles[pos.0 as usize][pos.1 as usize]) {}
    }
    DIRS.iter()
        .map(|dir| {
            let now: Pos = (pos.0 + dir.0.0, pos.1 + dir.0.1);
            let mut visited: HashMap<Pos, bool> = HashMap::new();

            if now.0 >= 0
                && now.1 >= 0
                && now.0 < tiles.len() as isize
                && now.1 < tiles[0].len() as isize
                && dir.1.contains(tiles[now.0 as usize][now.1 as usize])
            {
                1 + dfs(&tiles, &init_pos, &mut visited, now)
            } else {
                isize::MIN
            }
        })
        .max()
        .unwrap()
}

impl Day for Day010 {
    fn problem(&self) -> &'static str {
        include_str!("problem.txt")
    }
    fn input(&self) -> &'static str {
        include_str!("input.txt")
    }
    fn part1(&self, input: &String) -> Result<String, String> {
        let mut result: usize = 0;

        let tiles: Vec<Vec<char>> = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();

        let init_pos: Pos = 'o: loop {
            let mut i: usize = 0;

            while i < tiles.len() {
                let mut j: usize = 0;

                while j < tiles[i].len() {
                    if tiles[i][j] == 'S' {
                        break 'o (i as isize, j as isize);
                    }
                    j += 1;
                }
                i += 1;
            }
        };
        println!("init_pos: {:?}", init_pos);
        let far: isize = DIRS
            .iter()
            .map(|dir| {
                let pos = init_pos;
                let now: Pos = (pos.0 + dir.0.0, pos.1 + dir.0.1);
                let mut visited: HashMap<Pos, bool> = HashMap::new();

                visited.insert((init_pos.0, init_pos.1), true);
                if now.0 >= 0
                    && now.1 >= 0
                    && now.0 < tiles.len() as isize
                    && now.1 < tiles[0].len() as isize
                    && dir.1.contains(tiles[now.0 as usize][now.1 as usize])
                {
                    dfs(&tiles, &init_pos, &mut visited, now)
                } else {
                    isize::MIN
                }
            })
            .max()
            .unwrap();

        result = far as usize;
        Ok(result.to_string())
    }

    fn part2(&self, input: &String) -> Result<String, String> {
        let result: usize = 0;

        Ok(result.to_string())
    }
}
