
use std::collections::HashSet;

pub fn solve(input: String) -> (i32, i32) {
    let input_split: Vec<i32> = input.split("-")
        .map(|n| n.parse().unwrap())
        .collect();
    let (min, max) = (input_split[0], input_split[1]);
    let (mut p1, mut p2) = (0, 0);
    for x in min..=max {
        if check_p1(x) {
            p1 += 1;
            if check_p2(x) {
                p2 += 1;
            }
        }
    }
    (p1, p2)
}

fn check_p1(n: i32) -> bool {
    let mut has_double = false;
    let mut last = n % 10;
    let mut m = n;
    while m > 0 {
        m /= 10;
        let k = m % 10;
        if k > last {
            return false;
        }
        if k == last {
            has_double = true;
        }
        last = k;
    }
    has_double
}

fn check_p2(n: i32) -> bool {
    let mut doubles = HashSet::new();
    let mut removed = HashSet::new();
    let mut second_last = n % 10;
    let mut last = (n / 10) % 10;
    if second_last == last {
        doubles.insert(last);
    }
    if last > second_last {
        return false;
    }
    let mut m = n / 10;
    while m > 0 {
        m /= 10;
        let k = m % 10;
        if k > last {
            return false;
        }
        if k == last {
            if !removed.contains(&k) {
                if last == second_last {
                    doubles.remove(&second_last);
                    removed.insert(second_last);
                } else {
                    doubles.insert(k);
                }
            }
        }
        second_last = last;
        last = k;
    }
    doubles.len() > 0
}

#[test]
fn example1() {
    assert!(check_p1(111111));
    assert!(!check_p1(223450));
    assert!(!check_p1(123789));
}

#[test]
fn example2() {
    assert!(check_p2(112233));
    assert!(!check_p2(123444));
    assert!(check_p2(111122));
}
