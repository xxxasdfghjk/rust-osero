use std::{
    fs,
    path::{Path, PathBuf},
};

use rusqlite::{params, Connection, Error, Result};

use crate::{book::Book, computer::get_next_hand_from_book};

fn get_relative_path(relative_path: &str) -> PathBuf {
    let current_file_path = Path::new(file!());
    let current_dir = current_file_path.parent().unwrap();
    current_dir.join(relative_path)
}

#[test]
pub fn open_my_db_test() {
    let path = "./../../assets/book";
    let db_path = get_relative_path(path);
    if !fs::metadata(&db_path).is_ok() {
        panic!(
            "Database file does not exist at path: {}",
            db_path.to_str().unwrap()
        );
    }
    let history = "F5D6";
    let con = Connection::open(&db_path).unwrap();
    let mut statement = con
        .prepare("select id,name,moves from book where moves like ?1")
        .unwrap();
    let res = statement
        .query_map(params![format!("{history}%")], |row| {
            Ok(Book {
                name: row.get(1).unwrap(),
                id: row.get(0).unwrap(),
                moves: row.get(2).unwrap(),
            })
        })
        .unwrap()
        .filter_map(|x| x.ok())
        .filter(|x| (x.moves.len() > history.len()));
    for i in res {
        let item = i;
        println!("{}", &item.moves[history.len()..]);
    }
}

#[test]
pub fn get_book_test() {
    let res = get_next_hand_from_book("F5");
    println!("{}", res.unwrap());
    assert!(res.unwrap() > 0)
}
