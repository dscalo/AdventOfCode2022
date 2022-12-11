use std::collections::VecDeque;

pub type Monkeys = Vec<Monkey>;

#[derive(Debug)]
pub struct Monkey {
    pub items: VecDeque<i64>,
    pub operation: fn(i64) -> i64,
    pub inspected: i64,
    pub divisible_by: i64,
    pub if_true: usize,
    pub if_false: usize,
}

impl Monkey {
    //    pub fn new() -> Monkey {
    //         let items = VecDeque::new();
    //         Monkey {
    //             items,
    //             operation: |a: i64, b: i64| a * b,
    //             divisible_by: 10,
    //             if_true: 2,
    //             if_false: 4,
    //         }
    //     }

    pub fn get_monkeys(data: &str) -> Vec<Monkey> {
        let mut monkeys = Monkeys::new();

        if data == "puzzle" {
          

            return monkeys;
        }

        monkeys.push(Monkey {
            items: VecDeque::from(vec![79, 98]),
            operation: |old: i64| old * 19,
            inspected: 0,
            divisible_by: 23,
            if_true: 2,
            if_false: 3,
        });

        monkeys.push(Monkey {
            items: VecDeque::from(vec![54, 65, 75, 74]),
            operation: |old: i64| old + 6,
            inspected: 0,
            divisible_by: 19,
            if_true: 2,
            if_false: 0,
        });

        monkeys.push(Monkey {
            items: VecDeque::from(vec![79, 60, 97]),
            operation: |old: i64| old * old,
            inspected: 0,
            divisible_by: 13,
            if_true: 1,
            if_false: 3,
        });

        monkeys.push(Monkey {
            items: VecDeque::from(vec![74]),
            operation: |old: i64| old + 3,
            inspected: 0,
            divisible_by: 17,
            if_true: 0,
            if_false: 1,
        });

        monkeys
    }
}
