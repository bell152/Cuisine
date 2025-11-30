
use std::rc::Rc;
use crate::{api::{api_errors::ApiError, request::{get_cuisine_detail, get_cuisine_list, get_curr_user_cuisine_list, get_random}}, organisms::navbar::create_login,  store::YewduxStore, utils::get_google_url::get_paypal_url};
use gloo::console::log;
use stylist::yew::styled_component;
use web_sys::{HtmlDocument, HtmlInputElement};
use yew::prelude::*;
use wasm_bindgen::JsCast;
use yewdux::use_store;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_need_reload: bool,
}

#[styled_component(Home)]
pub fn home() -> Html {
    let (store, dispatch) = use_store::<YewduxStore>();

    //let global_ctx = use_context::<MessageContext>().unwrap();

    let is_visible = use_state(||String::from("invisible"));
    let dispatch_for_cuisine = dispatch.clone();
    let dispatch_for_pay = dispatch.clone();
    let store_view_or_login = store.clone();
    let store_create_list = store.clone();
    let store_for_detail = store.clone();
    let store_token = store.token.clone();
    use_effect_with(store_token.clone(), move |_| {
        if store.token.is_empty(){
            //未登录,获取通用列表
            log!("触发首页effect: 获取 未登录 列表");
            wasm_bindgen_futures::spawn_local(async move {
                let response = get_cuisine_list().await;
                match response {
                    Ok(item) => {
                        dispatch_for_cuisine.reduce_mut(|x|{
                            x.cuisine_title_list = item;
                        });
                    },
                    Err(_e) => {
                        log!("没有获取到列表");
                    },
                }
            });
        }else if !store.token.is_empty(){
            //有token,代表登录,获取通用列表 和 当前用户列表
            log!("触发首页effect: 获取当前用户列表, 状态:");
            wasm_bindgen_futures::spawn_local(async move {
                let response = get_curr_user_cuisine_list().await;
                match response {
                    Ok(item) => {
                        dispatch_for_cuisine.reduce_mut(|x|{
                            x.cuisine_title_list = item;
                            log!("effect->home,更新了 当前登录 用户列表");
                        });
                    },
                    Err(e) => {
                        match e {
                            ApiError::NotAuthenticated => {
                                //清空cookies
                                log!("expired");
                                let document = gloo::utils::document().unchecked_into::<HtmlDocument>();
                                let cookie = format!("token=; Max-Age=0;");
                                let _ = document.set_cookie(cookie.as_str());
                              
                            },
                            ApiError::Unknow => {
                                log!("主页获取当前用户列表为空,e:",e.to_string());
                            },
                        }
                    },
                }
            });
             //加载后就不用再加载了
             is_visible.set(String::from("visible"));
        }
        || {}
    });


    let onclick_for_pay: Callback<MouseEvent> = {
        //通过点击事件,往store里面放当前要支付的ID
        Callback::from(move |event:MouseEvent| {
            event.prevent_default();
            let dispatch_for_pay_id = dispatch_for_pay.clone();
            let data_cuisine_id = event.target().unwrap().unchecked_into::<HtmlInputElement>().get_attribute("data-cuisine-id").unwrap();
            let data_cuisine_title = event.target().unwrap().unchecked_into::<HtmlInputElement>().get_attribute("data-cuisine-title").unwrap();
            log!("will go pay for ",data_cuisine_id.clone());
            dispatch_for_pay_id.reduce_mut( |store: &mut YewduxStore|{
                store.curr_cuisine_id = Some(data_cuisine_id);//curr_cuisine_detail.is_none理解为没有购买,需要id,方便下次购买
                store.curr_cuisine_title = Some(data_cuisine_title);//配合点击购买按钮,点击的弹出框需要显示标题信息.
                store.curr_cuisine_detail = None;//弹出框以curr_cuisine_detail.is_some判断已购买了,并展示详情.
            });
        })
    };


    let onclick: Callback<MouseEvent> = {
        Callback::from(move |event:MouseEvent| {
            event.prevent_default();
            let dispatch = dispatch.clone();
            let dispatch_for_detail = dispatch.clone();
            let rundom_old = dispatch.get().random.clone();
            let store_for_detail = store_for_detail.clone();
           
            //首次点击,获取随机数
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
            //已登录:,查询cuisine详情,必须是登录状态,
           if !store_for_detail.token.is_empty() && !store_for_detail.username.is_empty() {
                //token不为空,当前用户名不为空
                let data_cuisine_id = event.target().unwrap().unchecked_into::<HtmlInputElement>().get_attribute("data-cuisine-id").unwrap();
                wasm_bindgen_futures::spawn_local(async move {
                    let response = get_cuisine_detail(data_cuisine_id.parse::<i64>().unwrap()).await;
                    match response {
                        Ok(cuisine) => {
                            //log!("get detail:",cuisine.description.clone());
                            dispatch_for_detail.reduce_mut( |store: &mut YewduxStore|{
                                store.curr_cuisine_detail = Some(cuisine);
                            });
                            },
                        Err(_) => { 
                            log!("Api Error");
                            dispatch_for_detail.reduce_mut(  |store|{
                                store.curr_cuisine_detail = None;
                            });
                        },
                    }
                });
           }
            
           
            
        })
    };

    let _button_cancel: Callback<MouseEvent>= {
        Callback::from(move|event:MouseEvent|{
            event.prevent_default();
            /* dispatch_for_random.reduce_mut(  |_store|{
                  // 不要清空random,防止用户频繁点击登录>取消,
                //store.random = "".to_string();
            }); */

        })
    };

    let _fallback = html! {
        <div class="spinner-border text-primary" role="status">
            <span class="visually-hidden">{"Loading..."}</span>
        </div>
    };
   
    html!(
            <> 
              
              /*   <h1 class="m-3 text-start   ">{"Example heading "}<span class="badge bg-secondary">{"New"}</span></h1> */
                <div class=" ms-2 mt-5 badge" style="background-color: #11caf0e0;">{"View List :"}</div>
                <div class="row row-cols-4 row-cols-lg-6 g-2 g-lg-3 my-1 mx-1 text-center">
               
                   { create_list(store_create_list,onclick,onclick_for_pay) }
                </div>

                <nav aria-label="Page navigation ">
                    <ul class="pagination mt-5">
                        <li class="page-item"><a class="page-link" href="#">{"Previous"}</a></li>
                        <li class="page-item"><a class="page-link" href="#">{1}</a></li>
                        <li class="page-item"><a class="page-link" href="#">{2}</a></li>
                        <li class="page-item"><a class="page-link" href="#">{3}</a></li>
                        <li class="page-item"><a class="page-link" href="#">{"Next"}</a></li>
                    </ul>
                </nav>

                /* <Suspense {fallback}>
                    <Content />
                </Suspense> */
               /*  <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#staticBackdrop">
               {" Launch static backdrop modal"}
                </button> */

                {   //页面弹框: 生成点击后,弹框应该是登录框还是去支付的框框
                    create_view_or_login_or_purchase(store_view_or_login)
                }
               /*  <nav aria-label="breadcrumb">
                    <ol class="breadcrumb">
                    <li class="nav-item">
                        <Link<Route> to={Route::Privacy} classes={classes!("nav-link")} >{"Privacy Policy"}</Link<Route>>
                    </li>
                    </ol>
                </nav> */
            </>
    )
}

