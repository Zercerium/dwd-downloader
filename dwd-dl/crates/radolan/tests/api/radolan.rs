use radolan::Radolan;

const RADOLAN_FILE_BYTES: &[u8] =
    include_bytes!("files/raa01-yw2017.002_10000-2301010525-dwd---bin");
const RADOLAN_FILE_ASCII_STR: &str = include_str!("files/YW_2017.002_20230101_0525.asc");

/// Test if the radolan binary format is parsed and returns the same values as the ascii equivalent
#[test]
fn cmp_radolan_file() {
    let ascii_vec = parse_ascii_str_to_vec(RADOLAN_FILE_ASCII_STR);
    let radolan_file = Radolan::new(RADOLAN_FILE_BYTES).unwrap();

    for column in 0..900 {
        for row in 0..1100 {
            let point = radolan_file
                .get_point(row, column)
                .expect(&format!("{},{}", row, column));
            let (row, column) = (row as usize, column as usize);

            let value_ascii = ascii_vec[row][column];
            let value_bin = point.default_f32(100);

            // sometimes the DWD rounds up, sometimes down, so we have to check booth
            // also there is only one decimal place (like 0.1)
            let value_bin_ceil = (value_bin * 10.).ceil() as f32 / 10.;
            let value_bin_floor = (value_bin * 10.).floor() as f32 / 10.;
            assert!(
                (value_ascii == value_bin_ceil || value_ascii == value_bin_floor),
                "row: {}, column: {}, ascii: {}, bin: {}, record: {:#?}",
                row,
                column,
                value_ascii,
                value_bin,
                point,
            );
        }
    }

    dbg!(radolan_file.get_point(0, 0).unwrap());
}

fn parse_ascii_str_to_vec(str: &str) -> Vec<Vec<f32>> {
    let parse_single_line = |line: &str| {
        line.split_ascii_whitespace()
            .map(|v| v.trim().parse::<f32>().unwrap())
            .collect::<Vec<_>>()
    };
    str.lines().skip(6).map(parse_single_line).collect()
}
