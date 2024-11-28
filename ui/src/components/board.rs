use crate::types::enums::{Stone, BoardSize};
use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board_size: BoardSize,
    pub board: Vec<Vec<Stone>>,
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<(usize, usize)>,
}

fn get_w_style(row: usize, col: usize, board_size: usize) -> Vec<&'static str> {
    if col == board_size - 1 {
        vec![
            "w-1/2",
            "absolute",
            "top-1/2",
            "border-t",
            "border-black",
            "-z-10",
            "bg-[#352F44]"
        ]
    } else {
        vec![
            "w-full",
            "absolute",
            "top-1/2",
            "left-1/2",
            "border-t",
            "border-black",
            "-z-10",
            "bg-[#352F44]"
        ]
    }
}

fn get_h_style(row: usize, col: usize, board_size: usize) -> Vec<&'static str> {
    if row == board_size - 1 {
        vec![
            "h-1/2",
            "absolute",
            "left-1/2",
            "border-l",
            "border-black",
            "-z-10",
            "bg-[#352F44]"
        ]
    } else {
        vec![
            "h-full",
            "absolute",
            "top-1/2",
            "left-1/2",
            "border-l",
            "border-black",
            "-z-10",
            "bg-[#352F44]"
        ]
    }
}

fn get_board_style(size: BoardSize) -> &'static str {
    match size {
        BoardSize::Small => "grid-cols-9",
        BoardSize::Medium => "grid-cols-13",
        BoardSize::Large => "grid-cols-19",
    }
}

fn get_stone_style(stone: Stone) -> Vec<&'static str> {
    match stone {
        Stone::Black => vec![
            "bg-[#352F44]",
            "outline",
            "outline-1", 
            "outline-black",
            "flex-1",
            "box-border",
            "relative",
            "hover:outline",
            "outline-blue-500",
            "rounded-full"
        ],
        Stone::White => vec![
            "bg-[#FAF0E6]",
            "outline",
            "outline-1", 
            "outline-black",
            "flex-1",
            "box-border",
            "relative",
            "hover:outline",
            "outline-blue-500",
            "rounded-full"
        ],
        Stone::Empty => vec![
            "flex-1",
            "box-border",
            "relative",
            "hover:outline",
            "outline-blue-500",
            "rounded-full"
        ]
    }
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    html! {
        <div class={classes!("grid", get_board_style(props.board_size), "w-[580px]", "h-[580px]")}>
            {for (0..props.board.len()).flat_map(|row| {
                (0..props.board.len()).map(move |col| {
                    let stone = props.board[row][col].clone();
                    let on_click = props.on_click.clone();
                    let disabled = props.disabled.clone();
                    let onclick = Callback::from(move |_| {
                        if !disabled {
                            on_click.emit((row, col));
                        }
                    });
                    html_nested!  {
                        <div 
                            class={classes!(
                                get_stone_style(stone)
                            )}
                            onclick={onclick}
                        >
                            <div 
                                class={classes!(
                                    get_w_style(row, col, props.board.len())
                                )} 
                            />
                            <div 
                                class={classes!(
                                    get_h_style(row, col, props.board.len()),
                                )} 
                            />
                        </div>
                    }
                })
            })}
        </div>
    }
}