use std::{ops::Deref, rc::Rc};

use gloo::console::log;
use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::{ HtmlDocument, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;
use yewdux:: prelude::*;

use crate::{api::request::get_random, atoms::bb_link_href::{BBLinkHref, LinkType}, router::Route, store::YewduxStore, utils::get_google_url::get_google_url, MessageContext};


#[styled_component(Navbar)]
pub fn navbar() -> Html {

    let mut nav_home_actived = "";
    // let (store,dispatch) = use_store::<YewduxStore>();
    //let location = use_location().unwrap().path().to_owned();
    // log!("location: ", location);
    let current_route = use_route::<Route>().unwrap();
    if matches!(&current_route, Route::Home) {
        nav_home_actived = "active";
    }
    let global_ctx = use_context::<MessageContext>().unwrap();
   
    let state = use_state(|| String::new());
    let _handle_onchange = {
        let state_clone = state.clone();
        Callback::from(move |event: Event| {
            let value = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state_clone.set(value);
        })
    };

    /*  Callback::from(move |event:SubmitEvent|{
           event.prevent_default();
    */
    let (store, dispatch) = use_store::<YewduxStore>();
    let dispatch_for_cookie = dispatch.clone();
    let dispatch_for_store_clear = dispatch.clone();
    let dispatch_for_logout = dispatch.clone();

    let store_random = store.clone();
    let store_name = store.clone();
    let store_token = store.clone();

    let navigator = use_navigator().unwrap();
    //点击获取随机数,弹出登录框
    let onclick: Callback<MouseEvent> = {
        Callback::from(move |event:MouseEvent| {
            event.prevent_default();
            let dispatch = dispatch.clone();
            let rundom_old = dispatch.get().random.clone();
            if rundom_old.is_empty(){
                wasm_bindgen_futures::spawn_local(async move {
                    let response: Result<String, crate::api::api_errors::ApiError> = get_random().await;
                    match response {
                        Ok(random) => {
                            dispatch.reduce_mut( |store|{
                                store.random = random;
                            });
                            },
                        Err(_) => { 
                            log!("Api Error");
                            dispatch.reduce_mut(  |store|{
                                store.random = "".to_string();
                            });
                        },
                    }
                });
            }
            
        })
    };

    let is_login_visible = use_state(||String::from("visible"));
    let is_href_visible: UseStateHandle<String> = use_state(||String::from("invisible"));

    let onclick_logout= {
        let is_login_visible = is_login_visible.clone();
        let is_href_visible = is_href_visible.clone();
        Callback::from(move|event:MouseEvent|{
            let  global_ctx = global_ctx.clone();
            event.prevent_default();
            //清空cookies
            let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
            let cookie = format!("token=; Max-Age=0;");
            let _ = document.set_cookie(cookie.as_str());
            dispatch_for_logout.reduce(  |_store: Rc<YewduxStore>|
                YewduxStore{
                    ..Default::default()
                }.into()
            );
             //重置登录按钮
            is_login_visible.set(String::from("visible"));
            is_href_visible.set(String::from("invisibel"));
            //设置home页面需要刷新
           // global_ctx.set_need_reload(true);
            //log!("logout, is_need_reload  ", global_ctx.get_need_reload().to_owned());
            global_ctx.dispatch(true);
            //log!("logout: is_need_reload; ",global_ctx.get_need_reload().to_owned());
            navigator.push(&Route::Home);
        })
    };

    { 
        let is_login_visible: UseStateHandle<String> = is_login_visible.clone();
        let is_href_visible: UseStateHandle<String> = is_href_visible.clone();
        use_effect_with( (),move |_| {
            let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
            let cookies = document.cookie().unwrap();
            if cookies.is_empty()  {
                log!("cookie is emplty");
                //token没有找到,也许是token过期了
                dispatch_for_store_clear.reduce_mut(|store|{
                    store.token = "".to_string();
                    store.username = "".to_string();
                });
               return 
            }
             //处理登录回调的token
            if !cookies.is_empty() {
                log!("检查token:已登录");
                for one_cookie in cookies.split(";"){
                    let key_value:Vec<&str> = one_cookie.split("=").collect();
                    if !key_value.is_empty() && key_value.len() >0 {
                        let key = key_value[0];
                        let value = key_value[1];
                        if "token".eq(key.trim()) {
                            let user_info :Vec<&str>= value.split(":").collect();
                            dispatch_for_cookie.reduce_mut(|store|{
                                store.token = value.to_string();
                                store.username = user_info[2].to_string();
                            });
                            if (*is_login_visible =="visible") && (*is_href_visible =="invisible") {
                                is_login_visible.set(String::from("invisible"));
                                is_href_visible.set(String::from("visibel"));
                            }
                            break;
                        }
                    }
                        
                } 
            }
            
        })
    }

    html!(
        
        <nav class={"navbar navbar-expand-lg bg-primary-subtle bg-gradient"} >
            <div class="container-fluid">
                //<a class="navbar-brand text-li" href="#" >{"Home"}</a>
                <Link<Route> to={Route::Home} classes={classes!("navbar-brand", "text-li")} >{"CUISINE"}</Link<Route>>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarSupportedContent">
                    <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                        <li class="nav-item">
                           // <a class={classes!("nav-link",nav_is_active)} aria-current="page" href="#">{"onePage"}</a>
                            <Link<Route> to={Route::Home} classes={classes!("nav-link",nav_home_actived)} >{"Home Page"}</Link<Route>>
                        </li>
                        <li class="nav-item">
                            <Link<Route> to={Route::Privacy} classes={classes!("nav-link")} >{"Privacy"}</Link<Route>>
                        </li>
                       /*  <li class="nav-item">
                            <Link<Route> to={Route::TxList} classes={classes!("nav-link",nav_tx_actived)} >{"Create Tx"}</Link<Route>>
                           // <a class="nav-link" href="#">{"Create Tx"}</a>
                        </li> */
                       /*  <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                            {"Contract"}
                            </a>
                            <ul class="dropdown-menu bg-info bg-gradient dropdown-menu-dark">
                                <li><a class="dropdown-item text-dark" href="#">{"Create Contract"}</a></li>
                                <li><a class="dropdown-item text-dark" href="#">{"Another action"} </a></li>
                                <li> <hr class="dropdown-divider text-dark"/> </li>
                                <li><a class="dropdown-item text-dark" href="#">{"Something else here"}</a></li>
                            </ul>
                        </li> */
                           
                    </ul>
                  /*   <form class="d-flex mx-2" role="search"  >
                        <input id="searchid" class="form-control me-2" type="search" placeholder="Search" aria-label="Search"  onchange = { handle_onchange}/>
                        <button class="btn btn-outline-success" type="button">{"Search"}</button>
                    </form> */
                /*
                    唯有美食与爱不可辜负
                 */
                    <h3 class="me-5 text-center text-info">{"Love deeply, eat well"}</h3>
                </div>
                {
                    create_nologin_or_logined(store_token,is_login_visible,is_href_visible,onclick,onclick_logout,store_name.username.clone())
                }
                <div class="modal fade" id="staticBackdrop-login"  data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                    <div class="modal-dialog ">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h1 class="modal-title fs-5" id="staticBackdropLabel">{"Login"}</h1>
                                <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                            </div>
                            <div class="modal-body text-center">
                                {   
                                   create_login(store_random.random.clone())
                                }
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal"  /* onclick = {button_cancel} */>{"Close"}</button>
                               /*  <button type="button" class="btn btn-primary">{"Save changes"}</button> */
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    )
}

pub fn is_logged_in(token: &str) -> bool {
    log!("logined??: ", !token.is_empty());
    !token.is_empty()
}

pub fn create_login(random: String) ->Html{
    if random.is_empty(){
        html!{
            <div class="spinner-border text-primary" role="status">
                <span class="visually-hidden">{"Loading..."}</span>
            </div>
        }
    }else{
        let url = get_google_url(random);
        html!{
            <a href={url}>
                <img class="pr-2" src="static/google.svg" alt="" style={"height: 2.2rem"} />
                {" Continue with Google"}
            </a>
        }
    }
   
}

pub fn create_nologin_or_logined(store_token: Rc<YewduxStore>,is_login_visible:UseStateHandle<String> ,is_href_visible: UseStateHandle<String>, onclick: Callback<MouseEvent>, onclick_logout: Callback<MouseEvent>, store_name:String) -> Html{
    if store_token.token.is_empty(){
        //未登录
        html!{
            <div class={is_login_visible.deref()}>
                <button  class={classes!("btn", "btn-outline-success",is_login_visible.deref())} type="button" data-bs-toggle="modal" data-bs-target="#staticBackdrop-login"  {onclick} >{"Login"} </button>
            </div>
        }
    }else{
        html!{
            <>
                <div class={is_href_visible.deref()}>
                    <BBLinkHref text={store_name} href={""} link_type={LinkType::Button} > </BBLinkHref>
                </div>
                <div class={classes!{is_href_visible.deref(),"ms-1"}}>
                    <button type="button" class="btn btn-primary" onclick = {onclick_logout}>
                        {" 退出 "}
                    </button> 
                </div>
            </>
        }
    }
   
}