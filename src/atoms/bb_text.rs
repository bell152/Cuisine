use stylist::{css, yew::styled_component};
use yew::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum TextType {
    Normal,
    Title,
}
impl Default for TextType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
    Danger,
    Info,
    Normal,
}
impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Danger => "danger",
            Color::Info => "info",
            Color::Normal => "normal",
        }
        .to_owned()
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    pub data_test: String,
    #[prop_or_default]
    pub text_type: Option<TextType>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
}

#[styled_component(BBText)]
pub fn bb_text(props: &Props) -> Html {
    let text_type = props.text_type.unwrap_or_default();
    let color = props.color.unwrap_or_default();
    let stylesheet = css! {
    r#"
        .danger {
            color:red;
        }
        .info {
            color:yellow;
        }
        .normal {
            color:green;
        }
    "#
    };
    html! {
        <span class={stylesheet}>{
            match text_type{
                TextType::Normal => normal_text(props.data_test.clone(),&props.text,color.to_string()),
                TextType::Title => title_text(props.data_test.clone(),&props.text,color.to_string()),
            }
        }
       </span>
    }
}

pub fn normal_text(data_test: String, text: &str, class: String) -> Html {
    let stylesheet = css!(
        r#"
        font-size: 18px;
    "#
    );

    html!(
        <p class={classes!(stylesheet,class)} data-test={data_test}> {text} </p>
    )
}

pub fn title_text(data_test: String, text: &str, class: String) -> Html {
    let stylesheet = css!(
        r#"
        font-size: 36px;
    "#
    );

    html!(
        <h1 class={classes!(stylesheet,class)} data-test={data_test}>{text} </h1>
    )
}
