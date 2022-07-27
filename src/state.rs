use std::rc::Rc;
use yew::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct State {
    pub user: User,
    pub updates: Vec<Update>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub score: u8,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub id: usize,
    pub user_id: usize,
    pub score: String,
    // pub date: String,
}

pub enum Action {
    Add(Update),
    Remove(usize),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(update) => {
                let mut updates = self.updates.clone();
                updates.push(Update {
                    id: updates.last().map(|entry| entry.id + 1).unwrap_or(1),
                    user_id: update.user_id,
                    score: update.score,
                    // date: update.date,
                });
                State {
                    user: self.user.clone(),
                    updates,
                }
                .into()
            }
            Action::Remove(id) => {
                let user = self.user.clone();
                let mut updates = self.updates.clone();
                updates.retain(|entry| entry.id != id);
                State { user, updates }.into()
            }
        }
    }
}
