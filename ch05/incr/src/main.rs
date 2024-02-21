use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick_incr = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let onclick_reset = {
        let counter = counter.clone();
        Callback::from(move |_| {
            counter.set(0);
        })
    };
    let onkeydown = {
        let counter = counter.clone();
        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "+" => counter.set(*counter + 1),
            "0" => counter.set(0),
            _ => {}
        })
    };

    html! {
        <div>
            <button onclick={onclick_incr}>{ "Increment" }</button>
            <button onclick={onclick_reset}>{ "Reset" }</button>
            <input readonly={true} value={(*counter).to_string()} onkeydown={onkeydown} />
        </div>
    }
}

// https://yew.rs/docs/getting-started/build-a-sample-app#view-your-web-application
fn main() {
    yew::Renderer::<App>::new().render();
}
