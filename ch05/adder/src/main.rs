use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let addend1 = use_state(|| "".to_string());
    let addend2 = use_state(|| "".to_string());
    let sum = use_state(|| None);

    let onchangeaddend1 = {
        let addend1 = addend1.clone();
        move |e: Event| addend1.set(e.target_unchecked_into::<HtmlInputElement>().value())
    };

    let onchangeaddend2 = {
        let addend2 = addend2.clone();
        move |e: Event| addend2.set(e.target_unchecked_into::<HtmlInputElement>().value())
    };

    let onclickadd = {
        let addend1 = addend1.clone();
        let addend2 = addend2.clone();
        let sum = sum.clone();
        move |_| {
            sum.set(
                match ((*addend1).parse::<f64>(), (*addend2).parse::<f64>()) {
                    (Ok(a1), Ok(a2)) => Some(a1 + a2),
                    _ => None,
                },
            );
        }
    };

    let num_style = "text-align: right;";

    html! {
        <table>
            <tr>
                <td>{ "Addend 1:" }</td>
                <td>
                    <input type="number" style={num_style} value={(*addend1).to_string()} onchange={onchangeaddend1} />
                </td>
            </tr>
            <tr>
                <td>{ "Addend 2:" }</td>
                <td>
                    <input type="number" style={num_style} value={(*addend2).to_string()} onchange={onchangeaddend2} />
                </td>
            </tr>
            <tr>
                <td></td>
                <td align="center">
                    <button onclick={onclickadd} >{ "Add" }</button>
                </td>
            </tr>
            <tr>
                <td>{ "Sum" }</td>
                <td>
                    <input readonly=true type="number"
                        style={num_style.to_string() + "background-color: " + if sum.is_some() { "lightgreen;" } else { "yellow;" }}
                        value={match *sum { Some(n) => n.to_string(), None => "".to_string() }}
                    />
                </td>
            </tr>
        </table>
    }
}

// https://yew.rs/docs/getting-started/build-a-sample-app#view-your-web-application
fn main() {
    yew::Renderer::<App>::new().render();
}
