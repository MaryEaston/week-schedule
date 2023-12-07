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
    let view = match url.search().get("view") {
        Some(view) => match &*view.first().unwrap().to_string() {
            "all" => View::All,
            "morning" => View::Morning,
            "night" => View::Night,
            "normal" => View::Normal,
            _ => View::Normal,
        },
        None => View::Normal,
    };
    let check: Data = match url.search().get("data") {
        Some(data) => Data::from(data.first().unwrap().to_string()),
        None => Data::new(),
    };
    Model {
        day: 0,
        view: view,
        check: check,
    }
}

// ------ ------
//     Model
// ------ ------
#[derive(PartialEq)]
enum View {
    All,
    Morning,
    Night,
    Normal,
}
// `Model` describes our app state.
struct Model {
    day: i32,
    view: View,
    check: Data,
}

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {
    Check((bool, i32, i32, i32, i32)),
    ChangeDay(i32),
    Input(String),
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
        Msg::Input(text) => {
            let mut datas: Vec<Data> = vec![];
            for line in text.lines() {
                datas.push(Data::from(line));
            }
            model.check = Data::and(datas);
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

fn hour_td(model: &Model, day: i32, hours: i32, hour: i32, minuit: i32) -> Node<Msg> {
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

fn hours_td(model: &Model, day: i32, section: i32) -> Vec<Node<Msg>> {
    vec![
        tr!(
            hour_td(model, day, section, -1, -1),
            hour_td(model, day, section, section * 4 + 0, -1),
            hour_td(model, day, section, section * 4 + 0, 0)
        ),
        tr!(hour_td(model, day, section, section * 4 + 0, 1)),
        tr!(hour_td(model, day, section, section * 4 + 0, 2)),
        tr!(hour_td(model, day, section, section * 4 + 0, 3)),
        tr!(
            hour_td(model, day, section, section * 4 + 1, -1),
            hour_td(model, day, section, section * 4 + 1, 0)
        ),
        tr!(hour_td(model, day, section, section * 4 + 1, 1)),
        tr!(hour_td(model, day, section, section * 4 + 1, 2)),
        tr!(hour_td(model, day, section, section * 4 + 1, 3)),
        tr!(
            hour_td(model, day, section, section * 4 + 2, -1),
            hour_td(model, day, section, section * 4 + 2, 0)
        ),
        tr!(hour_td(model, day, section, section * 4 + 2, 1)),
        tr!(hour_td(model, day, section, section * 4 + 2, 2)),
        tr!(hour_td(model, day, section, section * 4 + 2, 3)),
        tr!(
            hour_td(model, day, section, section * 4 + 3, -1),
            hour_td(model, day, section, section * 4 + 3, 0)
        ),
        tr!(hour_td(model, day, section, section * 4 + 3, 1)),
        tr!(hour_td(model, day, section, section * 4 + 3, 2)),
        tr!(hour_td(model, day, section, section * 4 + 3, 3)),
    ]
}

fn day_td_for_pc(model: &Model, day: i32) -> Node<Msg> {
    let morning = if model.view == View::Morning || model.view == View::All {
        vec![hours_td(model, day, 0), hours_td(model, day, 1)]
    } else {
        vec![]
    };
    let night = if model.view == View::Night || model.view == View::All {
        vec![hours_td(model, day, 5)]
    } else {
        vec![]
    };
    table!(
        C!("calender"),
        thead!(th!(attrs!(At::ColSpan => "3"), "　")),
        morning,
        hours_td(model, day, 2),
        hours_td(model, day, 3),
        hours_td(model, day, 4),
        night
    )
}

fn day_td_for_phone(model: &Model, day: i32) -> Node<Msg> {
    let morning = if model.view == View::Morning || model.view == View::All {
        vec![hours_td(model, day, 0), hours_td(model, day, 1)]
    } else {
        vec![]
    };
    let night = if model.view == View::Night || model.view == View::All {
        vec![hours_td(model, day, 5)]
    } else {
        vec![]
    };
    table!(
        C!("calender"),
        morning,
        hours_td(model, day, 2),
        hours_td(model, day, 3),
        hours_td(model, day, 4),
        night
    )
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
                C!("day"),
                thead!(
                    th!(C!["false"], "日"),
                    th!(C!["false"], "月"),
                    th!(C!["false"], "火"),
                    th!(C!["false"], "水"),
                    th!(C!["false"], "木"),
                    th!(C!["false"], "金"),
                    th!(C!["false"], "土")
                )
            ),
            day_td_for_pc(model, 0),
            day_td_for_pc(model, 1),
            day_td_for_pc(model, 2),
            day_td_for_pc(model, 3),
            day_td_for_pc(model, 4),
            day_td_for_pc(model, 5),
            day_td_for_pc(model, 6),
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
            table!(thead!(th!(a!("　")))),
            day_td_for_phone(model, model.day),
        ),
        p!(format!("予定コード : {:?}", model.check.to_string())),
        p!("予定コード入力欄"),
        textarea!(input_ev(Ev::Input, move |text| Msg::Input(text)))
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
