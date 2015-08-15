use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

#[derive(Debug)]
enum JSON<T> {
    Thing(T),
    None
}

struct ObjString {
    val: HashMap<String, String>
}

#[derive(Debug)]
struct ObjInt {
    val: HashMap<String, usize>
}

#[derive(Debug)]
struct PrimeObj<T> {
    val: HashMap<String, T>
}

struct PrimeArray<T> {
    val: Vec<T>
}

#[derive(Debug)]
enum JsonTypes<T> {
    Arr(Vec<T>),
    Bool(i32),
    Str,
    Int(usize),
    Obj,
    None
}

trait JSONInsert {
    fn insert(&mut self, key: String, val: JsonTypes);
    fn get(&self, key: String) -> &JsonTypes;
}


impl JSONInsert for PrimeObj<JsonTypes> {
    fn insert(&mut self, key: String, val: JsonTypes) {
        self.val.insert(key, val);
    }

    fn get(&self, key: String) -> &JsonTypes {
        self.val.get(&key).unwrap()
    }
}

// whatever type is passed in, that type will respond to JsonMethod methods


fn main() {
    let json_file = Path::new("/Users/Matt/projects/ruby/hearthstone/public/data/AllSets.json");
    let mut file = File::open(json_file).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let mut z = buffer.chars();
    let mut c = z.next().unwrap();
    // hashmap that takes vector lists of i32 numbers
    // let mut prime_object_arrays: PrimeObj<PrimeArray<i32>> = PrimeObj { val: HashMap::new() };
    // let mut prime_object_ints: PrimeObj<i32> = PrimeObj { val: HashMap::new() };
    let mut prime_object_of_objects: PrimeObj<JsonTypes> = PrimeObj { val: HashMap::new() };
    let json_type: JsonTypes = JsonTypes::Bool(10);

    prime_object_of_objects.insert("ajsontype".to_string(), json_type);
    let retrieval = prime_object_of_objects.get("ajsontype".to_string());
    println!("{:?}", retrieval);
}





/*
when hashmap accesses THISKEY
an enum is pulled
that has
    array
    string
    hash
    etc

pattern match to get the right thinger
destructure.



hash
    THISKEY
        array
            hash
                key
                    string
                key
                    string
                key
                    string
            hash
                key
                key
    key
    key

*/
















//
// fn parse_string(chars: &mut Iterator<Item=char>) -> String {
//     chars.next().unwrap(); // skip initial quote
//     chars.take_while(|&x| x != '"' ).collect()
// }
//
// fn parse_int(chars: &mut Iterator<Item=char>) -> usize {
//     let n: String = chars.take_while(|&x| x.is_numeric() ).collect();
//     n.parse::<usize>().unwrap()
// }
//
// // fn parse_key(chars: &mut Iterator<Item=char>) -> String {
// //
// // }
//
// fn parse_object(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>) {
//
// }
//
// fn parse_string_value(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>, key: String) {
//     let value = chars.take_while(|&x| x != '"' ).collect();
//     hash.insert(key, value);
// }
//
// fn parse_array(chars: &mut Iterator<Item=char>, hash: &mut HashMap<String, String>) {
//     let next_object = chars.next().unwrap();
//
//     if next_object == '{' {
//         parse_object(chars, hash);
//         //
//         // let key = parse_key(chars);
//         // println!("{}", key);
//         // let next_object_type = chars.next().unwrap();
//         // println!("{}", next_object_type);
//     }
// }
