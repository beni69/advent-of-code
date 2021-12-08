use aoc::get_input_lines;
fn main() {
    let input = get_input_lines();
    let in_split = input
        .iter()
        .map(|l| {
            let split = l.split("|").collect::<Vec<&str>>();
            (split[0].trim(), split[1].trim())
        })
        .collect::<Vec<(&str, &str)>>();

    // part 1
    // 1: 2
    // 4: 4
    // 7: 3
    // 8: 7
    {
        let input = in_split
            .iter()
            .map(|x| x.1.to_string())
            .collect::<Vec<String>>();

        let mut sum = 0;
        for line in input.iter() {
            for s in line.split_ascii_whitespace() {
                match s.len() {
                    2 | 4 | 3 | 7 => sum += 1,
                    _ => {}
                }
            }
        }
        println!("part 1: {}", sum);
    }

    // part 2
    /*
     *  0000
     * 1    2
     * 1    2
     *  3333
     * 4    5
     * 4    5
     *  6666
     */
    {
        let valid_indexes = vec![
            /* 0 */ vec![0, 1, 2, 4, 5, 6],
            /* 1 */ vec![2, 5],
            /* 2 */ vec![0, 2, 3, 4, 6],
            /* 3 */ vec![0, 2, 3, 5, 6],
            /* 4 */ vec![1, 2, 3, 5],
            /* 5 */ vec![0, 1, 3, 5, 6],
            /* 6 */ vec![0, 1, 3, 4, 5, 6],
            /* 7 */ vec![0, 2, 5],
            /* 8 */ vec![0, 1, 2, 3, 4, 5, 6],
            /* 9 */ vec![0, 1, 2, 3, 5, 6],
        ];
        let d_base: Display = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
        let mut displays: Vec<Display> = Vec::new();
        permutate(&mut d_base.clone(), d_base.len(), &mut |x| {
            displays.push(*x)
        });
        dbg!(displays.len());

        let mut sol: Vec<i32> = Vec::new();
        // go through every line
        for line in in_split.iter() {
            let mut found = true;
            let mut tmp: Option<Display> = None;
            // try every combination
            for d in displays.iter() {
                found = true;
                // verify every pattern
                for pat in line.0.split_ascii_whitespace() {
                    let mut indexes = Vec::new();
                    for c in pat.chars() {
                        indexes.push(d.iter().position(|x| *x == c).unwrap());
                    }
                    indexes.sort();
                    let valid = valid_indexes.contains(&indexes);
                    if !valid {
                        found = false;
                        break;
                    }
                    tmp = Some(d.clone());
                }
                if found {
                    break;
                }
            }
            if !found {
                panic!("no valid combination found");
            } else {
                let d: Display = tmp.unwrap();
                let x = line.1.split_ascii_whitespace().collect::<Vec<&str>>();
                let mut digits = Vec::new();
                for pat in x.iter() {
                    let mut indexes = Vec::new();
                    for c in pat.chars() {
                        indexes.push(d.iter().position(|x| *x == c).unwrap());
                    }
                    indexes.sort();
                    let digit = valid_indexes
                        .iter()
                        .position(|x| *x == indexes)
                        .expect("dumbass");
                    digits.push(digit.to_string());
                }
                let x = digits.join("");
                sol.push(x.parse().expect("bruh"));
            }
        }
        println!("sum: {}", sol.iter().sum::<i32>());
    }
}

type Display = [char; 7];

fn permutate<F>(array: &mut Display, size: usize, f: &mut F)
where
    F: FnMut(&mut Display),
{
    if size == 1 {
        f(array);
        return;
    }

    for i in 0..size {
        permutate(array, size - 1, f);

        if size % 2 == 1 {
            array.swap(0, size - 1);
        } else {
            array.swap(i, size - 1);
        }
    }
}
