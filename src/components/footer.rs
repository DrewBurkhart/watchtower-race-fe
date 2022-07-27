use yew::{function_component, html};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="info">
            <p>{ "Written by " }<a href="https://github.com/drewburkhart/" target="_blank">{ "Andrew Burkhart <drewburkhart>" }</a></p>
            <p>{ "From " }<a href="https://1password.com/" target="_blank">{ "1Password" }</a></p>
        </footer>
    }
}
