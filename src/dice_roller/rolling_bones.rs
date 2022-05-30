use wasm_bindgen::JsCast;
use yew::{Component, Context, html, Html, Properties, Event, classes};
use web_sys::HtmlInputElement;

use super::dice::Dice;
use crate::services::roller::roller;

#[derive(PartialEq, Properties)]
pub struct Props;

pub enum Msg {
    RollDice(String),
    DieTotal(Event)
}

pub struct RollingBones {
    roll: i32,
    rolls: Vec<i32>,
    die_number: i32
}

impl Component for RollingBones {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        RollingBones {
            roll: 0,
            rolls: Vec::new(),
            die_number: 1
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        match _msg {
            Msg::RollDice(event) => {
                let dice_to_roll = format!("{}{}", &self.die_number, event);
                let new_roll = roller(dice_to_roll).unwrap();
                self.roll = new_roll.total();
                self.rolls = new_roll.rolled_dice();
                true
            },
            Msg::DieTotal(event) => {
                let target = event.target();
                let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
                let value = input.map(|input| input.value()).unwrap_or(String::from("1"));

                let mut die_number:i32 = value.parse::<i32>().unwrap_or(1);
                if die_number < 1 {
                    die_number = 1;
                }
                self.die_number = die_number;
                true
            }
        }   
    }
    
    fn view(&self, _ctx: &Context<Self>) -> Html {
        let die_clicked = &_ctx.link().callback(|event: String| Msg::RollDice(event));
        let die_number = _ctx.link().callback(|event: Event| Msg::DieTotal(event));

        let die_rolls_style = &String::from("bg-green-500 font-bold py-1 px-2 rounded m-1");

        let roll_total = move || -> Html {
            if self.roll > 0 {
                html! {
                    <p class={classes!(die_rolls_style)}>
                        {self.roll.to_string()}
                    </p>
                }
            } else {
                html! {}
            }
        };

        html! {
            <section>
                <h1>{"Roll dem bones"}</h1>
                <div>
                    <p>{"How many?"}</p>
                    <input class={"text-black"} type="number" onchange={die_number} />
                </div>

                <div class={"py-3"}><Dice die_type="d2" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d3" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d4" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d6" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d8" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d10" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d12" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d20" {die_clicked} /></div>
                <div class={"py-3"}><Dice die_type="d100" {die_clicked} /></div>

                {roll_total()}

                <p class={"flex flex-wrap"}>
                    {
                        self.rolls.iter().map(|d| {
                            html!{<span class={classes!(die_rolls_style)} >{d}</span>}
                        }).collect::<Html>()
                    }
                </p>
            </section>
        }
    }
}