pub fn is_logged_in(token: &str) -> bool {
    log!("logined??: ", !token.is_empty());
    !token.is_empty()
}

//生成菜单列表
pub fn create_list(store_create_list: Rc<YewduxStore>, onclick: Callback<MouseEvent>, onclick_pay: Callback<MouseEvent>) ->Html{
    let cuisine_list = store_create_list.cuisine_title_list.clone();
    //用户是否登录,store存储的list不同
    if cuisine_list.len() >0 {
         //未登录,生成普通点击按钮,弹出登录框
        if store_create_list.token.is_empty(){
            log!("首页创建列表, store_create_list.token.is_empty()",store_create_list.token.is_empty());
            // 1. 如果未登录> 根据查询的列表,生成普通title和登录超链接.列表已经处理了登录和未登录情况下不同的列表值.
            cuisine_list.into_iter().map(|item|{
                let onclick = onclick.clone();
                //let onclick_pay = onclick_pay.clone();
                //有用户信息, 生成详情链接,不生产购买按钮.
                html!{
                    <div class="col text-secondary-emphasis bg-primary-subtle border border-primary-subtle rounded-3 mx-1">
                        <h6 class="m-1" data-bs-toggle="modal" data-bs-target="#staticBackdrop" data-cuisine-id={item.id.to_string()} {onclick} > { item.title} 
                        
                        </h6>
                        
                    </div>
                }
            }).collect()
        }else{
            //有token,已登录.
            // 2. 如果已登录, 根据用户是否购买(user_email是否为空 and pay_status > 0 ),来判断显示购买按钮还是详情链接
            //  pay_status 0: init , 1: payed
            log!("have token, cuisine_list size:", cuisine_list.len());
            cuisine_list.into_iter().map(|item|{
                let onclick = onclick.clone();
                let onclick_for_pay = onclick_pay.clone();
                if item.user_email.is_some() && item.pay_status.unwrap() > 0{
                    //有用户信息, 生成详情链接,不生产购买按钮. 0: init , 1: payed
                    log!("有用户信息, 生成详情链接,不生产购买按钮: for detail pay_status: ", item.pay_status);
                    html!{
                        <div class="col text-secondary-emphasis bg-primary-subtle border border-primary-subtle rounded-3 mx-1">
                            <h6 class="m-1" data-bs-toggle="modal" data-bs-target="#staticBackdrop" data-cuisine-id={item.id.to_string()} {onclick} > {item.title} 
                            
                            </h6>
                            
                        </div>
                    }
                }else{
                        //查询的列表没有关联用户信息,代表没有购买,需要生成: 购买按钮,但是不生成详情链接.
                        log!("no purched, pay_status: ", item.pay_status);
                        html!{//购买按钮
                            <div class="col text-secondary-emphasis bg-primary-subtle border border-primary-subtle rounded-3 mx-1">
                                <h6 class="m-1" > { item.title.clone()} 
                                    <span class="badge bg-secondary p-0" style="width:20%;float: right;" data-bs-toggle="modal" data-bs-target="#staticBackdrop" data-cuisine-id ={item.id.to_string()} data-cuisine-title = { item.title.clone()} onclick = {onclick_for_pay}> /* {item.id.to_string()} */
                                        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"  fill="currentColor" class="bi bi-credit-card" viewBox="0 0 16 16" style="pointer-events:none;">
                                            <path d="M0 4a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H2a2 2 0 0 1-2-2V4zm2-1a1 1 0 0 0-1 1v1h14V4a1 1 0 0 0-1-1H2zm13 4H1v5a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V7z"/>
                                            <path d="M2 10a1 1 0 0 1 1-1h1a1 1 0 0 1 1 1v1a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1v-1z"/>
                                        </svg>
                                    </span>
                                </h6>
                            
                            </div>
                        }
                }
            }).collect()
        }
        
       
    }else{
        html!{
            <div>{"Refres.."}</div>
        }
    }
   
}



