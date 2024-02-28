use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::common::Person;

#[function_component]
pub fn PersonsList() -> Html {
    let filtered_persons = use_list::<Person>(vec![]);

    html! {
        <div>
            <div>
                <label>{ "Id: " }</label>
                <input type="number" />
                { " " }
                <button>{ "Find" }</button>
            </div>
            <div>
                <label>{ "Name portion: " }</label>
                <input />
                { " " }
                <button>{ "Filter" }</button>
            </div>
            <button>{ "Delete Selected Persons" }</button>
            { " " }
            <button>{ "Add New Person" }</button>
            {
                if filtered_persons.current().is_empty() {
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
                                for filtered_persons.current().iter().map(|p| {
                                    let id = p.id.clone();
                                    let name = p.name.clone();
                                    html! {
                                        <tr>
                                            <td>
                                                <input type="checkbox" />
                                            </td>
                                            <td>
                                                <button>{ "Edit" }</button>
                                            </td>
                                            <td>{ id.to_string() }</td>
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
