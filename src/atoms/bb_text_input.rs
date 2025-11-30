use std::ops::Deref;

use stylist::{css, yew::styled_component};
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum InputType {
    Text,
    Password,
}
impl ToString for InputType {
    fn to_string(&self) -> String {
        match self {
            InputType::Text => "text".to_string(),
            InputType::Password => "password".to_string(),
        }
    }
}
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub label: String,
    pub data_test: String,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    pub input_type: InputType,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub value: Option<String>,
}

#[styled_component(BBTextInput)]
pub fn bb_text_input(props: &Props) -> Html {
    let sytlesheet = css!(
        r#"
        label {
            font-size:16px;
        }
        input {
            font-size:16px;
            width:100%;
        }
        "#
    );

    let placeholder = props.placeholder.clone().unwrap_or_default();
    let id = props.label.to_lowercase().replace(" ", "-");
    let class = props.class.clone().unwrap_or_default();
    let state = use_state(|| String::new());
    let initial_load = use_state(|| false);

    let handle_onchange = {
        let handle_onchange = props.onchange.clone();
        let state = state.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            handle_onchange.emit(value.clone());
            state.set(value);
        })
    };
    {
        let state = state.clone();
        let props_value = props.value.clone();
        let initial_load = initial_load.clone();
        use_effect(move || {
            if props_value.is_some() && !props_value.as_ref().unwrap().is_empty() && !*initial_load
            {
                state.set(props_value.unwrap());
                initial_load.set(true);
            };
            || ()
        });
    }

    //let value = props.value.clone().unwrap_or_default();
    html!(
        <div class={classes!(sytlesheet,class)}>
            <div>
                <label for={id.clone()}>{&props.label} </label>
            </div>
            <div >
                <input type={props.input_type.to_string()} id={id} {placeholder} data-test={props.data_test.clone()} onchange = { handle_onchange} autocomplete="off"
                value = {state.deref().clone()}

                  />
            </div>
            <p>{&*state}</p>
        </div>
    )
}
