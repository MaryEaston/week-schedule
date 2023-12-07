pub struct Data {
    check: [bool; 672],
}

impl From<String> for Data {
    fn from(item: String) -> Self {
        Data {
            check: string_to_check(&item),
        }
    }
}

impl From<&str> for Data {
    fn from(item: &str) -> Self {
        Data {
            check: string_to_check(item),
        }
    }
}

impl Data {
    pub fn new() -> Self {
        Data {
            check: [false; 672],
        }
    }

    // 自身とのANDをとる
    pub fn and(datas: Vec<Data>) -> Self {
        let mut check = [true; 672];
        for data in datas {
            for (i, bit) in data.check.iter().enumerate() {
                check[i] &= bit
            }
        }
        Data { check: check }
    }

    pub fn to_string(&self) -> String {
        let mut string: String = "".to_string();

        let mut mode: bool = false;
        let mut count: u16 = 0;
        for bit in self.check {
            if !(bit ^ mode) {
                count += 1;
            } else {
                mode = !mode;
                string = string + &int_to_string(count);
                count = 1;
            }
        }
        string
    }

    pub fn is_checked(&self, index: usize) -> bool {
        self.check[index]
    }

    pub fn mod_check(&mut self, index: usize, value: bool) {
        self.check[index] = value
    }
}

fn int_to_char(int: u16) -> char {
    match int {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => 'a',
        27 => 'b',
        28 => 'c',
        29 => 'd',
        30 => 'e',
        31 => 'f',
        32 => 'g',
        33 => 'h',
        34 => 'i',
        35 => 'j',
        36 => 'k',
        37 => 'l',
        38 => 'm',
        39 => 'n',
        40 => 'o',
        41 => 'p',
        42 => 'q',
        43 => 'r',
        44 => 's',
        45 => 't',
        46 => 'u',
        47 => 'v',
        48 => 'w',
        49 => 'x',
        50 => 'y',
        51 => 'z',
        52 => '0',
        53 => '1',
        54 => '2',
        55 => '3',
        56 => '4',
        57 => '5',
        58 => '6',
        59 => '7',
        60 => '8',
        61 => '9',
        62 => '-',
        63 => '_',
        _ => '*',
    }
}

fn int_to_string(int: u16) -> String {
    let mut string = String::new();
    let first = int_to_char(int % 64);
    let second = int_to_char((int / 64) % 64);
    string.push(second);
    string.push(first);
    string
}

fn char_to_int(char: char) -> u16 {
    match char {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        'a' => 26,
        'b' => 27,
        'c' => 28,
        'd' => 29,
        'e' => 30,
        'f' => 31,
        'g' => 32,
        'h' => 33,
        'i' => 34,
        'j' => 35,
        'k' => 36,
        'l' => 37,
        'm' => 38,
        'n' => 39,
        'o' => 40,
        'p' => 41,
        'q' => 42,
        'r' => 43,
        's' => 44,
        't' => 45,
        'u' => 46,
        'v' => 47,
        'w' => 48,
        'x' => 49,
        'y' => 50,
        'z' => 51,
        '0' => 52,
        '1' => 53,
        '2' => 54,
        '3' => 55,
        '4' => 56,
        '5' => 57,
        '6' => 58,
        '7' => 59,
        '8' => 60,
        '9' => 61,
        '-' => 62,
        '_' => 63,
        _ => 0,
    }
}

fn string_to_check(string: &str) -> [bool; 672] {
    let mut ints: Vec<u16> = vec![];
    let mut second: Option<u16> = Option::None;
    for char in string.chars() {
        match second {
            Some(second_in) => {
                ints.push(second_in * 64 + char_to_int(char));
                second = Option::None;
            }
            None => second = Option::Some(char_to_int(char)),
        }
    }

    let mut check = [false; 672];
    let mut cursor: u16 = 0;
    let mut mode: bool = false;
    for int in ints.iter() {
        for i in 0..*int {
            check[(cursor + i) as usize] = mode;
        }
        cursor += *int;
        mode = !mode;
    }
    check
}
