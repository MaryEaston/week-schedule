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
        let mut string = "".to_string();
        for i in 0..112 {
            let int = (self.check[i * 6 + 0] as i32) * 1
                + (self.check[i * 6 + 1] as i32) * 2
                + (self.check[i * 6 + 2] as i32) * 4
                + (self.check[i * 6 + 3] as i32) * 8
                + (self.check[i * 6 + 4] as i32) * 16
                + (self.check[i * 6 + 5] as i32) * 32;
            string.insert(i, _int_to_char(int));
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

fn _int_to_char(int: i32) -> char {
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

fn string_to_check(data: &str) -> [bool; 672] {
    let mut check: [bool; 672] = [false; 672];
    for (i, c) in data.chars().enumerate() {
        let (b5, b4, b3, b2, b1, b0) = _char_to_bits(c);
        check[i * 6 + 0] = b0;
        check[i * 6 + 1] = b1;
        check[i * 6 + 2] = b2;
        check[i * 6 + 3] = b3;
        check[i * 6 + 4] = b4;
        check[i * 6 + 5] = b5;
    }
    check
}

fn _char_to_bits(char: char) -> (bool, bool, bool, bool, bool, bool) {
    match char {
        'A' => (false, false, false, false, false, false),
        'B' => (false, false, false, false, false, true),
        'C' => (false, false, false, false, true, false),
        'D' => (false, false, false, false, true, true),
        'E' => (false, false, false, true, false, false),
        'F' => (false, false, false, true, false, true),
        'G' => (false, false, false, true, true, false),
        'H' => (false, false, false, true, true, true),
        'I' => (false, false, true, false, false, false),
        'J' => (false, false, true, false, false, true),
        'K' => (false, false, true, false, true, false),
        'L' => (false, false, true, false, true, true),
        'M' => (false, false, true, true, false, false),
        'N' => (false, false, true, true, false, true),
        'O' => (false, false, true, true, true, false),
        'P' => (false, false, true, true, true, true),
        'Q' => (false, true, false, false, false, false),
        'R' => (false, true, false, false, false, true),
        'S' => (false, true, false, false, true, false),
        'T' => (false, true, false, false, true, true),
        'U' => (false, true, false, true, false, false),
        'V' => (false, true, false, true, false, true),
        'W' => (false, true, false, true, true, false),
        'X' => (false, true, false, true, true, true),
        'Y' => (false, true, true, false, false, false),
        'Z' => (false, true, true, false, false, true),
        'a' => (false, true, true, false, true, false),
        'b' => (false, true, true, false, true, true),
        'c' => (false, true, true, true, false, false),
        'd' => (false, true, true, true, false, true),
        'e' => (false, true, true, true, true, false),
        'f' => (false, true, true, true, true, true),
        'g' => (true, false, false, false, false, false),
        'h' => (true, false, false, false, false, true),
        'i' => (true, false, false, false, true, false),
        'j' => (true, false, false, false, true, true),
        'k' => (true, false, false, true, false, false),
        'l' => (true, false, false, true, false, true),
        'm' => (true, false, false, true, true, false),
        'n' => (true, false, false, true, true, true),
        'o' => (true, false, true, false, false, false),
        'p' => (true, false, true, false, false, true),
        'q' => (true, false, true, false, true, false),
        'r' => (true, false, true, false, true, true),
        's' => (true, false, true, true, false, false),
        't' => (true, false, true, true, false, true),
        'u' => (true, false, true, true, true, false),
        'v' => (true, false, true, true, true, true),
        'w' => (true, true, false, false, false, false),
        'x' => (true, true, false, false, false, true),
        'y' => (true, true, false, false, true, false),
        'z' => (true, true, false, false, true, true),
        '0' => (true, true, false, true, false, false),
        '1' => (true, true, false, true, false, true),
        '2' => (true, true, false, true, true, false),
        '3' => (true, true, false, true, true, true),
        '4' => (true, true, true, false, false, false),
        '5' => (true, true, true, false, false, true),
        '6' => (true, true, true, false, true, false),
        '7' => (true, true, true, false, true, true),
        '8' => (true, true, true, true, false, false),
        '9' => (true, true, true, true, false, true),
        '-' => (true, true, true, true, true, false),
        '_' => (true, true, true, true, true, true),
        _ => (true, true, true, true, true, true),
    }
}
