use indextree::Arena;
use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut tree = Arena::new();
    let root = tree.new_node(("/", 0u32));
    let mut cwd = root;

    for line in input.iter().skip(1) {
        let mut s = line.split_ascii_whitespace();
        let c = s.next().unwrap();
        match c {
            "$" => match s.next().unwrap() {
                "cd" => {
                    let dir = s.next().unwrap();
                    if dir == ".." {
                        let up = cwd.ancestors(&tree).nth(1).unwrap();
                        tree.get_mut(up).unwrap().get_mut().1 += tree.get(cwd).unwrap().get().1;
                        cwd = up;
                        continue;
                    }
                    let n = tree.new_node((dir, 0));
                    cwd.append(n, &mut tree);
                    cwd = n;
                }
                "ls" => (),
                _ => unreachable!(),
            },
            "dir" => (),
            _ => {
                let x: u32 = c.parse().unwrap();
                tree.get_mut(cwd).unwrap().get_mut().1 += x;
            }
        }
    }
    // add remaining files all the way up
    while let Some(up) = cwd.ancestors(&tree).nth(1) {
        tree.get_mut(up).unwrap().get_mut().1 += tree.get(cwd).unwrap().get().1;
        cwd = up;
    }

    // eprintln!("{:?}", root.debug_pretty_print(&tree));

    let sum = tree
        .iter()
        .map(|n| if n.get().1 <= 100_000 { n.get().1 } else { 0 })
        .sum::<u32>();
    println!("part 1: {sum}");

    let used = tree.get(root).unwrap().get().1;
    assert!(used >= 40_000_000, "not all files were counted");
    let needed = 30_000_000 - (70_000_000 - tree.get(root).unwrap().get().1);

    let mut d = tree.iter().map(|n| n.get().1).collect::<Vec<_>>();
    d.sort_unstable();
    let res = d.into_iter().find(|x| *x >= needed).unwrap();
    println!("part 2: {res}");

    Ok(())
}
