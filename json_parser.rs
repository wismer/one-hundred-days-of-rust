use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

// STEP 1 - IDENTIFY THE TYPE OF PROPERTY THAT WILL BE PARSED
// either make methods for them
// enum JPrimitive {
//     Object { chunk: HashMap<String, JPrimitive> },
//     Array { chunk: Vec<JPrimitive> },
//     None
// }

// struct Object {
//     iter: Vec<char>
// }

// struct JSON<JPrimitive> {
//     payload: HashMap<String, JPrimitive>
// }

enum JPrimitive {
    Object { iter: Vec<char> },
    Array { iter: Vec<char> },
    Str { iter: Vec<char> },
    Int { iter: Vec<char> },
    Bool { iter: Vec<char> },
    None
}

struct JSON {
    payload: JPrimitive
}

impl JSON {
    fn new(payload: JPrimitive) -> JSON {
        JSON {
            payload: payload
        }
    }

    fn to_str(self) {
        match self.payload {
            JPrimitive::Object { iter } => parse_obj(iter),
            JPrimitive::Array { iter } => parse_array(iter),
            JPrimitive::Str { iter } => parse_str(iter),
            JPrimitive::Int { iter } => parse_int(iter),
            JPrimitive::Bool { iter } => parse_bool(iter),
            JPrimitive::None => what()
        }
    }
}

fn what() {
    println!("Assfsdfsfdsd");
}

fn parse_obj(chars: Vec<char>) {
    println!("OBJECT");
}

fn parse_array(chars: Vec<char>) {
}

fn parse_str(chars: Vec<char>) {
    chars.iter().take_while(|&x| x != '"' ).collect()
}

fn parse_int(chars: Vec<char>) {
}

fn parse_bool(chars: Vec<char>) {
}

fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    let data = file.read_to_string(&mut buffer);
    let mut chars = buffer.chars();
    let initial_char = chars.next().unwrap();
    let mut json = JSON::new(get_end_point(&mut chars, initial_char));
    json.to_str();
    // loop {
    //     let mut c = chars.next();
    //     if c.is_none() {
    //         break;
    //     }
    //     //
    //     {
    //         let n = &mut chars;
    //         let count = n.count();
    //         println!("{:?}", count);
    //     }
    //
    //     let obj = get_end_point(&mut chars, c.unwrap());
    //     to_str(obj);
    // }
}

fn to_str(obj: JPrimitive) {
    match obj {
        JPrimitive::Object { iter } => println!("OBJECT"),
        JPrimitive::Array { iter } => println!("ARRAY"),
        JPrimitive::Str { iter } => println!("STRING"),
        JPrimitive::Int { iter } => println!("INTEGER"),
        JPrimitive::Bool { iter } => println!("BOOLEAN"),
        JPrimitive::None => {}
    }
}

fn create_construct(obj: JPrimitive) {
    match obj {
        JPrimitive::Object { iter } => println!("OBJECT"),
        JPrimitive::Array { iter } => println!("ARRAY"),
        JPrimitive::Str { iter } => println!("STRING"),
        JPrimitive::Int { iter } => println!("INTEGER"),
        JPrimitive::Bool { iter } => println!("BOOLEAN"),
        JPrimitive::None => {}
    }
}

fn get_end_point(iter: &mut Chars, last_character: char) -> JPrimitive {
    match last_character {
        '{' => JPrimitive::Object { iter: iter.collect() },
        '[' => JPrimitive::Array { iter: iter.collect() },
        '"' => JPrimitive::Str { iter: iter.collect() },
        't' | 'f' => JPrimitive::Bool { iter: iter.collect() },
        '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => JPrimitive::Int { iter: iter.collect() },
         _ => JPrimitive::None
    }
}
