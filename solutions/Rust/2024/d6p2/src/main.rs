use winnow::{ascii, combinator, prelude::*, token};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Empty,
    Visited,
    Obstacle,
    GuardUp,
    GuardDown,
    GuardLeft,
    GuardRight,
}

impl Cell {
    fn parser(input: &mut &str) -> PResult<Self> {
        combinator::dispatch! {token::any;
            '.' => combinator::empty.value(Self::Empty),
            '#' => combinator::empty.value(Self::Obstacle),
            '^' => combinator::empty.value(Self::GuardUp),
            'v' => combinator::empty.value(Self::GuardDown),
            '>' => combinator::empty.value(Self::GuardRight),
            '<' => combinator::empty.value(Self::GuardLeft),
            _ => combinator::fail
        }
        .parse_next(input)
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    dir: Direction,
    x: isize,
    y: isize,
}

fn solution(input: &str) -> usize {
    let row = combinator::repeat::<_, _, Vec<_>, _, _>(1.., Cell::parser);
    let mut grid: Vec<Vec<Cell>> =
        combinator::repeat(1.., combinator::terminated(row, ascii::multispace0))
            .parse(input)
            .unwrap();
    let width = grid[0].len();
    let guard = grid
        .iter_mut()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, cell)| (x, y, cell))
        })
        .find(|(_, _, cell)| {
            matches!(
                *cell,
                Cell::GuardUp | Cell::GuardDown | Cell::GuardRight | Cell::GuardLeft
            )
        })
        .expect("no guard in input");
    let mut guard = Guard {
        dir: match std::mem::replace(guard.2, Cell::Visited) {
            Cell::GuardUp => Direction::Up,
            Cell::GuardDown => Direction::Down,
            Cell::GuardLeft => Direction::Left,
            Cell::GuardRight => Direction::Right,
            _ => unreachable!(),
        },
        x: guard.0 as isize,
        y: guard.1 as isize,
    };
    // While buard in bounds
    loop {
        // Mark cell as visited
        grid[guard.y as usize][guard.x as usize] = Cell::Visited;
        use Direction::*;
        let (x, y) = match guard.dir {
            Left => (guard.x - 1, guard.y),
            Right => (guard.x + 1, guard.y),
            Up => (guard.x, guard.y - 1),
            Down => (guard.x, guard.y + 1),
        };
        if !((0..grid.len() as isize).contains(&y) && (0..width as isize).contains(&x)) {
            break;
        }
        let target = &mut grid[y as usize][x as usize];
        if let Cell::Obstacle = target {
            guard.dir = match guard.dir {
                Up => Right,
                Right => Down,
                Down => Left,
                Left => Up,
            };
            continue;
        }
        // Step forward
        guard.x = x;
        guard.y = y;
    }
    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|c| matches!(c, Cell::Visited))
        .count()
}

fn main() {
    aoc_util::hook_solution(solution);
}
