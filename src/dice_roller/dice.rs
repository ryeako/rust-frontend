use yew::{Callback, function_component, html, Properties, classes};

#[derive(Properties, PartialEq)]
pub struct DiceProps {
    pub die_type: String,
    pub die_clicked: Callback<String>
}

#[function_component(Dice)]
pub fn dice(props: &DiceProps) -> Html {
    let die_type = props.die_type.clone();

    let button_style = &String::from("bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded");

    let onclick = { 
        let die_click = props.die_clicked.clone();    
        move |_| die_click.emit(die_type.clone())
    };

    html! {
        <button class={classes!(button_style)} {onclick}>
            {props.die_type.clone()}
        </button>
    }
}