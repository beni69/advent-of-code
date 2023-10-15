use lib::{dprintln, input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let input = input
        .iter()
        .map(|l| {
            l.split(" -> ")
                .map(|x| x.split(',').map(|n| n.parse::<u32>().unwrap()))
                .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    dprintln!("{input:?}");

    let mut sands = Vec::new();

    loop {
        let mut sand = (500, 0);
        // fall
        loop {
            let below = (sand.0, sand.1 + 1);
            dprintln!("sand: {sand:?} below: {below:?}");
            if find(&below, &input, &sands).is_some() {
                dprintln!("ground below");

                // check diagonally down
                let left = (sand.0 - 1, sand.1);
                if find(&left, &input, &sands).is_none() {
                    dprintln!("left is open");
                    sand = left;
                    break;
                }
                let right = (sand.0 + 1, sand.1);
                if find(&right, &input, &sands).is_none() {
                    dprintln!("right is open");
                    sand = right;
                    break;
                }

                // sands.push(_sand);
                // sand = below;
                // break;
            }
        }
    }

    // Ok(())
}

fn find(
    pos: &(u32, u32),
    items: &Vec<Vec<(u32, u32)>>,
    sands: &Vec<(u32, u32)>,
) -> Option<(u32, u32)> {
    for sand in sands {
        if sand.0 == pos.0 && sand.1 == pos.1 {
            dprintln!("{pos:?} == {sand:?}");
            return Some(*sand);
        }
    }
    for path in items {
        for i in 0..path.len() - 1 {
            let (x1, y1) = *path[i..=i + 1].into_iter().min().unwrap();
            let (x2, y2) = *path[i..=i + 1].into_iter().max().unwrap();
            if x1 <= pos.0 && pos.0 <= x2 && y1 <= pos.1 && pos.1 <= y2 {
                dprintln!("{x1} {y1} < {pos:?} < {x2} {y2}");
                return Some((x1, y1));
            }
        }
    }
    None
}
