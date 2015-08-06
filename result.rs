// fn quicksort(numbers: &Vec<i32>) -> Vec<i32> {
    // let mut gt: Vec<i32> = vec![];
    // let mut lt: Vec<i32> = vec![];
    // let mut nums = numbers.clone();
    // let pivot = nums.pop();
    //
    // if pivot.is_some() {
    //     for n in nums {
    //         if n > pivot.unwrap() {
    //             gt.push(n);
    //         } else {
    //             lt.push(n);
    //         }
    //     }
    //     nums
    // } else {
    //     nums
    // }
// }

fn even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    let mut even: Vec<i32> = vec![];

    for n in numbers {
        if n % 2 == 0 {
            even.push(n);
        }
    }

    even
}

fn mutable_to_string(numbers: &Vec<i32>) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    for n in numbers {
        if n % 2 == 0 {
            strings.push("even".to_string())
        } else {
            strings.push("odd".to_string())
        }
    }

    strings
}

fn sort_vec(nums: &Vec<i32>) -> Vec<i32> {
    let mut numbers = nums.clone();
    let mut greater_than = vec![];
    let mut less_than = vec![];

    if numbers.len() < 2 {
        return numbers
    }

    let pivot = get_piv(numbers.pop());

    {
        let gt = &mut greater_than;
        let lt = &mut less_than;

        for n in &numbers {
            if n > &pivot {
                gt.push(*n);
            } else {
                lt.push(*n);
            }
        }
    }

    let mut lesser = sort_vec(&less_than);
    let mut greater = sort_vec(&greater_than);

    lesser.push(pivot);

    for n in greater {
        lesser.push(n);
    }

    lesser
}

fn get_piv<T>(n: Option<T>) -> T {
    match n {
        Some(x) => x,
        None => panic!("NOOO!!!!!")
    }
}

fn main() {
    let mut random_numbers = vec![10, 100, 34, 23, 18, 22, 11, 85, 29];
    let lesser = sort_vec(&random_numbers);
    for n in lesser {
        println!("{}", n);
    }
}
