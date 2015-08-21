use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::str;
use std::char;
use std::collections::HashMap;
use std::fmt::Debug;

// this will handle the logic of parsing the json string
struct Parser;

// this will hold the data and allow for reading
struct JSON(JSONTypes);
// this will help disseminate the various types expected in JSON
enum JSONTypes {
    Obj { container: HashMap<String, JSONTypes> },
    Arr { container: Vec<JSONTypes> },
    Str(String),
    Int(i32),
    Bool(bool),
    None
}

impl Parser {
    fn new() -> Parser { Parser }

    fn load(&self, buffer: &mut Iterator<Item=char>) -> JSON {
        // start parsing
        let mut json: JSON = JSON(self.parse(buffer));
        json
    }

    fn parse(&self, buffer: &mut Iterator<Item=char>) -> JSONTypes {
        let mut c = buffer.next().unwrap();
        // println!("{}", c);
        match c {
            '{'  => self.parse_object(buffer),
            '\"' => self.parse_string(buffer),
            '['  => self.parse_array(buffer),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.parse_int(buffer, c),
            'f'  => self.parse_bool(buffer, false),
            't'  => self.parse_bool(buffer, true),
             _   => self.parse_null(buffer)
        }
    }

    fn parse_string(&self, buffer: &mut Iterator<Item=char>) -> JSONTypes {
        let mut container = String::new();
        loop {
            let next_char = buffer.next().unwrap();
            if next_char == '\"' {
                break;
            }

            if next_char == '\\' {
                let escape = buffer.next().unwrap();
                match escape {
                    'n'  => container.push('\n'),
                    'r'  => container.push('\r'),
                    '"'  => container.push('\"'),
                     _   => container.push(escape)
                }
            } else {
                container.push(next_char);
            }
        }

        JSONTypes::Str(container)
    }

    fn parse_int(&self, buffer: &mut Iterator<Item=char>, first: char) -> JSONTypes {
        let mut num = String::new();
        num.push(first);

        for digit in buffer.take_while( |&x| x.is_numeric() ) {
            // println!("digit: {}, first: {}", digit, first);
            num.push(digit);
        }

        let n: Result<i32, _> = num.parse::<i32>();
        let int = n.unwrap();
        // println!("digit: {}", int);
        JSONTypes::Int(int)
    }

    fn parse_null(&self, buffer: &mut Iterator<Item=char>) -> JSONTypes {
        let mut limit = 0;
        while limit != 4 {
            buffer.next();
            limit += 1;
        }

        JSONTypes::None
    }

    fn parse_bool(&self, buffer: &mut Iterator<Item=char>, b: bool) -> JSONTypes {
        let mut limit: usize = match b {
            true => 3,
            false => 4
        };

        while limit != 0 {
            let n = buffer.next().unwrap();
            limit -= 1;
        }

        JSONTypes::Bool(b)
    }

    fn parse_array(&self, buffer: &mut Iterator<Item=char>) -> JSONTypes {
        let mut array: Vec<JSONTypes> = vec![];
        let mut next_char = buffer.next().unwrap();
        if next_char == ',' {
            next_char = buffer.next().unwrap();
        }

        loop {
            println!("char for array {}", next_char);
            if next_char == ']' {
                break;
            } else {
                let value = self.parse(buffer);
                array.push(value);
            }
        }
        JSONTypes::Arr { container: array }
    }

