// use chrono::Utc;
use web_sys::HtmlInputElement;
use yew::events::KeyboardEvent;
use yew::{function_component, html, Callback, Properties, TargetCast};

use crate::state::Update;

#[derive(PartialEq, Properties, Clone)]
pub struct HeaderProps {
    pub user_id: usize,
    pub onadd: Callback<Update>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let onkeypress = {
        let onadd = props.onadd.clone();
        let user_id = props.user_id;

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                // let date = Utc::now().to_rfc3339();
                let input: HtmlInputElement = e.target_unchecked_into();
                let update = Update {
                    id: 0,
                    user_id,
                    score: input.value(),
                    // date,
                };

                input.set_value("");
                onadd.emit(update);
            }
        }
    };

    html! {
        <input
            class="new-update"
            placeholder="What is your new score?"
            {onkeypress}
        />
    }
}
