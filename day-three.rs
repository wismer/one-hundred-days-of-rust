// functions!

fn increment(n: i32) -> i32 {
    n + 1
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}

fn function_that_returns_nothing_supposedly() -> () {
    println!("I better not return anything or I'm gonna be so mad.");
}

fn reduce(list: Vec<i32>) -> i32 {
    let mut n = 0;

    for number in list {
        n += number;
    }

    n
}

fn print_contents(list: Vec<String>) {
    for i in list {
        println!("{}", i);
    }
}

fn map_vector_into_strings(list: Vec<i32>) -> Vec<String> {
    let mut boolean_list = vec![];
    for number in list {
        if x < number {
            boolean_list.push("crap".to_string());
        }
    }

    boolean_list
}

fn main() {
    function_that_returns_nothing_supposedly();
    let number_to_increment = 10;
    let list: Vec<i32> = vec![10,34,12,75,123];
    // let greatest = loop_through(list);
    let bool_list = map_vector_into_strings(list);
    // let n = reduce(list);
    print_contents(bool_list);
    // println!("{}", n);
}
