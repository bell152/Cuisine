use dotenv_codegen::dotenv;
use gloo::console::log;
use reqwasm::http::{self, Request};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{pages::suspense::CuisineUser, store::{Cuisine, CuisineList}};

use super::api_errors::ApiError;

#[derive(Deserialize, Serialize)]
pub struct ApiResponse {
    pub id: u32,
    pub username: String,
    pub token: String,
}
#[derive(Deserialize, Serialize)]
pub struct TxListRespose {
    pub tx_list: Vec<TxEntity>,
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TxEntity {
    pub key: String,
    pub field: String,
    pub tx_id: String,
    pub value: String,
}

#[derive(Deserialize, Serialize)]
pub struct TxOptResponse {
    pub message: String,
    pub contract_result: String,
    pub tx_id: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TxValue {
    pub name: String,
    pub age: String,
    pub des: String,
}

/* pub async fn get_tx_list(username: String, password: String, token: &str) -> TxListRespose {
    let body = json!({
        "token": username,
        "password": password,
    });
    //let req_url = env::var("API_URL").unwrap();
    // let listen_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //dotenv::dotenv().ok();
    let url = dotenv!("API_URL");
    Request::post(&format!("{}/chainmaker/getTx", url))
        .header("x-auth-token", &token)
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<TxListRespose>()
        .await
        .unwrap()
} */

/* pub async fn get_tx_by_value(param_str: String, token: &str) -> TxListRespose {
    let body = json!({
        "param_str": param_str,
    });
    //let req_url = env::var("API_URL").unwrap();
    // let listen_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //dotenv::dotenv().ok();
    let url = dotenv!("API_URL");
    Request::post(&format!("{}/chainmaker/getTxByValue", url))
        .header("x-auth-token", &token)
        .header("content-type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .unwrap()
        .json::<TxListRespose>()
        .await
        .unwrap()
} */

/* pub async fn add_tx(txvalue: TxValue, token: &str) -> TxOptResponse {
    /*  let v= TxValue {
        name: "业务的一些字段".to_owned(),
        age: "另一些字段".to_owned(),
        des: "其他描述信息".to_owned()
    };*/
    let params = serde_json::to_value(txvalue).unwrap();

    //let req_url = env::var("API_URL").unwrap();
    // let listen_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //dotenv::dotenv().ok();
    let url = dotenv!("API_URL");
    Request::post(&format!("{}/chainmaker/addTx", url))
        .header("x-auth-token", &token)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await
        .unwrap()
        .json::<TxOptResponse>()
        .await
        .unwrap()
} */
/* 
pub async fn delete_tx(txkey: &str, txfield: &str, token: &str) -> TxOptResponse {
    let params = json!({
        "txkey": txkey,
        "txfield": txfield,
    });
    // let params = serde_json::to_value(txvalue).unwrap();

    //let req_url = env::var("API_URL").unwrap();
    // let listen_addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    //dotenv::dotenv().ok();
    let url = dotenv!("API_URL");
    Request::post(&format!("{}/chainmaker/deleteTx", url))
        .header("x-auth-token", &token)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await
        .unwrap()
        .json::<TxOptResponse>()
        .await
        .unwrap()
} */


pub async fn get_user_one() -> Result<CuisineUser, reqwasm::Error> {
    let url = dotenv!("API_URL");
    let response = Request::get(&format!("{}/api/getUser", url))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
        .unwrap()
        .json::<CuisineUser>()
        .await;
    let rest = response.unwrap();
    Ok(rest)
}


pub async fn get_random() -> Result<String,ApiError> {
    let url = dotenv!("API_URL");
    let response = Request::get(&format!("{}/api/random", url))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await;
    match  response {
       Ok(response) =>{
            //log!("remote api ,get remote cookie:{}",cookie);
            let response = response.text().await.unwrap();
            Ok(response)
        },
        Err(_) => {
            Err(ApiError::NotAuthenticated)
        },
    }
   
}



pub async fn get_cuisine_list() -> Result<Vec<CuisineList>,  reqwasm::Error> {
    let url = dotenv!("API_URL");
    let response: Result<Vec<CuisineList>, reqwasm::Error> =Request::get(&format!("{}/api/list", url))
        .credentials(http::RequestCredentials::Include)
        .send()
        .await
        .unwrap()
        .json::<Vec<CuisineList>>()
        .await;
    let rest = response.unwrap();
    Ok(rest)
}

pub async fn get_curr_user_cuisine_list() -> Result<Vec<CuisineList>, ApiError> {
    let url = dotenv!("API_URL");
    let response = Request::post(&format!("{}/svc/uclist", url))
    .credentials(http::RequestCredentials::Include)
    .send()
    .await;
    match response {
        Ok(response) => {
            if response.status() == 401{
                let msg = response.text().await.unwrap();
                log!("api error: ",msg);
                Err(ApiError::NotAuthenticated)
            }else{
                let result = response.json::<Vec<CuisineList>>().await.unwrap();
                Ok(result)
            }
        },
        Err(err) => {
            log!(err.to_string());
            Err(ApiError::Unknow)
        },

    }

}

pub async fn get_cuisine_detail(title_id: i64,)->Result<Cuisine, ApiError> {
    let url = dotenv!("API_URL");
    let params = json!({
        "title_id": title_id,
    });
    let response =Request::post(&format!("{}/svc/detail", url))
        .credentials(http::RequestCredentials::Include)
        .header("content-type", "application/json")
        .body(params.to_string())
        .send()
        .await
        .unwrap()
        .json::<Cuisine>()
        .await;
    match response {
        Ok(response) =>{
           Ok(response)
        },
        Err(_) => {
            Err(ApiError::NotAuthenticated)
        },
    }
}