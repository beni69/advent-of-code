use lib::{input::get_input, year_day, Result};

fn _main() -> Result<()> {
    let inp = get_input(year_day!())?;

    let mut s1 = 0;
    let mut s2 = 0;
    for line in inp.trim().lines() {
        let (id, content) = line[5..].split_once(':').unwrap();
        let id: u32 = id.parse().unwrap();

        let valid = content.split(&[',', ';']).all(|x| {
            let mut s = x.split_whitespace();
            let n: u32 = s.next().unwrap().parse().unwrap();
            let limit = match s.next().unwrap() {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => unreachable!(),
            };
            n <= limit
        });
        if valid {
            s1 += id;
        }

        let mut mins = [0, 0, 0];
        content
            .split(&[',', ';'])
            .map(|x| {
                let mut s = x.split_whitespace();
                let n: u32 = s.next().unwrap().parse().unwrap();
                let i = match s.next().unwrap() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                if n > mins[i] {
                    mins[i] = n;
                }
            })
            .count();

        let prod = mins.iter().product::<u32>();
        s2 += prod;

        let grp = content
            .split(&[',', ';'])
            .map(|cube| {
                let mut split = cube.split_whitespace();
                let n: u32 = split.next().unwrap().parse().unwrap();
                let i = match split.next().unwrap() {
                    "red" => 0,
                    "green" => 1,
                    "blue" => 2,
                    _ => unreachable!(),
                };
                (n, i)
            })
            .collect::<Vec<_>>();

        let valid2 = grp.iter().all(|(n, i)| *n <= *i + 12);
        assert_eq!(valid, valid2);

        let x = [0, 1, 2]
            .iter()
            .map(|x| {
                grp.iter()
                    .filter_map(|(n, i)| if i == x { Some(n) } else { None })
                    .max()
                    .unwrap()
            })
            .product::<u32>();

        assert_eq!(x, prod);
    }
    println!("{s1}\n{s2}");

    Ok(())
}

fn main() -> Result<()> {
    let (sol1, sol2): (Vec<_>, Vec<_>) = get_input(year_day!())?
        .trim()
        .lines()
        .map(|line| {
            let (id, content) = line[5..].split_once(':').unwrap();
            let id: u32 = id.parse().unwrap();
            let cubes = content
                .split(&[',', ';'])
                .map(|cube| {
                    let mut split = cube.split_whitespace();
                    let n: u32 = split.next().unwrap().parse().unwrap();
                    let i = match split.next().unwrap() {
                        "red" => 0,
                        "green" => 1,
                        "blue" => 2,
                        _ => unreachable!(),
                    };
                    (n, i)
                })
                .collect::<Vec<_>>();

            let p1 = if cubes.iter().all(|(n, i)| *n <= *i + 12) {
                id
            } else {
                0
            };

            let p2 = [0, 1, 2]
                .iter()
                .map(|x| {
                    cubes
                        .iter()
                        .filter_map(|(n, i)| if i == x { Some(n) } else { None })
                        .max()
                        .unwrap()
                })
                .product::<u32>();

            (p1, p2)
        })
        .unzip();
    Ok(println!(
        "{}\n{}",
        sol1.iter().sum::<u32>(),
        sol2.iter().sum::<u32>()
    ))
}
