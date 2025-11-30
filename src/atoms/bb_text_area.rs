use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub data_test: String,
    #[prop_or_default]
    pub value: Option<String>,
    pub onchange: Callback<String>,
    pub label: String,
    pub id: String,
}

#[styled_component(BBTextArea)]
pub fn bb_text_field(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
    label {
        font-size:16px;
    }
    textarea {
        font-size:16px;
        width:100%;
    }
    "#
    );

    let state = use_state(|| String::new());
    let has_load = use_state(|| false);
    let onchange = {
        let props_onchange = props.onchange.clone();
        let state = state.clone();
        Callback::from(move |event: Event| {
            let change = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlTextAreaElement>()
                .value();
            props_onchange.emit(change.clone());
            state.set(change);
        })
    };
    {
        let state = state.clone();
        let has_load = has_load.clone();
        let value = props.value.clone();
        use_effect(move || {
            if !*has_load && state.is_empty() && value.is_some() {
                state.set(value.unwrap());
                has_load.set(true);
            }
            || ()
        })
    }
    html! {
        <div class ={stylesheet}>
            <label for={props.id.clone()}>{props.label.clone()}</label>
            <textarea id={props.id.clone()} data-test={props.data_test.clone()} value={(&*state).clone()} onchange={onchange} />
        </div>
    }
}
