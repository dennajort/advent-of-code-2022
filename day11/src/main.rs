use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug,Clone)]
enum InspectOp {
    Add(u64),
    Mul(u64),
    MulOld,
}

impl InspectOp {
    pub fn from(input: &str) -> Self {
        if input == "old * old" {
            return Self::MulOld;
        }
        if let Some(val) = input.strip_prefix("old + ") {
            let val = val.parse().unwrap();
            return Self::Add(val);
        }
        if let Some(val) = input.strip_prefix("old * ") {
            let val = val.parse().unwrap();
            return Self::Mul(val);
        }
        panic!("cannot parse operation '{input}'");
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            InspectOp::Add(v) => old + v,
            InspectOp::Mul(v) => old * v,
            InspectOp::MulOld => old * old,
        }
    }
}

#[derive(Debug,Clone)]
struct Monkey {
    items: VecDeque<u64>,
    op: InspectOp,
    test_val: u64,
    monkey_true: usize,
    monkey_false: usize,
    inspect_count: usize,
}

impl Monkey {
    fn inspect(&mut self, man: &dyn Fn(u64) -> u64) -> Inspected {
        self.inspect_count+=1;
        let item = self.items.pop_front().unwrap();
        let item = man(self.op.apply(item));
        let target_monkey = if item % self.test_val == 0 {
            self.monkey_true
        } else {
            self.monkey_false
        };
        return Inspected { item, target_monkey };
    }

    fn has_item(&self) -> bool { !self.items.is_empty() }

    fn give(&mut self, item: u64) { self.items.push_back(item) }
}

struct Inspected {
    item: u64,
    target_monkey: usize,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut lines = io::BufReader::new(file).lines();

    let mut monkeys = Vec::new();

    loop {
        let line = lines.next().unwrap().unwrap();
        if !line.starts_with("Monkey ") {
            panic!("expecting 'Monkey *:' got '{line}'");
        }
        let line = lines.next().unwrap().unwrap();
        let items = match line.strip_prefix("  Starting items: ") {
            None => panic!("expecting 'Starting items:' got '{line}'"),
            Some(items) => items.split(", ").map(|i| i.parse().unwrap()).collect(),
        };
        let line = lines.next().unwrap().unwrap();
        let op = match line.strip_prefix("  Operation: new = ") {
            None => panic!("expecting 'Operation:' got '{line}'"),
            Some(op) => InspectOp::from(op),
        };
        let line = lines.next().unwrap().unwrap();
        let test_val = match line.strip_prefix("  Test: divisible by ") {
            None => panic!("expecting 'Operation:' got '{line}'"),
            Some(test_val) => test_val.parse().unwrap(),
        };
        let line = lines.next().unwrap().unwrap();
        let monkey_true = match line.strip_prefix("    If true: throw to monkey ") {
            None => panic!("expecting 'Operation:' got '{line}'"),
            Some(monkey_true) => monkey_true.parse().unwrap(),
        };
        let line = lines.next().unwrap().unwrap();
        let monkey_false = match line.strip_prefix("    If false: throw to monkey ") {
            None => panic!("expecting 'Operation:' got '{line}'"),
            Some(monkey_false) => monkey_false.parse().unwrap(),
        };
        let monkey = Monkey{items, op, test_val, monkey_true, monkey_false, inspect_count: 0};
        monkeys.push(monkey);
        let line = lines.next();
        if line.is_none() { break; }
    }

    let monkeys_bck = monkeys.clone();

    for _ in 0..20 {
        for n in 0..monkeys.len() {
            while monkeys[n].has_item() {
                let item = monkeys[n].inspect(&|v| v / 3);
                monkeys[item.target_monkey].give(item.item);
            }
        }
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspects.sort_unstable_by(|a,b| b.cmp(a));
    let monkey_business = inspects[0] * inspects[1];

    println!("part 1 result {}", monkey_business);

    let mut monkeys = monkeys_bck;
    let monkey_mod = monkeys.iter().fold(1, |a,m| a*m.test_val);
    let manager = &|v| v % monkey_mod;

    for _ in 0..10000 {
        for n in 0..monkeys.len() {
            while monkeys[n].has_item() {
                let item = monkeys[n].inspect(manager);
                monkeys[item.target_monkey].give(item.item);
            }
        }
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|m| m.inspect_count).collect();
    inspects.sort_unstable_by(|a,b| b.cmp(a));
    let monkey_business = inspects[0] * inspects[1];

    println!("part 2 result {}", monkey_business);

    Ok(())
}
