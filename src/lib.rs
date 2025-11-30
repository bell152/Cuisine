use yew::prelude::*;
use stylist::yew::styled_component;
use yew_router::{BrowserRouter, Switch};
use std::rc::Rc;

pub mod organisms;
pub mod router;
pub mod pages;
pub mod store;
pub mod api;
pub mod utils;
pub mod atoms;

use crate::{
    organisms::navbar::Navbar,
    router::{switch, Route},
};

/// App theme
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalStatus {
    is_need_reload: bool,
}
impl GlobalStatus{

    pub fn  get_need_reload(&self) -> bool {
        self.is_need_reload
    }
}

impl Reducible for GlobalStatus {
    type Action = bool;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        GlobalStatus { is_need_reload: action }.into()
    }
}

pub type MessageContext = UseReducerHandle<GlobalStatus>;

#[styled_component(App)]
pub fn view() -> Html {
   /*  let ctx = use_state(|| GlobalStatus {
        is_need_reload: false,
    }); */
    let ctx = use_reducer(|| GlobalStatus {
        is_need_reload: false,
    });
   html! {
    <>
    <div style="height:100%;">
        <div class="container-fluid text-left">
            <ContextProvider<MessageContext> context={ctx}>
                <BrowserRouter>
                    <Navbar />
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<MessageContext>>
        </div> 
        
        <div class="fixed-bottom container-fluid text-center" > 
            <nav aria-label="breadcrumb">
                <ol class="breadcrumb">
                <li class="nav-item">
                    <span style="font-size: 12px">{"@2024-Z.B.E "}</span>
                    <a class="text-decoration-none" style="font-size: 12px" href="https://www.cuisinezbe.com/privacy">{" Privacy Policy -  "}</a>
                    <a href="mailto:zhengbeier2002@gmail.com" class="text-decoration-none" style="font-size: 12px">{"zhengbeier2002@gmail.com"}</a>
                </li>
                </ol>
            </nav>
        </div>
    </div>
    </>
       /*   <div class="container-fluid" >
            <h1>{ "Hello, Bootstrap with Yew!" }</h1>
            <BootstrapModal />
        </div> */
    } 
}
