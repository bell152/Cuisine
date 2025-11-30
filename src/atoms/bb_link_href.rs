use stylist::{css, yew::styled_component, StyleSource};
use yew::prelude::*;
use yew_hooks::use_geolocation;
use yewdux::use_store;

use crate::store::YewduxStore;

#[derive(Clone, PartialEq)]
pub enum LinkType {
    Link,
    Button,
}
impl Default for LinkType {
    fn default() -> Self {
        Self::Link
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub text: String,
    pub href: String,
    #[prop_or_default]
    pub data_test: Option<String>,
    #[prop_or_default]
    pub link_type: Option<LinkType>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
}

#[styled_component(BBLinkHref)]
pub fn bb_link(props: &Props) -> Html {

    let link_type = props.link_type.clone().unwrap_or_default();
    let stylesheet = choose_stylesheet(link_type);
    let (_store, _dispatch) = use_store::<YewduxStore>();
    let _geo = use_geolocation();

    
    //let state = use_async(async move { fetch().await });


    let onclick = {
        let props_onclick = props.onclick.clone();
       // let state = state.clone();
        Callback::from(move |_event: MouseEvent| {
            //event.prevent_default();
            //let dispatch = dispatch.clone();
            //let event = event.clone();
           // state.run();
            let props_onclick = props_onclick.clone();
           
           /*  wasm_bindgen_futures::spawn_local(async move {
                let random: Result<String, crate::api::api_errors::ApiError> = get_random().await;
                match random {
                    Ok(random) => {
                        
                       dispatch.reduce_mut(  |store|{
                            log!("set state random: ",random.clone());
                            store.random = random;
                        });
                        },
                    Err(_) => { log!("Api Error")},
                }
            }); */
          //  let random: Result<String, crate::api::api_errors::ApiError> = get_random_sync();
        
        /*   if let Some(random) = state.data.clone() {
            let props_onclick = props_onclick.clone();
            let url = get_google_url(random);
            event.target().unwrap().unchecked_into::<HtmlAnchorElement>().set_href(&url);
            log!("real url:",url);
            props_onclick.unwrap().emit(event);
            }  */
         
         
         
          if let Some(_props_onclick) = props_onclick {
            
          }
           
        })
    };

    html!(
        <span data-test={props.data_test.clone()}>
            <a href={props.href.clone()} classes={classes!(stylesheet)} {onclick} >{ props.text.clone() }</a>
        </span>
    )
}

fn choose_stylesheet(link_type: LinkType) -> StyleSource {
    let style_source = css! {
        r#"
            color: antiqueblue;
            text-decoration: none;
            font-size:16px;
        "#
    };
    let link_sytle_sheet = style_source;
    let button_sytle_sheet = css! {
        r#"
            font-size:16px;
            text-decoration: none;
            background-color:aquamarine;
            padding: 3px;
            color:black;
            border-radius:4px;
            border: solid;
            margin: 0 10px;
        "#
    };
    match link_type {
        LinkType::Link => link_sytle_sheet,
        LinkType::Button => button_sytle_sheet,
    }
}