//页面弹框: 生成点击后,弹框应该是登录框还是去支付的框框
pub fn create_view_or_login_or_purchase(store_view_or_login: Rc<YewduxStore>) ->Html{
    if store_view_or_login.token.is_empty(){
        //token为空,未登录,生成登录框.
        html!{
            <div class="modal fade" id="staticBackdrop"  data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                <div class="modal-dialog ">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5" id="staticBackdropLabel">{"Login"}</h1>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body text-center">
                            {  
                                create_login(store_view_or_login.random.clone())
                            }
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" /*  onclick = {button_cancel} */>{"Close"}</button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }else if !store_view_or_login.token.is_empty() && store_view_or_login.curr_cuisine_detail.is_some() {
        //已支付,生成详情页面
        html!{
            <div class="modal fade" id="staticBackdrop"  data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-scrollable">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5" id="staticBackdropLabel">{store_view_or_login.curr_cuisine_detail.clone().unwrap().title}</h1>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body text-start">
                        { create_html_detail(store_view_or_login) }
                           /*  {
                                store_view_or_login.curr_cuisine_detail.clone().unwrap().description
                            } */
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                            <div id="paypal-container-MUYVQZ85NQHAS"></div>
                           /*  <button type="button" class="btn btn-primary">{"Save changes"}</button> */
                        </div>
                    </div>
                </div>
            </div>
        }
    }else if !store_view_or_login.token.is_empty() &&
             store_view_or_login.curr_cuisine_detail.is_none() && 
             store_view_or_login.curr_cuisine_id.is_some()&& 
             store_view_or_login.curr_cuisine_title.is_some(){
        //未支付,生成支付框
        let url = get_paypal_url(store_view_or_login.curr_cuisine_id.clone().unwrap(),store_view_or_login.token.clone());
        html!{
            <div class="modal fade" id="staticBackdrop"  data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                <div class="modal-dialog modal-dialog-scrollable">
                    <div class="modal-content">
                        <div class="modal-header">
                            <h1 class="modal-title fs-5" id="staticBackdropLabel">{"支付: "}{store_view_or_login.curr_cuisine_title.clone().unwrap()}</h1>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                        </div>
                        <div class="modal-body text-center">
                            <a href={url}>
                                <img class="pr-2 img-fluid" src="static/PayPal.png" alt="" style={"height: 2.2rem"} />
                            </a>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                            <div id="paypal-container-MUYVQZ85NQHAS"></div>
                        /*  <button type="button" class="btn btn-primary">{"Save changes"}</button> */
                        </div>
                    </div>
                </div>
            </div>

        }
    }else{
        html!{
            <div>{"no page"} </div>
        }       
    }
}

use yew::Html;

pub fn create_html_detail(store_view_or_login: Rc<YewduxStore>) -> Html{
    log!("to html");
    html!{
        Html::from_html_unchecked(store_view_or_login.curr_cuisine_detail.clone().unwrap().description.into())
       // store_view_or_login.curr_cuisine_detail.clone().unwrap().description
    }
   
}