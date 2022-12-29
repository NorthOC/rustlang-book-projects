use std::collections::HashMap;

fn main() {

    let mut v = vec![12, 25, 8, 1, 13, 12, 69, 6];
    let length = v.len();

    v.sort();

    println!("{:?}", v);

    if length % 2 == 0 {
        let median = (v[length / 2] + v[length / 2 - 1]) / 2;
        println!("Median: {median}");
    } else {
        let median = v[length / 2];
        println!("Median: {median}");
    };

    let mut map = HashMap::new();

    for item in v{
        let count = map.entry(item).or_insert(0);
        *count += 1;
    }

    let biggest = map.iter().max_by_key(|&(_, count)| count).unwrap();
    let key = biggest.0;
    let value = biggest.1;

    println!("Most repeated number ({value} times) was {key}");

}
