// examine lifetimes
// &mut
// &ref
// *own (i think)

struct Collection {
    pub items: Vec<Answer>
}

impl Collection {
    fn show(&self) {
        for item in &self.items {
            println!("{:?}", item);
        }
    }

    fn get_answer_from(&self, idx: usize) -> Option<i32> {
        let number: i32;
        let answer = &self.items[idx];
        match *answer {
            Answer::Even(x) => Some(x),
            Answer::Odd(x) => Some(x),
            Answer::None => None
        }
    }
}

#[derive(Debug)]
enum Answer {
    Odd(i32),
    Even(i32),
    None
}

fn get_answer(items: &Vec<i32>) -> Collection {
    let mut collection = Collection { items: vec![] };
    for item in items {
        if *item == 0 {
            collection.items.push(Answer::None);
        } else if *item % 2 == 0 {
            collection.items.push(Answer::Even(*item));
        } else {
            collection.items.push(Answer::Odd(*item));
        }
    }

    collection
}

fn main() {
    let numbers: Vec<i32> = vec![1,2,3,4,5,0];
    let answers = get_answer(&numbers);
    let answer = answers.get_answer_from(0);
    answers.show();

    match answer {
        Some(x) => println!("{}", x),
        None => println!("WHAT")
    }
}
