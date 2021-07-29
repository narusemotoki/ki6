#![allow(clippy::non_ascii_literal)]

use chrono::prelude::*;
use seed::prelude::*;
mod libs;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        date: chrono::Local::today().naive_local(),
    }
}

struct Model {
    date: chrono::NaiveDate,
}

#[derive(Copy, Clone)]
enum Msg {
    MoveToPreviousWeek,
    MoveToNextWeek,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    let one_week = chrono::Duration::days(7);
    match msg {
        Msg::MoveToPreviousWeek => model.date -= one_week,
        Msg::MoveToNextWeek => model.date += one_week,
    }
}

static CELL_STYLE: once_cell::sync::Lazy<seed::virtual_dom::Style> =
    once_cell::sync::Lazy::new(|| {
        seed::style! {
            "display" => "flex";
            "align-items" => "flex-start";
            "padding" => "0 4px";
            "background-color" => "white"
        }
    });

fn week_column_view(model: &Model) -> Node<Msg> {
    let monday = libs::date::compute_last_monday(model.date);

    let iso_week = monday.iso_week();
    let year_and_week = format!("{}年第{}週", iso_week.year(), iso_week.week());

    seed::div![
        CELL_STYLE.clone(),
        seed::button![ev(Ev::Click, |_| Msg::MoveToPreviousWeek), "◀"],
        seed::p![
            seed::style! {
                "margin" => "0";
                "flex-grow" => "1";
                "text-align" => "center";
            },
            year_and_week
        ],
        seed::button![ev(Ev::Click, |_| Msg::MoveToNextWeek), "▶"],
    ]
}

fn view(model: &Model) -> Node<Msg> {
    let mut cells: Vec<Node<Msg>> = vec![week_column_view(model)];
    let monday = libs::date::compute_last_monday(model.date);
    for i in 0..7 {
        let date = monday + chrono::Duration::days(i);
        cells.push(seed::div![
            CELL_STYLE.clone(),
            seed::p![
                seed::style! {
                    "margin" => "0";
                    "text-align" => "center";
                    "width" => "100%"
                },
                format!(
                    "{}日 ({})",
                    date.day(),
                    libs::date::weekday_to_japanese(date.weekday())
                )
            ]
        ]);
    }

    seed::div![
        seed::style! {
            "display" => "grid";
            "grid-template-columns" => "repeat(4, 1fr)";
            "height" => "100vh";
            "column-gap" => "4px";
            "row-gap" => "4px";
            "background-color" => "grey";
        },
        "",
        cells
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
