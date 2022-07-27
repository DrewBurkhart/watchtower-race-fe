// use crate::hooks::use_bool_toggle::use_bool_toggle;
use crate::state::Update as Item;
use yew::{function_component, html, Callback, Classes, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct UpdateProps {
    pub update: Item,
    pub onremove: Callback<usize>,
}

#[function_component(Update)]
pub fn update(props: &UpdateProps) -> Html {
    let id = props.update.id;
    let class = Classes::from("update");

    html! {
        <li {class}>
            <div class="view">
                <label>
                    { format!("User: {} - Score: {}", &props.update.user_id, &props.update.score) }
                </label>
                <button class="destroy" onclick={props.onremove.reform(move |_| id)} />
            </div>
        </li>
    }
}
