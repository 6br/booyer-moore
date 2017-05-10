#![feature(test)]
extern crate rand;
extern crate test;

use std::collections::HashMap;
use rand::Rng;

fn main() {
    //let seed = "UACCUA";
    //let target = "UACATTUCACUACCUACCUAAACCUA";
    let seed = "UACCUA";//.as_bytes();
    let target = &generate_random_sequence(10000);
    let seed_bytes = seed.as_bytes();
    let vec = convert_str_to_array(seed_bytes);
    /*for (contact, number) in hashmap.iter() {
        println!("Calling {}: {}", contact, number);
    }*/
    let count = bm_search_vec(target.as_bytes(), seed_bytes, vec);

    println!("{}", count);
}

mod tests{
    use super::*;
    use test::Bencher;
    #[test]
    fn test() {
        let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let target = &generate_random_sequence(10000);
        let target2 = target.as_bytes();
        assert_eq!(bruteforce(target2, seed), bruteforce_break(target2, seed));
        assert_eq!(
            bm_search(target2, seed, convert_str_to_hashmap(seed))
            , bruteforce_break(target2, seed));
        assert_eq!(
            bm_search_vec(target2, seed, convert_str_to_array(seed))
                , bruteforce_break(target2, seed));
    }

    #[bench]
    fn bench_brutefore_break_1000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(1000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce_break(target2, seed)
        });
    }

    #[bench]
    fn bench_bm_1000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(1000);
        let target2 = target.as_bytes();
        b.iter(|| {
            //let hashmap = convert_str_to_hashmap(target2);
            bm_search(target2, seed, convert_str_to_hashmap(seed))
        });
    }

    #[bench]
    fn bench_brutefore_1000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(1000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce(target2, seed)
        });
    }

    #[bench]
    fn bench_bm_vec_1000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(1000);
        let target2 = target.as_bytes();
        b.iter(|| {
            //let hashmap = convert_str_to_hashmap(target2);
            bm_search_vec(target2, seed, convert_str_to_array(seed))
        });
    }


    #[bench]
    fn bench_bm_10000(b: &mut Bencher) {
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(10000);
        let target2 = target.as_bytes();
        b.iter(|| {
            //let hashmap = convert_str_to_hashmap(target2);
            bm_search(target2, seed, convert_str_to_hashmap(seed))
        });
    }

    #[bench]
    fn bench_bm_vec_10000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(10000);
        let target2 = target.as_bytes();
        b.iter(|| {
            //let hashmap = convert_str_to_hashmap(target2);
            bm_search_vec(target2, seed, convert_str_to_array(seed))
        });
    }

    #[bench]
    fn bench_bm_vec_100000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(100000);
        let target2 = target.as_bytes();
        b.iter(|| {
            //let hashmap = convert_str_to_hashmap(target2);
            bm_search_vec(target2, seed, convert_str_to_array(seed))
        });
    }

    #[bench]
    fn bench_brutefore_10000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(10000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce(target2, seed)
        });
    }

    #[bench]
    fn bench_bm_100000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(100000);
        let target2 = target.as_bytes();
        b.iter(|| {
               //let hashmap = convert_str_to_hashmap(target2);
               bm_search(target2, seed, convert_str_to_hashmap(seed))
        });
    }

    #[bench]
    fn bench_brutefore_100000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(100000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce(target2, seed)
        });
    }

    #[bench]
    fn bench_brutefore_break_10000(b: &mut Bencher) {
        //let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let seed = "UACCUA".as_bytes();
        let target = &generate_random_sequence(10000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce_break(target2, seed)
        });
    }

    #[bench]
    fn bench_brutefore_break_100000(b: &mut Bencher) {
        let seed = "UACCUAACCCAAUUUUUUUUUAA".as_bytes();
        let target = &generate_random_sequence(100000);
        let target2 = target.as_bytes();
        b.iter(|| {
            bruteforce_break(target2, seed)
        });
    }
}

fn generate_random_sequence(length: i32) -> String {
    const CHARS: &'static [u8] = b"AUGC";
    let mut rng = rand::thread_rng();
    let id: String = (0..length).map(|_| *rng.choose(&CHARS).unwrap() as char)
        .collect();
    return id;
}

fn convert_str_to_hashmap(seed: &[u8]) -> HashMap<&u8, usize> {
    let mut map: HashMap<&u8, usize> = HashMap::new();
    for (i, v) in seed.into_iter().enumerate() {
        map.insert(v, i);
    }
    return map;
}

fn bit_encoding(chara: u8) -> usize {
    return (((chara >> 2) ^ (chara >> 1)) & 3) as usize;
}

fn convert_str_to_array(seed: &[u8]) -> [usize; 5] {
    //let m = seed.len();
    let mut ary: [usize; 5] = [0,0,0,0,0];
    for (i, v) in seed.into_iter().enumerate() {
        let ptr = bit_encoding(*v);
        ary[ptr] = i;
    }
    return ary;
}

fn bm_search_vec(target: &[u8], seed: &[u8], table: [usize; 5]) -> i64 {
    let n = target.len();
    let m = seed.len();
    let mut i = 0;
    let mut counter = 0;
    while i < n-m+1 {
        let mut j = m-1;
        while target[i+j] == seed[j] && j>0 {
            j -= 1;
        }
        if j == 0 && target[i] == seed[0] {
            counter += 1;
            i += 1;
        } else {
            //println!("{} {}", m, table[bit_encoding(target[i+j])]);
            i += m - table[bit_encoding(target[i+j])];
        }
    }
    return counter;
}

fn bm_search(target: &[u8], seed: &[u8], table: HashMap<&u8, usize>) -> i64 {
    let n = target.len();
    let m = seed.len();
    let mut i = 0;
    let mut counter = 0;
    while i < n-m+1 {
        let mut j = m-1;
        while target[i+j] == seed[j] && j>0 {
            j -= 1;
        }
        if j == 0 && target[i] == seed[0] {
            counter += 1;
            i += 1;
        } else {
            i += match table.get(&target[i+j]) {
                Some(value) => m - *value,
                None => m
            }
        }
    }
    return counter;
}

fn bruteforce(target: &[u8], seed: &[u8]) -> i64 {
    let n = target.len()-1;
    let m = seed.len()-1;
    let mut counter = 0;
    for i in 0..n-m+1 {
        let mut boolean = true;
        for j in 0..m {
            if target[i+j] != seed[j] {
                boolean = false;
            }
        }
        if boolean {
            counter += 1;
        }
    }
    return counter;
}

fn bruteforce_break(target: &[u8], seed: &[u8]) -> i64 {
    let n = target.len()-1;
    let m = seed.len()-1;
    let mut counter = 0;
    'outer: for i in 0..n-m+1 {
        let mut boolean = true;
        'inner: for j in 0..m {
            if target[i+j] != seed[j] {
                boolean = false;
                break 'inner;
            }
        }
        if boolean {
            counter += 1;
        }
    }
    return counter;
}

