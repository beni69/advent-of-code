use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?
        .iter()
        .map(|l| l.split(',').map(String::from))
        .map(|mut l| (l.next().unwrap(), l.next().unwrap()))
        .map(|p| (to_iter(&p.0), to_iter(&p.1)))
        .collect::<Vec<_>>();

    println!(
        "part 1: {}",
        input.iter().map(overlap).filter(|x| *x).count()
    );
    println!(
        "part 2: {}",
        input.iter().map(overlap2).filter(|x| *x).count()
    );

    Ok(())
}
fn to_iter(s: &str) -> Vec<usize> {
    let mut split = s.split('-').map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap()..split.next().unwrap() + 1).collect()
}
fn overlap((v1, v2): &(Vec<usize>, Vec<usize>)) -> bool {
    let v = if v1.len() > v2.len() {
        (v1, v2)
    } else {
        (v2, v1)
    };

    v.0[0] <= v.1[0] && v.0[v.0.len() - 1] >= v.1[v.1.len() - 1]
}
fn overlap2((v1, v2): &(Vec<usize>, Vec<usize>)) -> bool {
    v1.iter().any(|x| v2.binary_search(x).is_ok())
}
