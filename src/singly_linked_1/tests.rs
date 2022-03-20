#[cfg(test)]
use super::List;
use crate::traits::List as _;

#[test]
fn append_scale_test() {
    let mut list: List<u32> = List::new();
    for i in 0..=1_000_000 {
        list.put_first(i);
    }
    println!("List length: {}", list.length());
    drop(list);
}

#[test]
fn append_remove_test1() {
    let mut list: List<u16> = List::new();
    list.put_first(1).put_first(2).put_first(3);
    assert_eq!(list.remove_first().unwrap(), 3);
    assert_eq!(list.remove_first().unwrap(), 2);
    assert_eq!(list.remove_first().unwrap(), 1);
    assert!(list.remove_first().is_none());
}

#[test]
fn append_remove_test2() {
    let mut list: List<u16> = List::new();
    list.put_last(1).put_last(2).put_last(3);
    assert_eq!(list.remove_last().unwrap(), 3);
    assert_eq!(list.remove_last().unwrap(), 2);
    assert_eq!(list.remove_last().unwrap(), 1);
    assert!(list.remove_last().is_none());
}

#[test]
fn append_remove_test3() {
    let mut list: List<u16> = List::new();
    list.put_first(3)
        .put_last(4)
        .put_first(2)
        .put_last(5)
        .put_first(1);
    assert_eq!(list.remove_first().unwrap(), 1);
    assert_eq!(list.remove_last().unwrap(), 5);
    assert_eq!(list.remove_first().unwrap(), 2);
    assert_eq!(list.remove_last().unwrap(), 4);
    assert_eq!(list.remove_first().unwrap(), 3);
    assert!(list.remove_last().is_none());
    assert!(list.remove_first().is_none());
}

#[test]
fn insert_at_test() {
    let mut list: List<u16> = List::new();
    assert!(list.insert_at(5, 0).is_err());
    list.insert_at(0, 0);
    list.insert_at(1, 5);
    list.insert_at(1, 1);
    list.insert_at(2, 4);
    list.insert_at(2, 3);
    list.insert_at(2, 2);
    list.insert_at(6, 6);
    assert_eq!(list.length(), 7);
    assert_eq!(list.remove_first().unwrap(), 0);
    assert_eq!(list.remove_first().unwrap(), 1);
    assert_eq!(list.remove_first().unwrap(), 2);
    assert_eq!(list.remove_first().unwrap(), 3);
    assert_eq!(list.remove_first().unwrap(), 4);
    assert_eq!(list.remove_first().unwrap(), 5);
    assert_eq!(list.remove_first().unwrap(), 6);
    assert!(list.remove_first().is_none());
    assert_eq!(list.length(), 0);
}

#[test]
fn remove_first_matching_test() {
    let mut list: List<u16> = List::new();
    for i in 1..=8 {
        list.put_last(i);
    }
    assert_eq!(list.remove_first_matching(|x| x % 2 == 0).unwrap(), 2);
    assert_eq!(list.remove_first_matching(|x| x % 2 == 0).unwrap(), 4);
    assert_eq!(list.remove_first_matching(|x| x % 2 == 0).unwrap(), 6);
    assert_eq!(list.remove_first_matching(|x| x % 2 == 0).unwrap(), 8);
    assert!(list.remove_first_matching(|x| x % 2 == 0).is_none());
    assert_eq!(list.length(), 4);
}

#[test]
fn remove_nth_test() {
    let mut list: List<u16> = List::new();
    for i in 0..=8 {
        list.put_last(i);
    }
    assert_eq!(list.remove_nth(4).unwrap(), 4);
    assert_eq!(list.remove_nth(4).unwrap(), 5);
    assert_eq!(list.remove_nth(4).unwrap(), 6);
    assert_eq!(list.remove_nth(4).unwrap(), 7);
    assert_eq!(list.remove_nth(4).unwrap(), 8);
    assert!(list.remove_nth(4).is_none());
}

#[test]
fn replace_nth_test() {
    let mut list: List<u32> = List::new();
    for i in 0..=8 {
        list.put_last(i);
    }
    for i in 0..=8 {
        list.replace_nth(i, 10 * i as u32);
    }
    for i in 0..=8 {
        assert_eq!(list.remove_first().unwrap(), 10 * i);
    }
}

#[test]
fn length_test() {
    let mut list: List<String> = List::new();
    assert_eq!(list.length(), 0);
    for i in 1..=50 {
        list.put_first(i.to_string());
    }
    assert_eq!(list.length(), 50);
    for _ in 1..=50 {
        list.remove_last();
    }
    assert_eq!(list.length(), 0);
}
