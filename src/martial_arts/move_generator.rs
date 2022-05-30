use yew::{ Component, Context, html, Html, Properties };

use crate::services::roller::{roller};

pub enum Msg {
    RollDice
}

#[derive(PartialEq, Properties)]
pub struct Props;

#[derive(Clone, PartialEq, Debug)]
pub struct MoveGenerator {
    roll: i32,
    dice: String,
    rolls: Vec<i32>
}

impl Component for MoveGenerator {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        MoveGenerator {
            roll: 0,
            dice: String::from("2d8"),
            rolls: Vec::new()
        }
    }
    
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::RollDice => {
                let new_roll = roller(self.dice.clone()).unwrap();
                self.roll = new_roll.total();
                self.rolls = new_roll.rolled_dice();
                true
            }
        }   
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let onclick = _ctx.link().callback(|_| Msg::RollDice);
        html! {
            <section>
                <h1>{"Sick Moves bra"}</h1>
                <button {onclick}>{"D8 please"}</button>
                <p>
                    {self.dice.clone()}
                </p>
                <p>
                    {self.roll.to_string()}
                </p>
                <p>
                    {
                        self.rolls.iter().map(|d| {
                            html!{<span>{d}</span>}
                        }).collect::<Html>()
                    }
                </p>
            </section>
        }
    }
}