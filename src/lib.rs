// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

mod data;

use data::Data;

use gloo_console::log;
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, _: &mut impl Orders<Msg>) -> Model {
    match url.search().get("data") {
        Some(data) => Model {
            day: 0,
            check: Data::from(data.first().unwrap().to_string()),
        },
        None => Model {
            day: 0,
            check: Data::new(),
        },
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    day: i32,
    // check: [bool; 672],
    check: Data,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Check((bool, i32, i32, i32, i32)),
    ChangeDay(i32),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    // log!("{:?}", model.check);
    match msg {
        Msg::Check((value, day, hours, hour, minuit)) => {
            log!("msg: {:?}", hour * 4 + minuit);
            mod_check(model, value, day, hours, hour, minuit);
        }
        Msg::ChangeDay(day) => {
            model.day = day;
        }
    }
}

fn is_checked(model: &Model, day: i32, hours: i32, hour: i32, minuit: i32) -> bool {
    let mut is = true;
    if hour == -1 {
        for index in day * 96 + hours * 16..day * 96 + (hours + 1) * 16 {
            is &= model.check.is_checked(index as usize);
        }
    } else if minuit == -1 {
        for index in day * 96 + hour * 4..day * 96 + (hour + 1) * 4 {
            is &= model.check.is_checked(index as usize);
        }
    } else {
        let index = day * 96 + hour * 4 + minuit;
        is &= model.check.is_checked(index as usize);
    }
    is
}

fn is_day_checked(model: &Model, day: i32) -> bool {
    model.day == day
}

fn mod_check(model: &mut Model, value: bool, day: i32, hours: i32, hour: i32, minuit: i32) {
    if hour == -1 {
        for index in day * 96 + hours * 16..day * 96 + (hours + 1) * 16 {
            model.check.mod_check(index as usize, value);
        }
    } else if minuit == -1 {
        for index in day * 96 + hour * 4..day * 96 + (hour + 1) * 4 {
            model.check.mod_check(index as usize, value);
        }
    } else {
        let index = day * 96 + hour * 4 + minuit;
        model.check.mod_check(index as usize, value);
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
            ev(Ev::Click, move |_| Msg::Check((
                !is, day, hours, hour, minuit
            ))),
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
        tr!(minuit_td(model, day, section, section * 4 + 3, 3)),
    ]
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div!(
        section!(
            C!("pc"),
            attrs!(At::Id => "calender"),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "日")),
                trs(model, 0, 0),
                trs(model, 0, 1),
                trs(model, 0, 2),
                trs(model, 0, 3),
                trs(model, 0, 4),
                trs(model, 0, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "月")),
                trs(model, 1, 0),
                trs(model, 1, 1),
                trs(model, 1, 2),
                trs(model, 1, 3),
                trs(model, 1, 4),
                trs(model, 1, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "火")),
                trs(model, 2, 0),
                trs(model, 2, 1),
                trs(model, 2, 2),
                trs(model, 2, 3),
                trs(model, 2, 4),
                trs(model, 2, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "水")),
                trs(model, 3, 0),
                trs(model, 3, 1),
                trs(model, 3, 2),
                trs(model, 3, 3),
                trs(model, 3, 4),
                trs(model, 3, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "木")),
                trs(model, 4, 0),
                trs(model, 4, 1),
                trs(model, 4, 2),
                trs(model, 4, 3),
                trs(model, 4, 4),
                trs(model, 4, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "金")),
                trs(model, 5, 0),
                trs(model, 5, 1),
                trs(model, 5, 2),
                trs(model, 5, 3),
                trs(model, 5, 4),
                trs(model, 5, 5)
            ),
            table!(
                C!("calender"),
                thead!(th!(attrs!(At::ColSpan => "3"), "土")),
                trs(model, 6, 0),
                trs(model, 6, 1),
                trs(model, 6, 2),
                trs(model, 6, 3),
                trs(model, 6, 4),
                trs(model, 6, 5)
            )
        ),
        section!(
            C!("phone"),
            attrs!(At::Id => "calendar"),
            table!(
                C!("day"),
                thead!(
                    th!(
                        C![is_day_checked(model, 0).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(0)),
                            attrs!(At::Href => ""),
                            "日"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 1).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(1)),
                            attrs!(At::Href => ""),
                            "月"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 2).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(2)),
                            attrs!(At::Href => ""),
                            "火"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 3).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(3)),
                            attrs!(At::Href => ""),
                            "水"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 4).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(4)),
                            attrs!(At::Href => ""),
                            "木"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 5).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(5)),
                            attrs!(At::Href => ""),
                            "金"
                        )
                    ),
                    th!(
                        C![is_day_checked(model, 6).to_string()],
                        a!(
                            ev(Ev::Click, move |_| Msg::ChangeDay(6)),
                            attrs!(At::Href => ""),
                            "土"
                        )
                    )
                )
            ),
            table!(
                C!("calender"),
                // thead!(th!(attrs!(At::ColSpan => "3"), "日")),
                trs(model, model.day, 0),
                trs(model, model.day, 1),
                trs(model, model.day, 2),
                trs(model, model.day, 3),
                trs(model, model.day, 4),
                trs(model, model.day, 5)
            )
        ),
        p!(model.check.to_string())
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
