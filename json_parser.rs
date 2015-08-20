use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::str;
use std::char;
use std::collections::HashMap;
use std::fmt::Debug;

struct Null;
struct Object;

#[derive(Debug)]
enum JSON {
    Obj { container: HashMap<String, JSON> },
    Arr { container: Vec<JSON> },
    Str(String),
    Int(i32),
    Bool(bool),
    None
}

impl JSON {
    fn len(&self) -> usize {
        match *self {
            JSON::Obj { ref container } => container.keys().len(),
            JSON::Arr { ref container } => container.len(),
            _ => panic!("what!?")
        }
    }

    fn get(&self, key: &str) -> &'static str {
        let json: &HashMap<String, JSON> = match *self {
            JSON::Obj { ref container } => container,
            _ => panic!("SDFKLSDJFLSKDJFSLDKFJ")
        };

        match *json.get(&key.to_string()).unwrap() {
            JSON::Str(ref x) => x,
            _ => panic!("omg")
        }
    }

    fn list(&self) -> Vec<String> {
        match *self {
            JSON::Arr { ref container } => container.iter().map(|&x| x.get("name") ).collect(),
            _ => panic!("sdfksd")
        }
    }
}

fn convert_escaped_character(buffer: &mut Iterator<Item=char>) -> char {
    let c = buffer.next().unwrap();
    match c {
        'n'  => '\n',
        'r'  => '\r',
        '"'  => '\"',
         _   => c
    }
}

fn parse_string(buffer: &mut Iterator<Item=char>) -> String {
    let mut str_obj = String::new();
    loop {
        let mut c = buffer.next().unwrap();
        println!("string what are you {}", c);
        if c == '\"' {
            break;
        }

        if c == '\\' {
            let next_char = convert_escaped_character(buffer);
            str_obj.push(next_char);
            println!("escaped character: {}", next_char);
        } else {
            str_obj.push(c);
        }
    }

    println!("this is a string value: {}", str_obj);

    str_obj
}

fn parse_null(buffer: &mut Iterator<Item=char>, c: char) -> JSON {
    let mut limit = 0;
    while limit != 4 {
        buffer.next();
        limit += 1;
    }
    JSON::None
}

fn parse_int(buffer: &mut Iterator<Item=char>, first: char) -> i32 {
    let mut num = String::new();
    num.push(first);
    for digit in buffer.take_while(|&x| x.is_numeric() ) {
        num.push(digit);
    }


    let skipped = buffer.next().unwrap();
    if skipped == ',' {
        buffer.next().unwrap();
    }

    let n: Result<i32, _> = num.parse::<i32>();
    n.unwrap()
}

fn parse_bool(buffer: &mut Iterator<Item=char>, b: bool) -> bool {


    let mut limit: usize = match b {
        true => 3,
        false => 4
    };

    while limit != 0 {
        let n = buffer.next().unwrap();
        limit -= 1;
    }

    b
}

fn parse_object(buffer: &mut Iterator<Item=char>) -> HashMap<String, JSON> {
    let mut hash: HashMap<String, JSON> = HashMap::new();
    let z = buffer.next();
    println!("obj");
    loop {
        let c = buffer.next().unwrap();

        if c == '}' {
            break;
        } else if c == ',' {
            // println!("comma");
        } else {
            println!("right before the key {}", c);
            let key = parse_key(buffer, c);
            println!("key: {}", key);
            let skipped = buffer.next().unwrap(); // skip colon
            if skipped != ':' {
                println!("{}", skipped);
                panic!("NOT");
            }
            let next_char = buffer.next().unwrap();
            println!("before value gets inserted {}", next_char);
            let mut json_obj = get_next_chunk(buffer, next_char);
            hash.insert(key, json_obj);
        }
    }
    hash
}

fn get_next_chunk(buffer: &mut Iterator<Item=char>, c: char) -> JSON {
    let mut next_char: char;

    if c == ',' {
        next_char = buffer.next().unwrap();
    } else {
        next_char = c;
    }
    println!("get next chunk: {}", next_char);

    match next_char {
        '{'  => JSON::Obj { container: parse_object(buffer) },
        '\"' => JSON::Str(parse_string(buffer)),
        '['  => JSON::Arr { container: parse_array(buffer) },
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => JSON::Int(parse_int(buffer, c)),
        'f' => JSON::Bool(parse_bool(buffer, false)),
        't' => JSON::Bool(parse_bool(buffer, true)),
         _ => parse_null(buffer, c)
    }
}

fn parse_array(buffer: &mut Iterator<Item=char>) -> Vec<JSON> {
    let mut arr: Vec<JSON> = vec![];
    let mut json_obj: JSON;


    loop {
        let mut c = buffer.next().unwrap();
        if c == ']' {
            println!("array loop broken");
            break;
        } else if c == ',' {
            println!("comma skipped");
        } else {
            json_obj = get_next_chunk(buffer, c);
            println!("pushed into array: {}", c);
            arr.push(json_obj);
        }
    }

    arr
}

fn parse_key(buffer: &mut Iterator<Item=char>, first: char) -> String {
    let mut key: String = String::new();
    if first != '\"' {
        key.push(first);
    }

    loop {
        let c = buffer.next().unwrap();
        if c == '\"' {
            break;
        }
        key.push(c);
    }

    key
}

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    let result = file.read_to_string(&mut buffer);
    let mut data = buffer.chars();
    let mut hash: HashMap<String, JSON> = parse_object(&mut data);
    // let mut json = JSON::Obj { container: parse_object(&mut data) };
    let mut total = 0;
    // for (k, h) in hash {
    //     println!("key: {} and length: {}", k, h.len());
    //     total += h.len();
    // }

    let results = hash.get("Blackrock Mountain").unwrap().list();

}