    fn parse_object(&self, buffer: &mut Iterator<Item=char>) -> JSONTypes {
        let mut jsobject: HashMap<String, JSONTypes> = HashMap::new();
        buffer.next();
        loop {
            let key = buffer.take_while( |&x| x != '\"' ).collect();
            let a = buffer.next();
            // println!("key {}, a {}", key, a.unwrap());
            let value: JSONTypes = self.parse(buffer);
            jsobject.insert(key, value);
            let mut end_point = buffer.next().unwrap();

            if end_point == ',' {
                end_point = buffer.next().unwrap();
            }

            if end_point == '}' {
                // println!("END OBJECT");
                break;
            }
        }

        JSONTypes::Obj { container: jsobject }
    }
}
//
// fn convert_escaped_character(buffer: &mut Iterator<Item=char>) -> char {
//     let c = buffer.next().unwrap();
//     match c {
//         'n'  => '\n',
//         'r'  => '\r',
//         '"'  => '\"',
//          _   => c
//     }
// }
//
// fn parse_string(buffer: &mut Iterator<Item=char>) -> String {
//     let mut str_obj = String::new();
//     loop {
//         let mut c = buffer.next().unwrap();
//         println!("string what are you {}", c);
//         if c == '\"' {
//             break;
//         }
//
//         if c == '\\' {
//             let next_char = convert_escaped_character(buffer);
//             str_obj.push(next_char);
//             println!("escaped character: {}", next_char);
//         } else {
//             str_obj.push(c);
//         }
//     }
//
//     println!("this is a string value: {}", str_obj);
//
//     str_obj
// }
//
// fn parse_null(buffer: &mut Iterator<Item=char>, c: char) -> JSON {
//     let mut limit = 0;
//     while limit != 4 {
//         buffer.next();
//         limit += 1;
//     }
//     JSON::None
// }
//
// fn parse_int(buffer: &mut Iterator<Item=char>, first: char) -> i32 {
//     let mut num = String::new();
//     num.push(first);
//     for digit in buffer.take_while(|&x| x.is_numeric() ) {
//         num.push(digit);
//     }
//
//
//     let skipped = buffer.next().unwrap();
//     if skipped == ',' {
//         buffer.next().unwrap();
//     }
//
//     let n: Result<i32, _> = num.parse::<i32>();
//     n.unwrap()
// }
//
// fn parse_bool(buffer: &mut Iterator<Item=char>, b: bool) -> bool {
//
//
//     let mut limit: usize = match b {
//         true => 3,
//         false => 4
//     };
//
//     while limit != 0 {
//         let n = buffer.next().unwrap();
//         limit -= 1;
//     }
//
//     b
// }
//
// fn parse_object(buffer: &mut Iterator<Item=char>) -> HashMap<String, JSON> {
//     let mut hash: HashMap<String, JSON> = HashMap::new();
//     let z = buffer.next();
//     println!("obj");
//     loop {
//         let c = buffer.next().unwrap();
//
//         if c == '}' {
//             break;
//         } else if c == ',' {
//             // println!("comma");
//         } else {
//             println!("right before the key {}", c);
//             let key = parse_key(buffer, c);
//             println!("key: {}", key);
//             let skipped = buffer.next().unwrap(); // skip colon
//             if skipped != ':' {
//                 println!("{}", skipped);
//                 panic!("NOT");
//             }
//             let next_char = buffer.next().unwrap();
//             println!("before value gets inserted {}", next_char);
//             let mut json_obj = get_next_chunk(buffer, next_char);
//             hash.insert(key, json_obj);
//         }
//     }
//     hash
// }
//
// fn get_next_chunk(buffer: &mut Iterator<Item=char>, c: char) -> JSON {
//     let mut next_char: char;
//
//     if c == ',' {
//         next_char = buffer.next().unwrap();
//     } else {
//         next_char = c;
//     }
//     println!("get next chunk: {}", next_char);
//
//     match next_char {
//         '{'  => JSON::Obj { container: parse_object(buffer) },
//         '\"' => JSON::Str(parse_string(buffer)),
//         '['  => JSON::Arr { container: parse_array(buffer) },
//         '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => JSON::Int(parse_int(buffer, c)),
//         'f' => JSON::Bool(parse_bool(buffer, false)),
//         't' => JSON::Bool(parse_bool(buffer, true)),
//          _ => parse_null(buffer, c)
//     }
// }
//
// fn parse_array(buffer: &mut Iterator<Item=char>) -> Vec<JSON> {
//     let mut arr: Vec<JSON> = vec![];
//     let mut json_obj: JSON;
//
//
//     loop {
//         let mut c = buffer.next().unwrap();
//         if c == ']' {
//             println!("array loop broken");
//             break;
//         } else if c == ',' {
//             println!("comma skipped");
//         } else {
//             json_obj = get_next_chunk(buffer, c);
//             println!("pushed into array: {}", c);
//             arr.push(json_obj);
//         }
//     }
//
//     arr
// }
//
// fn parse_key(buffer: &mut Iterator<Item=char>, first: char) -> String {
//     let mut key: String = String::new();
//     if first != '\"' {
//         key.push(first);
//     }
//
//     loop {
//         let c = buffer.next().unwrap();
//         if c == '\"' {
//             break;
//         }
//         key.push(c);
//     }
//
//     key
// }

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut data = String::new();
    // load the json data into `data`
    file.read_to_string(&mut data);
    // instantiate the parser
    let parser: Parser = Parser::new();
    // convert buffer string into an iterator of characters
    let mut buffer = data.chars();
    // load buffer into parser
    let json: JSON = parser.load(&mut buffer);
}
