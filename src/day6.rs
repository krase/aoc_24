use std::cmp::{PartialEq, PartialOrd};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
enum Field {
    #[default]
    Empty,
    Obstacle,
    Guard(Direction),
    Visited,
    Visited2,
    Visited3,
}

#[derive(Debug, Clone, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone)]
struct Map {
    pos: Pos, // Guard position
    direction: Direction,
    width: usize,
    height: usize,
    fields: Vec<Field>,
    visited: usize,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        println!("Size: w: {}, h: {}", width, height);
        Self {
            pos: Pos { x: 0, y: 0 },
            direction: Direction::Up,

            width,
            height,

            fields: vec![Field::Empty; width * height],
            visited: 0,
        }
    }

    fn xy2index(&self, pos: &Pos) -> usize {
        pos.y as usize * self.width + pos.x as usize
    }

    fn index2pos(&self, index: usize) -> Pos {
        Pos {
            x: (index % self.width) as isize,
            y: (index / self.width) as isize,
        }
    }

    fn set_pos(&mut self, pos: Pos) {
        self.pos.x = pos.x;
        self.pos.y = pos.y;
        //println!("Next Position {:?}", self.pos);
    }

    fn field_at(&self, pos: &Pos) -> &Field {
        &self.fields[self.xy2index(&pos)]
    }

    fn field_mut(&mut self) -> &mut Field {
        let index = self.xy2index(&self.pos);
        &mut self.fields[index]
    }

    fn field_mut_at(&mut self, x: usize, y: usize) -> &mut Field {
        let index = self.xy2index(&Pos {x: x as isize, y: y as isize});
        &mut self.fields[index]
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn next_pos(&self) -> Option<Pos> {
        let mut pos: Pos = self.pos.clone();
        match self.direction {
            Direction::Up => {
                if self.pos.y > 0isize {
                    pos.y -= 1;
                } else {
                    return None;
                }
            }
            Direction::Down => {
                if self.pos.y + 1 < self.height as isize {
                    pos.y += 1;
                } else {
                    return None;
                }
            }
            Direction::Left => {
                if pos.x > 0isize {
                    pos.x -= 1;
                } else {
                    return None;
                }
            }
            Direction::Right => {
                if pos.x + 1 < self.width as isize {
                    pos.x += 1;
                } else {
                    return None;
                }
            }
        }
        Some(pos)
    }
}

fn read_input(file: &str) -> Map {
    let file = File::open(file).expect("file not found!");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();
    let mut map: Map = Map::new(lines[0].len(), lines.len());

    let mut i = 0;

    for line in lines {
        for c in line.chars() {
            match c {
                '#' => map.fields[i] = Field::Obstacle,
                'O' => map.fields[i] = Field::Obstacle,
                '.' => map.fields[i] = Field::Empty,
                '<' => map.fields[i] = Field::Guard(Direction::Left),
                '>' => map.fields[i] = Field::Guard(Direction::Right),
                'v' => map.fields[i] = Field::Guard(Direction::Down),
                '^' => map.fields[i] = Field::Guard(Direction::Up),
                _ => {}
            }
            if let Field::Guard(dir) = &map.fields[i] {
                map.pos = map.index2pos(i);
                map.direction = dir.clone();
            }
            i += 1;
        }
    }

    map
}

fn simulate(map: &mut Map) -> bool {
    map.visited += 1;
    *map.field_mut() = Field::Visited;
    loop {
        let next_pos = map.next_pos();
        if let Some(next_pos) = next_pos {
            let next_field = map.field_at(&next_pos).clone();
            match next_field {
                Field::Obstacle => {
                    map.turn_right();
                }
                _ => {
                    map.set_pos(next_pos);
                    if next_field < Field::Visited {
                        *map.field_mut() = Field::Visited;
                    } else if next_field < Field::Visited2 {
                        *map.field_mut() = Field::Visited2;
                    } else if next_field < Field::Visited3 {
                        *map.field_mut() = Field::Visited3;
                    } else if next_field == Field::Visited3 {
                        return true;
                    }
                }
            }
        } else {
            break;
        }
    }
    false
}

fn main() {
    println!("Hello, world!");
    //let n_workers = 8;
    //let pool = ThreadPool::builder().pool_size(n_workers).create().expect("invalid pool");
    //let mut map = read_input("src/example6.txt");
    let map_base = read_input("src/day6.txt");
    //println!("Data: {:?}", &data);
    let mut count = 0usize;
    for x in 0 .. map_base.width {
        for y in 0 .. map_base.height {
            let mut map = map_base.clone();
            if *map.field_at(&Pos{x:x as isize, y:y as isize}) != Field::Empty {
                continue;
            }
            *map.field_mut_at(x, y) = Field::Obstacle;
            let found_loop = simulate(&mut map);
            if found_loop {
                count += 1;
            }
        }
    }
    println!("Value: {:?}", count);
}
