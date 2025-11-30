use std::{ops::Deref, rc::Rc};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use yewdux::{prelude::*, Context};


#[derive(Clone, Store, PartialEq, Serialize, Deserialize)]
#[store(storage = "session", storage_tab_sync, listener(StoreListener))]
pub struct YewduxStore {
    // pub count:u32,
    pub username: String,
    pub token: String,
    pub cuisine_title_list: Vec<CuisineList>,
    pub random: String,

    pub curr_cuisine_detail: Option<Cuisine>,
    pub curr_cuisine_id:Option<String>,
    pub curr_cuisine_title: Option<String>,
    // pub is_loaded: bool,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CuisineList{
    pub id: i32,
    pub title:String,
    pub user_email:Option<String>,
    pub pay_status:Option<i32>
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Cuisine {
    pub id: i32,
    pub title: String,
    pub description: String, 
}


impl YewduxStore {
    pub fn get_list_by_id(&self, task_id: i32) -> Option<&CuisineList> {
        self.cuisine_title_list.iter().find(|task| task.id == task_id)
    }
}
impl Default for YewduxStore {
    fn default() -> Self {
        Self {
            username: Default::default(),
            token: Default::default(),
            cuisine_title_list: Default::default(),
            random:Default::default(),
            curr_cuisine_detail:None,
            curr_cuisine_id:None,
            curr_cuisine_title:None,
        }
    }
}
struct StoreListener;
impl Listener for StoreListener {
    type Store = YewduxStore;

    fn on_change(&mut self, _: &Context, state: Rc<Self::Store>) {
        log!(format!( "自定义状态监听: {}",serde_json::to_string(&state.username).unwrap()));
    }
}

pub type StorageDispatch = Dispatch<YewduxStore>;

pub fn logout(dispatch: StorageDispatch) {
    dispatch.reduce(|store| {
        YewduxStore {
            username: String::new(),
            token: String::new(),
            ..store.deref().clone()
        }
        .into()
    });
}


/* 
pub fn mark_task_completed(dispatch: StorageDispatch, task_id: u32) {
    dispatch.reduce_mut(move |store| {
        let task = store.cuisine_title_list.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            console::error!("Error marking task");
            panic!();
        }
        let now = Date::new_0();
        task.unwrap().completed_at = now.to_utc_string().as_string();
    });
}

pub fn mark_task_uncompleted(dispatch: StorageDispatch, task_id: u32) {
    dispatch.reduce_mut(move |store| {
        let task = store.cuisine_title_list.iter_mut().find(|task| task.id == task_id);
        if task.is_none() {
            console::error!("Error marking task");
            panic!();
        }
        let _ = Date::new_0();
        task.unwrap().completed_at = None;
    });
}

pub fn mark_loaded(new_state: bool,dispatch:StorageDispatch){
    dispatch.reduce(move |store|{
        YewduxStore{
            is_loaded:new_state,
           ..store.deref().clone()
        }.into()
    })
} */
/* #[derive(Default, Clone, PartialEq, Eq, Store)]
struct State {
    count: u32,
    usernames: String,
    password: String,
}
 */
/* impl Default for YewduxStore{
     fn default() -> Self {
        Self { count: 5 }
    }
} */
