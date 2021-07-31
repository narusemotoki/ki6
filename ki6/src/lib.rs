#![allow(clippy::non_ascii_literal)]

use chrono::prelude::*;
use seed::prelude::*;
mod libs;

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    let date = chrono::Local::today().naive_local();
    Model {
        date,
        columns: build_columns(date),
    }
}

fn build_columns(date: NaiveDate) -> Vec<Column> {
    let monday = libs::date::compute_last_monday(date);
    (0..7)
        .into_iter()
        .map(|i| Column {
            date: monday + chrono::Duration::days(i),
            items: vec!["".to_string()],
        })
        .collect()
}

struct Column {
    date: chrono::NaiveDate,
    items: Vec<String>,
}

struct Model {
    date: chrono::NaiveDate,
    columns: Vec<Column>,
}

#[derive(Clone)]
enum Msg {
    MoveToPreviousWeek,
    MoveToNextWeek,
    TextChanged(chrono::NaiveDate, usize, String),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    let one_week = chrono::Duration::days(7);
    match msg {
        Msg::MoveToPreviousWeek => move_date(model, model.date - one_week),
        Msg::MoveToNextWeek => move_date(model, model.date + one_week),
        Msg::TextChanged(date, index, new_text) => {
            if let Some(column) = model.columns.iter_mut().find(|column| column.date == date) {
                column.items[index] = new_text;
            }
        }
    }
}

fn move_date(model: &mut Model, new_date: chrono::NaiveDate) {
    model.date = new_date;
    model.columns = build_columns(new_date);
}

static CELL_STYLE: once_cell::sync::Lazy<seed::virtual_dom::Style> =
    once_cell::sync::Lazy::new(|| {
        seed::style! {
            St::Padding => "0 4px";
            St::BackgroundColor => "white";
            St::Height => "100%";
            St::Display => "flex";
            St::FlexDirection => "column";
        }
    });

fn view_week_column(date: chrono::NaiveDate) -> Node<Msg> {
    let monday = libs::date::compute_last_monday(date);

    let iso_week = monday.iso_week();
    let year_and_week = format!("{}年第{}週", iso_week.year(), iso_week.week());

    seed::div![
        CELL_STYLE.clone(),
        seed::div![
            seed::style! {
                St::Display => "flex";
            },
            seed::button![ev(Ev::Click, |_| Msg::MoveToPreviousWeek), "◀"],
            seed::p![
                seed::style! {
                    St::Margin => "0";
                    St::FlexGrow => "1";
                    St::TextAlign => "center";
                },
                year_and_week
            ],
            seed::button![ev(Ev::Click, |_| Msg::MoveToNextWeek), "▶"],
        ],
    ]
}

fn view(model: &Model) -> Node<Msg> {
    let mut cells: Vec<Node<Msg>> = vec![view_week_column(model.date)];
    for column in &model.columns {
        let items: Vec<Node<Msg>> = column
            .items
            .iter()
            .enumerate()
            .map(|(index, text)| {
                let date = column.date;
                seed::li![
                    seed::style! {
                        St::Position => "relative";
                    },
                    seed::div![
                        seed::style! {
                            // borderなど、textareaが標準で持っている要素に合わせています。
                            St::Overflow => "hidden";
                            St::Visibility => "hidden";
                            St::BoxSizing => "border-box";
                            St::Padding => "4px 8px";
                            St::WhiteSpace => "pre-wrap";
                            St::OverflowWrap => "break-word";
                            St::Border => "1px solid";
                        },
                        // 行に何か文字がないと高さが計算されないため、0幅スペースを入れています。
                        format!("{}{}", text, "\u{200b}"),
                    ],
                    seed::textarea![
                        seed::style! {
                            St::Position => "absolute";
                            St::Top => "0";
                            St::Left => "0";
                            St::Display => "block";
                            St::Overflow => "hidden";
                            St::BoxSizing => "border-box";
                            St::Padding => "4px 8px";
                            St::Width => "100%";
                            St::Height => "100%";
                            St::LetterSpacing => "inherit";
                            St::Font => "inherit";
                            St::Resize => "none";
                            St::WhiteSpace => "pre-wrap";
                            St::OverflowWrap => "break-word";
                        },
                        seed::attrs! {
                            At::Value => text;
                        },
                        input_ev(Ev::Input, move |new_text| {
                            Msg::TextChanged(date, index, new_text)
                        }),
                    ]
                ]
            })
            .collect();

        cells.push(seed::div![
            CELL_STYLE.clone(),
            seed::p![
                seed::style! {
                    St::Margin => "0";
                    St::TextAlign => "center";
                    St::Width => "100%"
                },
                format!(
                    "{}日 ({})",
                    column.date.day(),
                    libs::date::weekday_to_japanese(column.date.weekday())
                )
            ],
            seed::ul![
                seed::style! {
                    St::ListStyle => "none";
                    St::Padding => "0";
                    St::Margin => "0";
                    St::OverflowY => "auto";
                    St::FlexGrow => "1";
                },
                items
            ]
        ]);
    }

    seed::div![
        seed::style! {
            St::Display => "grid";
            St::GridTemplateRows => "repeat(2, minmax(0, 1fr))";
            St::GridTemplateColumns => "repeat(4, minmax(0, 1fr))";
            St::Height => "100vh";
            St::ColumnGap => "4px";
            St::RowGap => "4px";
            St::BackgroundColor => "grey";
        },
        cells
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
