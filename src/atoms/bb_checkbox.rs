use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data_test: String,
    #[prop_or_default]
    pub label: Option<String>,
    pub id: String,
    pub onchange: Callback<OnchangeData>,
    pub checked: bool,
}

#[derive(Clone, PartialEq)]
pub struct OnchangeData {
    pub selected: bool,
    pub id: String,
}

#[styled_component(BBCheckbox)]
pub fn bb_checkbox(props: &Props) -> Html {
    let stylesheet = css!(
        r#"
        span {
            font-size:32px;
        }
      input + label::before{
            content: '\a0';
            display:inline-block;
            vertical-align: 6px;
           
            border-radius: .2em;
            background-color:silver;
            text-indent: 25px;
            line-height: .65;
        }
         input:checked + label::before{
            content: '\2713';
            text-indent: 10px;
            width:  32px;
            height: 32px;
            background: yellowgreen;
        }
        /*input {
            position: absolute;
            clip: rect(0,0,0,0);
        }  */
        "#
    );

    let onchange = {
        let props_onchange: Callback<OnchangeData> = props.onchange.clone();
        let id = props.id.to_string();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .checked();
            props_onchange.emit(OnchangeData {
                selected: value,
                id: id.clone(),
            });
        })
    };
    html!(
        <div class={stylesheet}>
        if props.label.is_some() {
            <span>{props.label.clone().unwrap()}</span>
        }
            <input
            type="checkbox"
            id={props.id.clone()}
            data-test={props.data_test.clone()}
            value="completed"
            checked={props.checked}
            {onchange}
            />
            <label for={props.id.clone()}></label>
        </div>
    )
}
