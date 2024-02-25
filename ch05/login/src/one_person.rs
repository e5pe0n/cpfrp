use std::cell::RefCell;
use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;

use crate::db_access::InsertingPerson;
use crate::db_access::{DbConnection, Person};

#[derive(PartialEq, Clone, Properties)]
pub struct OnePersonProps {
    pub op: Option<Person>,
    pub db_connection: Rc<RefCell<DbConnection>>,
    pub on_back: Callback<()>,
}

#[function_component]
pub fn OnePerson(props: &OnePersonProps) -> Html {
    let OnePersonProps {
        op,
        db_connection,
        on_back,
    } = (*props).clone();
    let is_inserting = op.is_none();

    let id = use_state(|| match &op {
        Some(p) => AttrValue::from(p.id.to_string()),
        _ => AttrValue::from(""),
    });
    let name = use_state(|| match &op {
        Some(p) => AttrValue::from(p.name.clone()),
        _ => AttrValue::from(""),
    });

    let on_change_name = {
        let name = name.clone();
        move |e: Event| {
            name.set(AttrValue::from(
                e.target_unchecked_into::<HtmlInputElement>().value(),
            ))
        }
    };

    let on_insert = {
        let name = name.clone();
        let db_connection = db_connection.clone();
        let on_back = on_back.clone();
        Callback::from(move |_| {
            db_connection.borrow_mut().insert_person(InsertingPerson {
                name: (*name).to_string(),
            });
            on_back.emit(());
        })
    };
    let on_update = {
        let id = id.clone();
        let name = name.clone();
        let db_connection = db_connection.clone();
        let on_back = on_back.clone();
        Callback::from(move |_| {
            db_connection.borrow_mut().update_person(Person {
                id: (*id).parse().unwrap(),
                name: (*name).to_string(),
            });
            on_back.emit(());
        })
    };

    html! {
        <div>
            <div>
                <label>{ "Id: " }</label>
                <input
                    type="number"
                    disabled=true
                    value={(*id).clone()}
                />
            </div>
            <div>
                <label>{ "Name: " }</label>
                <input
                    value={(*name).clone()}
                    onchange={on_change_name}
                />
            </div>
            <div>
                <button
                    onclick={if is_inserting { on_insert } else { on_update } }
                >{ if is_inserting { "Insert" } else { "Update" } }</button>
                { " " }
                <button>{ "Cancel" }</button>
            </div>
        </div>
    }
}
