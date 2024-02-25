use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use web_sys::wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, use_state, Properties};
use yew_hooks::prelude::*;

use crate::db_access::DbConnection;

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub on_one_person: Callback<Option<u32>>,
    pub db_connection: Rc<RefCell<DbConnection>>,
}

#[function_component]
pub fn PersonsList(props: &Props) -> Html {
    let db_connection = props.db_connection.clone();
    let on_one_person = props.on_one_person.clone();

    let filtered_persons = use_state(|| db_connection.borrow_mut().get_persons_by_partial_name(""));
    let id_to_find: UseStateHandle<Option<u32>> = use_state(|| None);
    let name_portion = use_state(|| AttrValue::from(""));
    let selected_ids = use_set(HashSet::<u32>::new());

    let on_change_name_portion = {
        let name_portion = name_portion.clone();
        move |e: Event| {
            name_portion.set(e.target_unchecked_into::<HtmlInputElement>().value().into());
        }
    };

    let on_find = {
        let filtered_persons = filtered_persons.clone();
        let id_to_find = id_to_find.clone();
        let db_connection = db_connection.clone();
        move |_| {
            filtered_persons.set(
                if let Some(p) = (*id_to_find)
                    .and_then(|id| db_connection.borrow_mut().get_person_by_id(id).cloned())
                {
                    vec![p.clone()]
                } else {
                    vec![]
                },
            )
        }
    };

    let on_filter = {
        let filtered_persons = filtered_persons.clone();
        let name_portion = name_portion.clone();
        let db_connection = db_connection.clone();
        move |_| {
            filtered_persons.set(
                db_connection
                    .borrow_mut()
                    .get_persons_by_partial_name(&name_portion),
            )
        }
    };

    let on_add = {
        let on_one_person = on_one_person.clone();
        move |_| on_one_person.emit(None)
    };

    let on_delete = {
        let db_connection = db_connection.clone();
        let selected_ids = selected_ids.clone();
        let filtered_persons = filtered_persons.clone();
        move |_| {
            for id in selected_ids.current().iter() {
                db_connection.borrow_mut().delete_by_id(*id);
            }
            filtered_persons.set(
                (*filtered_persons)
                    .iter()
                    .filter(|p| !selected_ids.current().contains(&(p.id)))
                    .cloned()
                    .collect(),
            );
            selected_ids.clear();
        }
    };

    html! {
        <div>
            <div>
                <label>{ "Id: " }</label>
                <input
                    type="number"
                />
                <button onclick={on_find}>{ "Find" }</button>
            </div>
            <div>
                <label>{ "Name portion: " }</label>
                <input
                    onchange={on_change_name_portion}
                />
                { " " }
                <button onclick={on_filter}>{ "Filter" }</button>
            </div>
            <button onclick={on_delete}>{ "Delete Selected Persons" }</button>
            { " " }
            <button onclick={on_add}>{ "Add New Person" }</button>
            {
                if filtered_persons.is_empty() {
                    html! {
                        <p>{ "No persons." }</p>
                    }
                } else {
                    html! {
                        <table>
                            <thead>
                                <th></th>
                                <th></th>
                                <th>{ "Id" }</th>
                                <th>{ "Name" }</th>
                            </thead>
                            <tbody>
                                {
                                    for filtered_persons.iter().map(|p| {
                                        let id = p.id.clone();
                                        let name = p.name.clone();
                                        let on_checked = {
                                            let selected_ids = selected_ids.clone();
                                            move |e: Event| {
                                                if e.target_unchecked_into::<HtmlInputElement>().checked() {
                                                    selected_ids.insert(id);
                                                } else {
                                                    selected_ids.remove(&id);
                                                }
                                            }
                                        };
                                        let on_edit = {
                                            let id = id.clone();
                                            let on_one_person = on_one_person.clone();
                                            move |_| {
                                                on_one_person.emit(Some(id))
                                            }
                                        };

                                        html! {
                                            <tr>
                                                <td>
                                                    <input
                                                        type="checkbox"
                                                        onchange={on_checked}
                                                        checked={selected_ids.current().contains(&id)}
                                                    />
                                                </td>
                                                <td>
                                                    <button onclick={on_edit}>{ "Edit" }</button>
                                                </td>
                                                <td>{ id }</td>
                                                <td>{ name }</td>
                                            </tr>
                                        }
                                    })
                                }
                            </tbody>
                        </table>
                    }
                }
            }
        </div>
    }
}
