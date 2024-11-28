use yew::prelude::*;
use crate::components::board::Board;
use crate::types::enums::{Stone, BoardSize};
use gloo_net::http::Request;
use serde::{Serialize, Deserialize};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub game_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameMove {
    row: usize,
    col: usize,
    player: Stone,
}

pub struct OnlineGame {
    board: Vec<Vec<Stone>>,
    game_id: String,
    current_player: Stone,
    my_stone: Stone,
    winner: Option<Stone>,
}

impl Component for OnlineGame {
    type Message = GameMove;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            board: vec![vec![Stone::Empty; 15]; 15],
            current_player: Stone::Black,
            game_id: _ctx.props().game_id.clone(),
            my_stone: Stone::Black,
            winner: Some(Stone::Black)
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let status = if let Some(winner) = &self.winner {
            match winner {
                Stone::Black => "Black wins!",
                Stone::White => "White wins!",
                Stone::Empty => unreachable!(),
            }
        } else {
            match self.current_player {
                Stone::Black => "Black's turn",
                Stone::White => "White's turn",
                Stone::Empty => unreachable!(),
            }
        };

        html! {
            <div class="gomoku-game">
                <div class="status">{status}</div>
                <Board 
                    board_size={BoardSize::Small}
                    board={self.board.clone()}
                    disabled={self.winner.is_some()}
                />
                <button 
                    class="reset-button"
                >
                    {"Reset Game"}
                </button>
            </div>
        }
    }
}