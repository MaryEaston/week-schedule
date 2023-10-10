// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{ prelude::*, * };
use gloo_console::log;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    match url.search().get("data") {
        Some(data) =>
            Model { counter: 0, check: Model::string_to_check(data.first().unwrap().to_string()) },
        None => Model { counter: 0, check: [false; 672] },
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    counter: i32,
    check: [bool; 672],
    // check: Bytes<bool>,
}

impl Model {
    pub fn check_to_string(&self) -> String {
        let mut string = "".to_string();
        for i in 0..112 {
            let int =
                (self.check[i * 6 + 0] as i32) * 1 +
                (self.check[i * 6 + 1] as i32) * 2 +
                (self.check[i * 6 + 2] as i32) * 4 +
                (self.check[i * 6 + 3] as i32) * 8 +
                (self.check[i * 6 + 4] as i32) * 16 +
                (self.check[i * 6 + 5] as i32) * 32;
            string.insert(i, Model::_int_to_char(int));
        }
        string
    }

    pub fn string_to_check(data: String) -> [bool; 672] {
        let mut check: [bool; 672] = [false; 672];
        for (i, c) in data.chars().enumerate() {
            let (b5, b4, b3, b2, b1, b0) = Model::_char_to_int(c);
            check[i * 6 + 0] = b0;
            check[i * 6 + 1] = b1;
            check[i * 6 + 2] = b2;
            check[i * 6 + 3] = b3;
            check[i * 6 + 4] = b4;
            check[i * 6 + 5] = b5;
        }
        check
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

    fn _char_to_int(char: char) -> (bool, bool, bool, bool, bool, bool) {
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
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Check((bool, i32, i32, i32, i32)),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    // log!("{:?}", model.check);
    match msg {
        Msg::Check((value, day, hours, hour, minuit)) => {
            log!("msg: {:?}", hour * 4 + minuit);
            check(model, value, day, hours, hour, minuit);
            model.counter += 1;
        }
    }
}

fn is_checked(model: &Model, day: i32, hours: i32, hour: i32, minuit: i32) -> bool {
    let mut is = true;
    if hour == -1 {
        for index in day * 96 + hours * 16..day * 96 + (hours + 1) * 16 {
            is &= model.check[index as usize];
        }
    } else if minuit == -1 {
        for index in day * 96 + hour * 4..day * 96 + (hour + 1) * 4 {
            is &= model.check[index as usize];
        }
    } else {
        let index = day * 96 + hour * 4 + minuit;
        is &= model.check[index as usize];
    }
    is
}

fn check(model: &mut Model, value: bool, day: i32, hours: i32, hour: i32, minuit: i32) {
    if hour == -1 {
        for index in day * 96 + hours * 16..day * 96 + (hours + 1) * 16 {
            model.check[index as usize] = value;
        }
    } else if minuit == -1 {
        for index in day * 96 + hour * 4..day * 96 + (hour + 1) * 4 {
            model.check[index as usize] = value;
        }
    } else {
        let index = day * 96 + hour * 4 + minuit;
        model.check[index as usize] = value;
    }
}

fn minuit_td(model: &Model, day: i32, hours: i32, hour: i32, minuit: i32) -> Node<Msg> {
    let is = is_checked(model, day, hours, hour, minuit);
    let text = if hour == -1 {
        "".to_string()
    } else {
        match minuit {
            0 => "0~15".to_string(),
            1 => "15~30".to_string(),
            2 => "30~45".to_string(),
            3 => "45~00".to_string(),
            -1 => hour.to_string(),
            _ => "error".to_string(),
        }
    };
    let size = if hour == -1 {
        attrs!(At::RowSpan => "16")
    } else if minuit == -1 {
        attrs!(At::RowSpan => "4")
    } else {
        attrs!(At::RowSpan => "1")
    };
    td!(
        C!(is.to_string()),
        size,
        a!(
            ev(Ev::Click, move |_| Msg::Check((!is, day, hours, hour, minuit))),
            attrs!(At::Href => ""),
            text
        ),
        input!(attrs!(At::Type => "Checkbox",At::Value => hours * 16 + hour * 4 + minuit))
    )
}

fn trs(model: &Model, day: i32, section: i32) -> Vec<Node<Msg>> {
    vec![
        tr!(
            minuit_td(model, day, section, -1, -1),
            minuit_td(model, day, section, section * 4 + 0, -1),

            minuit_td(model, day, section, section * 4 + 0, 0)
        ),
        tr!(minuit_td(model, day, section, section * 4 + 0, 1)),
        tr!(minuit_td(model, day, section, section * 4 + 0, 2)),
        tr!(minuit_td(model, day, section, section * 4 + 0, 3)),
        tr!(
            minuit_td(model, day, section, section * 4 + 1, -1),

            minuit_td(model, day, section, section * 4 + 1, 0)
        ),
        tr!(minuit_td(model, day, section, section * 4 + 1, 1)),
        tr!(minuit_td(model, day, section, section * 4 + 1, 2)),
        tr!(minuit_td(model, day, section, section * 4 + 1, 3)),
        tr!(
            minuit_td(model, day, section, section * 4 + 2, -1),

            minuit_td(model, day, section, section * 4 + 2, 0)
        ),
        tr!(minuit_td(model, day, section, section * 4 + 2, 1)),
        tr!(minuit_td(model, day, section, section * 4 + 2, 2)),
        tr!(minuit_td(model, day, section, section * 4 + 2, 3)),
        tr!(
            minuit_td(model, day, section, section * 4 + 3, -1),

            minuit_td(model, day, section, section * 4 + 3, 0)
        ),
        tr!(minuit_td(model, day, section, section * 4 + 3, 1)),
        tr!(minuit_td(model, day, section, section * 4 + 3, 2)),
        tr!(minuit_td(model, day, section, section * 4 + 3, 3))
    ]
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div!(
        section!(
            attrs!(At::Id => "calendar"),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "日")),
                trs(model, 0, 0),
                trs(model, 0, 1),
                trs(model, 0, 2),
                trs(model, 0, 3),
                trs(model, 0, 4),
                trs(model, 0, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "月")),
                trs(model, 1, 0),
                trs(model, 1, 1),
                trs(model, 1, 2),
                trs(model, 1, 3),
                trs(model, 1, 4),
                trs(model, 1, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "火")),
                trs(model, 2, 0),
                trs(model, 2, 1),
                trs(model, 2, 2),
                trs(model, 2, 3),
                trs(model, 2, 4),
                trs(model, 2, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "水")),
                trs(model, 3, 0),
                trs(model, 3, 1),
                trs(model, 3, 2),
                trs(model, 3, 3),
                trs(model, 3, 4),
                trs(model, 3, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "木")),
                trs(model, 4, 0),
                trs(model, 4, 1),
                trs(model, 4, 2),
                trs(model, 4, 3),
                trs(model, 4, 4),
                trs(model, 4, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "金")),
                trs(model, 5, 0),
                trs(model, 5, 1),
                trs(model, 5, 2),
                trs(model, 5, 3),
                trs(model, 5, 4),
                trs(model, 5, 5)
            ),
            table!(
                C!("gridTable", "gap"),
                thead!(th!(attrs!(At::ColSpan => "3"), "土")),
                trs(model, 6, 0),
                trs(model, 6, 1),
                trs(model, 6, 2),
                trs(model, 6, 3),
                trs(model, 6, 4),
                trs(model, 6, 5)
            )
        ),
        p!(model.check_to_string())
    )
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
