use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, use_state};

pub struct FormValues {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_login: Callback<FormValues>,
}

#[function_component]
pub fn LoginForm(props: &Props) -> Html {
    let username = use_state(|| AttrValue::from(""));
    let password = use_state(|| AttrValue::from(""));
    let on_login = props.on_login.clone();

    let on_change_username = {
        let username = username.clone();
        move |e: Event| {
            username.set(AttrValue::from(
                e.target_unchecked_into::<HtmlInputElement>().value(),
            ));
        }
    };

    let on_change_password = {
        let password = password.clone();
        move |e: Event| {
            password.set(AttrValue::from(
                e.target_unchecked_into::<HtmlInputElement>().value(),
            ));
        }
    };

    let on_login = {
        let username = username.clone();
        let password = password.clone();
        move |_| {
            on_login.emit(FormValues {
                username: (*username).to_string(),
                password: (*password).to_string(),
            });
        }
    };

    html! {
        <div>
            <div>
                <label>{ "User name: " }</label>
                <input
                    value={(*username).clone()}
                    onchange={on_change_username}
                />
            </div>
            <div>
                <label>{ "Password: " }</label>
                <input
                    type="password"
                    value={(*password).clone()}
                    onchange={on_change_password}
                />
            </div>
            <button onclick={on_login}>{ "Log in" }</button>
        </div>
    }
}
