use std::fs::{self, read};

use radolan::Radolan;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }

    let file =
        read("./dwd-dl/crates/radolan/tests/api/files/raa01-yw2017.002_10000-2301010525-dwd---bin")
            .expect("File not found");

    // create a new radolan file from byte array
    let radolan_file = Radolan::new(&file).unwrap();

    let row = 0;
    let column = 0;
    let point = radolan_file.get_point(row, column).unwrap();

    let value = point.default_f32(radolan_file.header().precision);
    let time = radolan_file.header().datetime;
    println!("time: {time}, row: {row}, column: {column}, value: {value}");
}
