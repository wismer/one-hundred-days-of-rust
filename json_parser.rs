use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

// match letter {
//     '{'  => parse_object::<String>(iter, i),
//     '\"' => parse_string(iter, i),
//     '['  => parse_array::<String>(iter, i),
//     'n'  => parse_null(iter, i),
//     't' | 'f' => parse_bool(iter, i),
//      _ => if letter.is_numeric(8) {
//          parse_number(iter, i)
//      } else {
//          // something?
//      }
// }


enum Parser<T> {
    Object { iter: Vec<char>, hash: HashMap<String, T> },
    Array { iter: Vec<char>, array: Vec<T> },
    Str,
    Int,
    Bool,
    None
}

enum JSON {
    Obj,
    Arr,
    Str,
    Int,
    Bool,
    None
}

fn convert_escaped_character(c: &char) -> char {
    match *c {
        'n'  => '\n',
        'r'  => '\r',
        '"'  => '\"',
         _   => *c
    }
}

fn parse_bool(c: &char, i: &mut usize) -> bool {
    if *c == 't' {
        *i += 4;
        true
    } else {
        *i += 5;
        false
    }
}

fn parse_number(chunk: &mut Iterator<Item=char>) -> usize {
    let stringified: String = chunk.take_while(|&x| x.is_digit(8) ).collect();
    stringified.parse::<usize>().unwrap()
}

fn parse_string(iter: &mut Iterator<Item=char>, i: &mut usize) -> String {
    let mut string_chunk = String::new();
    loop {
        let chunk = &mut string_chunk;
        let c = iter.next();
        if c.is_none() {
            break;
        }
        let unwrapped = c.unwrap();

        if unwrapped == '\"' {
            break;
        }


        if unwrapped == '\\' {
            *i += 1;
            chunk.push(convert_escaped_character(&unwrapped));
        } else {
            chunk.push(unwrapped);
        }

        *i += 1;
    }

    if iter.next().unwrap() == '\"' {
        *i += 1;
    }

    string_chunk.to_string()
}

fn parse_array<T>(iter: &mut Iterator<Item=char>, i: &mut usize) -> Vec<T> {
    let mut array: Vec<T> = vec![];
    // let mut nested_level = 1;
    // loop {
    //     let c = iter.next().unwrap();
    //     if c == ']' {
    //         nested_level -= 1;
    //         if nested_level == 0 {
    //             break;
    //         }
    //     } else {
    //         let parsed_object = parse(iter, i);
    //         array.push(parsed_object);
    //         let mut peekable_iter = iter.cloned().peekable();
    //         if *peekable_iter.peek().unwrap() == ',' {
    //             *i += 2;
    //         } else {
    //             *i += 1;
    //         }
    //     }
    // }
    //
    // *i += 1;
    array
}
//  brett victor
fn parse<T>(iter: &mut Iterator<Item=char>, i: &mut usize) {
    let c = iter.next();
    if c.is_none() {
        return;
    } else {
        let letter = c.unwrap();
        let letters = iter.collect();
        let parser_obj = match letter {
            '{'  => Parser::Object::<T> { iter: letters, hash: HashMap::new() },
            '['  => Parser::Array::<T> { iter: letters, array: vec![] },
            '\"' => Parser::Str,
            'n'  => Parser::None,
            't' | 'f' => Parser::Bool,
             _   => Parser::Int
        };
    }
}

fn fast_forward(iter: &mut Iterator<Item=char>, i: &mut usize) {
    iter.take_while( |&x| x != '\"' );
    {}
}

fn parse_key(iter: &mut Iterator<Item=char>, i: &mut usize) -> String {
    let mut key = String::new();
    fast_forward(iter, i);

    loop {
        let c = iter.next().unwrap();
        if c == '\"' {
            break;
        }
        key.push(c);
    }

    key
}

fn parse_object<T>(iter: &mut Iterator<Item=char>, i: &mut usize) -> HashMap<String, T>{
    let mut hash = HashMap::new();
    // let mut nested_level = 1;
    // while nested_level != 0 {
    //
    // }
    hash
}

fn parse_null(iter: &mut Iterator<Item=char>, i: &mut usize) -> String {
    *i += 4;
    "null".to_string()
}

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut z = buffer.chars();
    let mut c = z.next().unwrap();
    let escape = 'n';
    let n = convert_escaped_character(&escape);
    let mut num: usize = 10;
    parse::<JSON>(&mut z, &mut num);
}
