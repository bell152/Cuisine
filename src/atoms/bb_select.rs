use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub is_selected: bool,
}
impl SelectOption {
    pub fn new(value: &str, label: &str, is_selected: bool) -> Self {
        Self {
            value: value.to_owned(),
            label: label.to_owned(),
            is_selected,
        }
    }
}
#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data_test: String,
    pub id: String,
    pub label: String,
    pub options: Vec<SelectOption>,
    pub onchange: Callback<String>,
}

#[styled_component(BBSelect)]
pub fn bb_select(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        label {
            font-size:32px;
        }
        select {
            font-size:16px;
            width:100%;
        }
    "#
    );
    let onchange = {
        let change = props.onchange.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlSelectElement>()
                .value();
            change.emit(value);
        })
    };
    html!(
        <div class={stylesheet}>
            <label for={props.id.clone()}>{&props.label}</label>
            <select id={props.id.clone()} {onchange} data_test={props.data_test.clone()}>
                {create_option_tag(props.options.clone()) }
            </select>
        </div>
    )
}

fn create_option_tag(select_optios: Vec<SelectOption>) -> Vec<Html> {
    select_optios
        .iter()
        .map(|select_option| {
            html! {
               <option value={select_option.value.clone()} selected = {select_option.is_selected}>
                   {select_option.label.clone()}
               </option>
            }
        })
        .collect()
}
