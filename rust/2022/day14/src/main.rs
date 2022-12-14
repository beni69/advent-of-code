use lib::{dprintln, input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut items = input
        .iter()
        .map(|l| {
            l.split(" -> ")
                .map(|x| x.split(',').map(|n| n.parse::<u32>().unwrap()))
                .map(|mut s| (s.next().unwrap(), s.next().unwrap()))
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    // dprintln!("{input:?}");

    loop {
        let mut sand = (500, 0);
        // fall
        loop {
            let below = (sand.0, sand.1 + 1);
            dprintln!("sand: {sand:?} below: {below:?}");
            if find(&below, &items).is_some() {
                dprintln!("found below");
                std::process::exit(0);
                break;
            }
            sand = below;
        }
    }

    Ok(())
}

fn find(pos: &(u32, u32), items: &Vec<Vec<(u32, u32)>>) -> Option<(u32, u32)> {
    for path in items {
        for i in 0..path.len() - 1 {
            let (x1, y1) = path[i];
            let (x2, y2) = path[i + 1];
            dprintln!("x1: {x1} y1: {y1} x2: {x2} y2: {y2} pos: {pos:?}");
            // FIXME
            if x1 <= pos.0 && pos.0 <= x2 && y1 <= pos.1 && pos.1 <= y2 {
                return Some((x1, y1));
            }
        }
    }
    None
}
