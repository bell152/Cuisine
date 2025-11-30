

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew::suspense::use_future;

use crate::api::request::get_user_one;

#[derive(Debug,Deserialize,Serialize)]
pub struct CuisineUser {
    pub name: String,
}

#[function_component(Content)]
pub fn content() -> HtmlResult {
    //let user = use_user()?;
    //let response = get_user_one().await; 
    let res = use_future(|| async { get_user_one().await })?;
    let result: Option<&CuisineUser> = match *res {
        Ok(ref res) =>  Some(res) ,
        Err(_) => None,
    };
    let tt = result.unwrap();
    
    Ok(html! { <div>{"Hello,我是阻塞的content: "}{tt.name.clone()}</div>})
} 
