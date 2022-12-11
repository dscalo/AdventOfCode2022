use crate::monkey::*;
mod monkey;

fn monkey_print(monkeys: &Monkeys) {
    for i in 0..monkeys.len(){
        println!("{}. items: {:?}",i, monkeys[i].items);
    }

    for i in 0..monkeys.len() {
        println!("Monkey {} inspected {} ", i, monkeys[i].inspected);
    }

    
    for i in 0..monkeys.len() {
        println!("Monkey {} inspected {} ", i, monkeys[i].inspected);
    }
}

fn part1(monkeys: &mut Monkeys) -> i64 {
    //let mut monkeys = ms.clone();

    for r in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;
                let f =  monkeys[i].operation;
                let wl = f(item);
                let reduced = (wl as f64 / 3.0) as i64;

                if reduced % monkeys[i].divisible_by == 0 {
                    let m = monkeys[i].if_true;
                    monkeys[m].items.push_back(reduced);
                } else {
                    let m = monkeys[i].if_false;
                    monkeys[m].items.push_back(reduced);
                }
            }
        }

       

    }

    monkey_print(&monkeys);

    let mut inspected = vec![];

    for monkey in monkeys {
        inspected.push(monkey.inspected);
    }

    inspected.sort();

    inspected[inspected.len()-1] * inspected[inspected.len() - 2]
}

fn part2(monkeys: &mut Monkeys) -> i64 {
    //let mut monkeys = ms.clone();

    let mut divisor = 1;

    for i in 0..monkeys.len() {
        divisor *= monkeys[i].divisible_by;
    }


    for r in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.pop_front().unwrap();
                monkeys[i].inspected += 1;
                let f =  monkeys[i].operation;
                let wl = f(item);
                let reduced = wl % divisor; // (wl as f64 / 3.0) as i64;

                if reduced % monkeys[i].divisible_by == 0 {
                    let m = monkeys[i].if_true;
                    monkeys[m].items.push_back(reduced);
                } else {
                    let m = monkeys[i].if_false;
                    monkeys[m].items.push_back(reduced);
                }
            }
        }

       

    }

    monkey_print(&monkeys);

    let mut inspected = vec![];

    for monkey in monkeys {
        inspected.push(monkey.inspected);
    }

    inspected.sort();

    inspected[inspected.len()-1] * inspected[inspected.len() - 2]
}

fn main() {
    let mut monkeys = Monkey::get_monkeys("test");


    let p1 = part1(&mut monkeys);    
    println!("Part 1 {}", p1);

    monkeys = Monkey::get_monkeys("puzzle");
    let p2 = part2(&mut monkeys);    
    println!("Part 2 {}", p2);

}
