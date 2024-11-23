use yew::prelude::*;
use crate::components::board::{Board, Stone};
use crate::game::GameState;

pub enum Msg {
    Place(usize, usize),
    Reset,
}

pub struct LocalGame {
    game_state: GameState,
}

impl Component for LocalGame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game_state: GameState::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Place(row, col) => self.game_state.make_move(row, col),
            Msg::Reset => {
                self.game_state.reset();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let status = if let Some(winner) = &self.game_state.winner {
            match winner {
                Stone::Black => "Black wins!",
                Stone::White => "White wins!",
                Stone::Empty => unreachable!(),
            }
        } else {
            match self.game_state.current_player {
                Stone::Black => "Black's turn",
                Stone::White => "White's turn",
                Stone::Empty => unreachable!(),
            }
        };

        html! {
            <div class="gomoku-game">
                <div class="status">{status}</div>
                <Board 
                    board={self.game_state.board.clone()}
                    disabled={self.game_state.winner.is_some()}
                    on_click={ctx.link().callback(|(row, col)| Msg::Place(row, col))}
                />
                <button 
                    class="reset-button"
                    onclick={ctx.link().callback(|_| Msg::Reset)}
                >
                    {"Reset Game"}
                </button>
            </div>
        }
    }
}