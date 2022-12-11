use lib::{ddbg, dprintln, input::get_input_lines, year_day, Result};
use std::collections::VecDeque;

fn main() -> Result<()> {
    let input = get_input_lines(year_day!())?;
    let mut monkeys = Vec::<Monkey>::new();

    // parse le monky
    for monkey in input.split(|s| s.is_empty()) {
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
        dprintln!("{o} {n:?}");

        let mut test = monkey[3..]
            .iter()
            .map(|l| l.split_ascii_whitespace().last().unwrap().parse().unwrap());

        let m = Monkey {
            queue: items.collect(),
            add: o == "+",
            op_num: n,
            test_num: test.next().unwrap() as u64,
            next: (test.next().unwrap(), test.next().unwrap()),
            inspected: 0,
        };
        ddbg!(&m);
        monkeys.push(m);
    }

    let bigmod = monkeys.iter().map(|m| m.test_num).product::<u64>();

    let p1 = solve(monkeys.clone(), 20, |x| x / 3);
    let p2 = solve(monkeys, 10000, |x| x % bigmod);
    println!("part 1: {p1}\npart 2: {p2}");
    Ok(())
}

fn solve(mut monkeys: Vec<Monkey>, rounds: usize, calc: impl Fn(u64) -> u64) -> u64 {
    for _round in 1..=rounds {
        for i in 0..monkeys.len() {
            dprintln!("\n===== monkey {i} =====");
            for j in 0..monkeys[i].queue.len() {
                let m = &mut monkeys[i];
                let item = m.queue[j];
                let w = calc(m.calc(item));
                let next = m.get_next(w);
                dprintln!("item = {item}, w = {w}, next = {next}");
                monkeys[next].queue.push_back(w);
            }
            monkeys[i].queue.clear();
        }

        dprintln!("\n===== ROUND {_round} OVER =====");
        #[cfg(debug_assertions)]
        monkeys
            .iter()
            .enumerate()
            .for_each(|(i, m)| dprintln!("Monkey {i}: {:?}", m.queue));
        dprintln!()
    }

    let mut insp = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    insp.sort_unstable();
    insp.into_iter().rev().take(2).product::<u64>()
}

#[derive(Debug, Clone)]
struct Monkey {
    queue: VecDeque<u64>,
    add: bool,
    op_num: Option<u64>,
    test_num: u64,
    next: (usize, usize),
    inspected: u64,
}
impl Monkey {
    pub fn calc(&mut self, i: u64) -> u64 {
        self.inspected += 1;
        let n = self.op_num.unwrap_or(i);
        if self.add {
            i + n
        } else {
            i * n
        }
    }
    pub fn get_next(&self, i: u64) -> usize {
        if i % self.test_num == 0 {
            self.next.0
        } else {
            self.next.1
        }
    }
}
