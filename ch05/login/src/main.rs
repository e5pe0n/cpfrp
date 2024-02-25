use std::cell::RefCell;
use std::rc::Rc;

use yew::prelude::*;

use crate::db_access::{DbConnection, Person};
use crate::login::{FormValues, LoginForm};
use crate::one_person::OnePerson;
use crate::persons_list::PersonsList;

mod db_access;
mod login;
mod one_person;
mod persons_list;

enum Page {
    Login,
    PersonsList,
    OnePerson,
}

#[function_component]
fn App() -> Html {
    let db_connection = use_state(|| Rc::new(RefCell::new(DbConnection::new())));
    let current_user: UseStateHandle<Option<String>> = use_state(|| None);
    let page = use_state(|| Page::Login);
    let op: UseStateHandle<Option<Person>> = use_state(|| None);

    let on_change_user = {
        let page = page.clone();
        move |_| {
            page.set(Page::Login);
        }
    };

    let on_login = {
        let current_user = current_user.clone();
        let page = page.clone();
        let db_connection = db_connection.clone();
        move |data: FormValues| {
            if let Some(user) = (*db_connection)
                .borrow_mut()
                .get_user_by_username(&data.username)
            {
                if user.password == data.password {
                    current_user.set(Some(user.username.clone()));
                    page.set(Page::PersonsList);
                } else {
                    web_sys::window()
                        .unwrap()
                        .alert_with_message("Invalid password for the specified user.")
                        .expect("should alert invalid password");
                }
            } else {
                web_sys::window()
                    .unwrap()
                    .alert_with_message("User not found.")
                    .expect("should alert user not found");
            }
        }
    };

    let on_one_person = Callback::from({
        let page = page.clone();
        let op = op.clone();
        let db_connection = db_connection.clone();
        move |v: Option<u32>| {
            op.set(match v {
                Some(i) => (*db_connection).borrow_mut().get_person_by_id(i).cloned(),
                _ => None,
            });
            page.set(Page::OnePerson);
        }
    });

    let on_back = Callback::from({
        let page = page.clone();
        let op = op.clone();
        move |_| {
            op.set(None);
            page.set(Page::PersonsList);
        }
    });

    html! {
        <div>
            <style>
            {
                ".current-user { color: #0000C0 }"
            }
            </style>
            <header>
                <h2>{ "Persons management" }</h2>
                <p>
                    { "Current user: " }
                    <span class="current-user">
                    {
                        if let Some(user) = (*current_user).clone() {
                            user
                        } else {
                            "---".to_string()
                        }
                    }
                    </span>
                    {
                        match &(*page) {
                            Page::Login => html! { <div/> },
                            _ => html! {
                                <span>
                                    { " " }
                                    <button onclick={on_change_user}>{ "Change User" }</button>
                                </span>
                            }
                        }
                    }
                </p>
                <hr/>
            </header>
            {
                match &(*page) {
                    Page::Login => html! {
                        <LoginForm {on_login} />
                    },
                    Page::PersonsList => html! {
                        <PersonsList {on_one_person} db_connection={(*db_connection).clone()} />
                    },
                    Page::OnePerson => html! {
                        <OnePerson op={(*op).clone()} db_connection={(*db_connection).clone()} on_back={on_back} />
                    }
                }
            }
            <footer>
                <hr/>
                { "\u{A9} Carlo Milanesi - Developed using Yew" }
            </footer>
        </div>
    }
}

// https://yew.rs/docs/getting-started/build-a-sample-app#view-your-web-application
fn main() {
    yew::Renderer::<App>::new().render();
}
