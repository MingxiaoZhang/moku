use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Stone {
    Empty,
    Black,
    White,
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub board: Vec<Vec<Stone>>,
    pub disabled: bool,
    #[prop_or_default]
    pub on_click: Callback<(usize, usize)>,
}

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    html! {
        <div class="board">
            {for (0..props.board.len()).map(|row| {
                html! {
                    <div class="board-row">
                        {for (0..props.board.len()).map(|col| {
                            let stone = props.board[row][col].clone();
                            let on_click = props.on_click.clone();
                            let disabled = props.disabled;
                            
                            let onclick = Callback::from(move |_| {
                                if !disabled {
                                    on_click.emit((row, col));
                                }
                            });
                            html! {
                                <button 
                                    class={classes!(
                                        "cell",
                                        match stone {
                                            Stone::Empty => "",
                                            Stone::Black => "black",
                                            Stone::White => "white",
                                        },
                                    )}
                                    onclick={onclick}
                                    disabled={disabled}
                                />
                            }
                        })}
                    </div>
                }
            })}
        </div>
    }
}