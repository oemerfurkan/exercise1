/*

    - Given a list of integers, use a vector and return the
      median (when sorted, the value in the middle position) 
      and mode (the value that occurs most often; a hash map
      will be helpful here) of the list.
    
    - Convert strings to pig latin. The first consonant of each
      word is moved to the end of the word and “ay” is added, so 
      “first” becomes “irst-fay.” Words that start with a vowel 
      have “hay” added to the end instead (“apple” becomes 
      “apple-hay”). Keep in mind the details about UTF-8 encoding!

    - Using a hash map and vectors, create a text interface to allow 
      a user to add employee names to a department in a company. For 
      example, “Add Sally to Engineering” or “Add Amir to Sales.” Then 
      let the user retrieve a list of all people in a department or 
      all people in the company by department, sorted alphabetically.
*/

use std::collections::HashMap;

fn main() {
    let list_of_integers = [10, 23, 32, 5, 9, 4, 120, 120, 120, 120, 120,  5, 5, 5, 5];

    let vec = Vec::from(list_of_integers);

    let mode = calculate_mode(&vec);
    let median = calculate_median(&vec);

    println!("Mode: {mode:?}, Median: {median:?}");
    

}

fn calculate_median(l: &Vec<i32>) -> i32 {
    let mut vec: Vec<i32> = Vec::new();
    let len = l.len();

    vec = l.clone();

    vec.sort();

    println!("{vec:?}");

    if len % 2 == 0 {
        (vec[len / 2 - 1] + vec[len / 2 ]) / 2
    } else {
        vec[(len - 1) / 2]
    }
}

fn calculate_mode(l: &Vec<i32>) -> Vec<i32> {
    
    let mut vec: Vec<i32> = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();
    
    for item in l {
        let count = map.entry(*item).or_insert(0);
        *count += 1
    }
    
    let max = map.values().max().unwrap();

    for (key,_ ) in &map {
        if map.get(&key) == Some(&max) {
            let _ = &vec.push(*key);
        }
    }
    vec
}