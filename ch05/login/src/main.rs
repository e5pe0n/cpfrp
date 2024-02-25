use yew::prelude::*;

use crate::db_access::DbConnection;
use crate::login::{FormValues, LoginForm};

mod db_access;
mod login;

enum Page {
    Login,
    PersonsList,
}

#[function_component]
fn App() -> Html {
    let current_user: UseStateHandle<Option<String>> = use_state(|| None);
    let page = use_state(|| Page::Login);
    let db_connection = DbConnection::new();

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
            if let Some(user) = db_connection.get_user_by_username(&data.username) {
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
                        <h2>{ "Page to be implemented" }</h2>
                    },
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
