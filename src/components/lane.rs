use yew::events::MouseEvent;
use yew::{classes, function_component, html, Callback, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct HeaderProps {
    pub name: String,
    pub score: u16,
    pub position: u8,
}

#[function_component(Lane)]
pub fn lane(props: &HeaderProps) -> Html {
    let mut show_info = false;

    let onclick = {
        let name = props.name.to_owned();
        let score = props.score;
        let position = props.position;
        let onclick = Callback::from(move |_: MouseEvent| {
            println!("{} {} {}", name, score, position);
        });
        show_info = !show_info;
        onclick
    };

    let show_info_class = if show_info { "show-info" } else { "" };

    html! {
        <div class="lane" {onclick}>
            <h1 class={classes!("horse", show_info_class)}>{ "Horse" }</h1>
            { if show_info {
                html! {
                    <div class="info">
                        <p>{props.name.clone()}</p>
                        <p>{props.score}</p>
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
