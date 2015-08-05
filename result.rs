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

// fn get_pivot<'a>(pivot: &'a Option<i32>) -> i32 {
//     match pivot {
//         Some(x) => x,
//         None => 0i32
//     },
// }

fn sort_vec<'a>(numbers: Vec<&'a i32>, pivot: &'a Option<i32>) -> Vec<&'a i32> {
    if pivot.is_none() {
        return numbers
    }

    let p = pivot.expect("nothing");

    // below here, pivot should be

    let mut gt: Vec<&i32> = vec![];
    let mut lt: Vec<&i32> = vec![];

    for num in numbers {
        if num > &p {
            gt.push(num);
        } else {
            lt.push(num);
        }
    }
    lt.push(&p);
    for n in gt {
        lt.push(n);
    }
    lt
}

fn main() {
    let random_numbers = vec![10, 100, 34, 23, 18, 22, 11, 85, 101];
    let even = even_numbers(random_numbers);

    for n in even {
        println!("{}", n);
    }

    let random_numbers: Vec<i32> = vec![10, 100, 34, 23, 18, 22, 11, 85, 101];
    let string_numbers: Vec<String> = mutable_to_string(&random_numbers);

    for n in string_numbers {
        println!("{}", n);
    }
}
