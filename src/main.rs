use gloo::storage::{LocalStorage, Storage};
use state::{Action, State, Update, User};
use yew::{classes, function_component, html, use_effect_with_deps, use_reducer, Callback};

mod components;
mod state;

use components::{
    // footer::Footer,
    header::Header,
    update::Update as UpdateItem,
};

const KEY: &str = "1password.watchtowerrace.self";

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| State {
        user: User {
            id: 1,
            name: "Andrew".to_owned(),
            score: 0,
        },
        updates: LocalStorage::get(KEY).unwrap_or_else(|_| vec![]),
    });

    // Effect
    use_effect_with_deps(
        move |state| {
            LocalStorage::set(KEY, &state.clone().updates).expect("failed to set");
            || ()
        },
        state.clone(),
    );

    // Callbacks
    let onadd = {
        let state = state.clone();
        Callback::from(move |update: Update| {
            state.dispatch(Action::Add(update));
        })
    };

    let onremove = {
        let state = state.clone();
        Callback::from(move |id: usize| state.dispatch(Action::Remove(id)))
    };

    // Helpers
    let hidden_class = if state.updates.is_empty() {
        "hidden"
    } else {
        ""
    };

    let current_score = if !state.updates.is_empty() {
        state.updates.last().unwrap().score.parse::<u16>().unwrap()
    } else {
        0
    };

    let remark = match current_score {
        1000.. => "Ok fine you get a prize.",
        750...999 => "Security is a lifestyle. You gotta want it!",
        500...749 => "It's the thought that counts?",
        250...499 => "Mom, is that you?",
        _ => "Ugh",
    };

    // let cumulative_score: u16 = state
    //     .updates
    //     .iter()
    //     .map(|update| update.score.clone().parse::<u16>().unwrap())
    //     .sum();

    html! {
        <div class="race-wrapper">
            <section class="raceapp">
                <header class="header">
                    <h1>{ "Watchtower Race" }</h1>
                    <Header {onadd} user_id=1 />
                </header>
                <section class={classes!("main", hidden_class)}>
                    <ul class="score-list">
                        { for state.updates.iter().cloned().map(|update|
                            html! {
                                <UpdateItem {update}
                                    onremove={onremove.clone()}
                                />
                        }) }
                    </ul>
                </section>
                <section>
                        <strong>
                            {
                                format!(
                                    "Current Score: {:?} - {}",
                                    if !state.updates.is_empty() { current_score } else { 0 },
                                    remark
                                )
                            }
                        </strong>
                </section>
                <footer class={classes!("footer", hidden_class)}>
                    <span class="score-count">
                        <strong>{ state.updates.len() }</strong>
                        { format!(" score{} submitted", if state.updates.len() > 1 { "s" } else { "" } ) }
                    </span>
                </footer>
            </section>
            // <Footer />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
