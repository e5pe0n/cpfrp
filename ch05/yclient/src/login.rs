use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct LoginFormValues {
    pub username: String,
    pub password: String,
}

#[derive(Properties, PartialEq)]
pub struct LoginFormProps {
    pub on_submit: Callback<LoginFormValues>,
}

#[function_component]
pub fn LoginForm(
    LoginFormProps {
        on_submit: props_on_submit,
    }: &LoginFormProps,
) -> Html {
    let username = use_state(|| AttrValue::from(""));
    let password = use_state(|| AttrValue::from(""));

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
    let on_submit = {
        let username = username.clone();
        let password = password.clone();
        let props_on_submit = props_on_submit.clone();
        move |_| {
            props_on_submit.emit(LoginFormValues {
                username: (*username).to_string(),
                password: (*password).to_string(),
            })
        }
    };

    html! {
        <form onsubmit={on_submit}>
            <div>
                <label>{ "User name: " }</label>
                <input value={(*username).to_string()} onchange={on_change_username} />
            </div>
            <div>
                <label>{ "Password: " }</label>
                <input value={(*password).to_string()} onchange={on_change_password} />
            </div>
            <button type="submit">{ "Log in" }</button>
        </form>
    }
}
