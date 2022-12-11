use lib::{input::get_input_lines, year_day, Result};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let input = input.split(|s| s.is_empty()).collect::<Vec<_>>();
    let mut monkeys = Vec::<Monkey>::new();

    // parse le monky
    for monkey in input {
        let items = monkey[1]
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|n| n.trim().parse().unwrap());

        let mut op = monkey[2]
            .split_once('=')
            .unwrap()
            .1
            .split_ascii_whitespace();
        op.next();
        let o = op.next().unwrap();
        let n = op.next().unwrap().parse().ok();
        println!("{o} {n:?}");

        let mut test = monkey[3..]
            .iter()
            .map(|l| l.split_ascii_whitespace().last().unwrap().parse().unwrap());

        let m = Monkey {
            queue: items.collect(),
            add: o == "+",
            op_num: n,
            test_num: test.next().unwrap() as i32,
            next: (test.next().unwrap(), test.next().unwrap()),
            inspected: 0,
        };
        dbg!(&m);
        monkeys.push(m);
    }

    for round in 1..21 {
        for i in 0..monkeys.len() {
            eprintln!("\n===== monkey {i} =====");
            for j in 0..monkeys[i].queue.len() {
                let m = &mut monkeys[i];
                let item = m.queue[j];
                let w = m.calc(item) / 3;
                let next = m.get_next(w);
                eprintln!("item = {item}, w = {w}, next = {next}");
                monkeys[next].queue.push_back(w);
            }
            monkeys[i].queue.clear();
        }

        eprintln!("\n===== ROUND {round} OVER =====");
        monkeys
            .iter()
            .enumerate()
            .for_each(|(i, m)| eprintln!("Monkey {i}: {:?}", m.queue));
        eprintln!()
    }

    let mut insp = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    insp.sort_unstable();
    let monkey_business = insp
        .into_iter()
        .rev()
        .take(2)
        .reduce(i32::saturating_mul)
        .unwrap();
    println!("part 1: {monkey_business:?}");

    Ok(())
}

#[derive(Debug)]
struct Monkey {
    // id: usize,
    queue: VecDeque<i32>,
    add: bool,
    op_num: Option<i32>,
    test_num: i32,
    next: (usize, usize),
    inspected: i32,
}
impl Monkey {
    pub fn calc(&mut self, i: i32) -> i32 {
        self.inspected += 1;
        let n = self.op_num.unwrap_or(i);
        if self.add {
            i + n
        } else {
            i * n
        }
    }
    pub fn get_next(&self, i: i32) -> usize {
        if i % self.test_num == 0 {
            self.next.0
        } else {
            self.next.1
        }
    }
}
