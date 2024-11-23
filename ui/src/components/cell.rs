use yew::prelude::*;
use crate::components::board::Stone;

#[derive(Properties, PartialEq)]
pub struct CellProps {
    pub row: usize,
    pub col: usize,
    pub stone: Stone,
    pub on_click: Callback<(usize, usize)>,
}

#[function_component(Cell)]
pub fn cell(props: &CellProps) -> Html {
    let on_click = {
        let row = props.row;
        let col = props.col;
        let callback = props.on_click.clone();
        Callback::from(move |_| callback.emit((row, col)))
    };

    let style = match props.stone {
        Stone::Empty => "background: white;",
        Stone::Black => "background: black;",
        Stone::White => "background: white; border: 1px solid black;",
    };

    html! {
        <div
            onclick={on_click}
            style={format!("width: 30px; height: 30px; {}", style)}
        ></div>
    }
}
