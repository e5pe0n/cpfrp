use web_sys::wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::common::{request, LoggingUser, User};
use crate::login::{LoginForm, LoginFormValues};
use crate::persons_list::PersonsList;

mod common;
mod login;
mod persons_list;

enum Page {
    Login,
    PersonsList,
    OnePerson,
}

#[function_component]
fn App() -> Html {
    let username: UseStateHandle<Option<String>> = use_state(|| None);
    let page = use_state(|| Page::Login);

    let on_submit_login_form = {
        let username = username.clone();
        // let page = page.clone();
        Callback::from(move |data: LoginFormValues| {
            // username.set(Some(data.username));
            // let username = username.clone();
            // let page = page.clone();
            // let data = data.clone();
            // web_sys::console::log_1(&JsValue::from(format!(
            //     "username={}, password={}",
            //     data.username, data.password
            // )));
            // yew::platform::spawn_local(async move {
            //     match request::<(), User>(
            //         reqwest::Method::GET,
            //         "/authenticate".to_string(),
            //         Some(LoggingUser {
            //             username: data.username.clone(),
            //             password: data.password.clone(),
            //         }),
            //         (),
            //     )
            //     .await
            //     {
            //         Ok(u) => {
            //             username.set(Some(u.username.clone()));
            //             page.set(Page::PersonsList);
            //         }
            //         Err(msg) => {
            //             web_sys::console::log_1(&JsValue::from(msg));
            //         }
            //     }
            // });
        })
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
                    { "Current User: " }
                    <span class="current-user">
                    {
                        if let Some(username) = (*username).clone() {
                            username.clone()
                        } else {
                            "---".to_string()
                        }
                    }
                    </span>
                    {
                        match *page {
                            Page::Login => html! { <div/> },
                            _ => html! {
                                <span>
                                    {" "}
                                    <button>{ "Change User" }</button>
                                </span>
                            }
                        }
                    }
                </p>
                <hr/>
            </header>
            {
                match *page {
                    Page::Login => html! {
                        <LoginForm on_submit={on_submit_login_form} />
                    },
                    Page::PersonsList => html! {
                        <PersonsList />
                    },
                    Page::OnePerson => html! {},
                }
            }
            <footer>
                <hr/>
                { "\u{A9} Carlo Milanesi - Developed using Yew and Actix-web" }
            </footer>
        </div>
    }
}

// https://yew.rs/docs/getting-started/build-a-sample-app#view-your-web-application
fn main() {
    yew::Renderer::<App>::new().render();
}
