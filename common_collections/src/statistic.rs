use std::vec::Vec;
use std::collections::HashMap;

pub fn mean(numbers: &Vec<i32>) -> i32 {
    let mut sum = 0;
    let len= numbers.len() as i32;
    for i in numbers {
        sum += i;
    };

    sum/len
}

pub fn median(numbers: &mut Vec<i32>) -> i32{
    numbers.sort();
    let len = numbers.len();
    let half = len/2;

    if len % 2 == 0{
        (numbers[half] + numbers[half-1])/2
    }else{
        numbers[half]
    }

}

pub fn mode(numbers: Vec<i32>) -> i32{
    let mut hash_map = HashMap::new();
    for i in numbers{
        let count = hash_map.entry(i).or_insert(0);
        *count += 1
    };

    let mut collected: Vec<_> = hash_map.iter().collect();
    collected.sort_by(|a,b| b.1.cmp(a.1));
    let a =match collected.first() {
        Some(x) => x.1,
        None => &-1
    };

    *a
}