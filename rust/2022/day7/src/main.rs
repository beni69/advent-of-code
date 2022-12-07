use indextree::{Arena, NodeId};
use lib::{input::get_input_lines, year_day, Result};

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut tree = Arena::new();
    let mut cwd = tree.new_node(("/", 0u32));

    for line in input.iter().skip(1) {
        let mut s = line.split_ascii_whitespace();
        let c = s.next().unwrap();
        match c {
            "$" => match s.next().unwrap() {
                "cd" => {
                    let dir = s.next().unwrap();
                    // if dir == "/" {
                    //     cwd = find_root(&tree);
                    //     continue;
                    // }
                    if dir == ".." {
                        let next = cwd.ancestors(&tree).nth(1).unwrap();
                        tree.get_mut(next).unwrap().get_mut().1 += tree.get(cwd).unwrap().get().1;
                        cwd = next;
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
            } // _ => sum += c.parse::<u32>().unwrap(),
        }
    }
    let next = cwd.ancestors(&tree).nth(1).unwrap();
    tree.get_mut(next).unwrap().get_mut().1 += tree.get(cwd).unwrap().get().1;

    let root = find_root(&tree);

    println!("{:?}", root.debug_pretty_print(&tree));

    let sum = tree
        .iter()
        .map(|n| if n.get().1 <= 100_000 { n.get().1 } else { 0 })
        .sum::<u32>();
    dbg!(sum);

    let used = tree.get(root).unwrap().get().1;
    dbg!(used);
    let needed = (70_000_000 - tree.get(root).unwrap().get().1);
    dbg!(needed);

    let mut d = tree.iter().map(|n| n.get().1).collect::<Vec<_>>();
    d.sort_unstable();
    let res = d.iter().find(|x| **x >= needed).unwrap();
    dbg!(res);

    Ok(())
}

fn find_root(tree: &Arena<(&str, u32)>) -> NodeId {
    tree.get_node_id(
        tree.iter()
            .find(|n| !n.is_removed() && n.get().0 == "/")
            .unwrap(),
    )
    .unwrap()
}
