#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

mod nqueens;
mod singly_linked_1;
mod singly_linked_2;
mod doubly_linked_1;
mod traits;

use singly_linked_1::List;
use traits::List as _;

fn main() {
    let mut list: List<String> = List::new();
    for i in 0..=20 as usize {
        list.put_first((20 - i).to_string());
    }
    println!("{}", &list[0]);
    // let mut lll: singly_linked_1::List<i32> = List::new();
    // let x = &mut lll;
    // let y = lll.get(5);
    // let z = lll.get_mut(5);
    // nqueens::nqueens::<8>();

    // let mut list: List<i8> = List::new();
    // list.put_last(4)
    //     .put_last(5)
    //     .put_last(6)
    //     .put_last(7)
    //     .put_last(8);
    // let x = list.remove_last().unwrap();
    // println!("Removed: {removed:}\nList({len:}): {:?}", list, len = list.length(), removed = x);
    // let y = list.remove_nth(1).unwrap();
    // println!("Removed: {removed:}\nList({len:}): {:?}", list, len = list.length(), removed = y);
    // let z = list.remove_first_matching(|x| *x > 6).unwrap();
    // println!("Removed: {removed:}\nList({len:}): {:?}", list, len = list.length(), removed = z);
    // let w = list.replace_nth(1, 100).unwrap();
    // println!("Replaced: {replaced:}\nList({len:}): {:?}", list, len = list.length(), replaced = w);

    // let mut list = singly_linked_2::List::<i8>::new();
    // list.put_first(1).put_first(2).put_first(3);
    // println!("List({}): {:?}", list.length(), list);
    // let mut x = list.remove_first();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // x = list.remove_first();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // // x = list.remove_first();
    // // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // // x = list.remove_first();
    // // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // list.put_last(4).put_last(5).put_last(6);
    // println!("List({}): {:?}", list.length(), list);

    // x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let mut list = singly_linked_2::List::<i8>::new();
    // list.put_first(3).put_first(2).put_first(1).put_first(0);
    // let x = list.remove_last().unwrap();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let x = list.remove_nth(1).unwrap();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);

    // let x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let x = list.remove_last();
    // println!("Removed {removed:?} -- List({length:}): {list:?}", removed = x, length = list.length(), list = list);
    // let x = list.insert_at(4, 99);
    // println!(
    //     "Removed {removed:?} -- List({length:}): {list:?}",
    //     removed = x,
    //     length = list.length(),
    //     list = list
    // );
}
