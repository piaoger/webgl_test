// https://github.com/rustwasm/wasm-bindgen/blob/master/examples/fetch/src/lib.rs
// http://yun.uzhujia.com/api/accounts/v1/accounts/eb64654a9c904612a63bf2a9197ad692

// {
//   "identityName": "飘行天下",
//   "headimgurl": "http://thirdwx.qlogo.cn/mmopen/2nmqEIXKCRJOs8AxicwYDYovXLmw7g8qf1RNh0NgLehcEMIcbkiaq5Aa2aNY4JrU1FZwSKWuK5U2Bic6sMYs2oibAs5KzZ4ql9bq/132",
//   "wechatName": "飘行天下",
//   "wechatImgUrl": "http://wx.qlogo.cn/mmopen/vi_32/28RCge3kSQLGBQuYwb8fGkibRJFRic9S9mAibVNgAPRSgw5VTFlHgJCRfpicctOTPWra0IWhC1uGrHnEAbeVhyhPlw/0",
//   "wechatOpenid": "oc2xLwdONgLkCYflwIZBK3FcAC5I",
//   "id": "eb64654a9c904612a63bf2a9197ad692",
//   "properties": {
//     "lastLogin": "2018-10-12T14:43:23.635Z"
//   },
//   "_id": "eb64654a9c904612a63bf2a9197ad692",
//   "displayname": "飘行天下"
// }

use futures::{future, Future};
use js_sys::Promise;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Request, RequestInit, RequestMode, Response};

use fetch;
use js;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    #[serde(rename = "identityName")]
    identity_name: String,
    #[serde(rename = "displayname")]
    display_name: String,
    #[serde(rename = "headimgurl")]
    avatar: String,
    #[serde(rename = "id")]
    id: String,
}

#[wasm_bindgen]
pub fn get_account_info() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let accout_id = "eb64654a9c904612a63bf2a9197ad692";
    let end_point = format!(
        "{}{}",
        "https://yun.uzhujia.com/api/accounts/v1/accounts/", accout_id
    );

    let request = Request::new_with_str_and_init(&end_point, &opts).unwrap();

    request.headers().set("Accept", "application/json").unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            //assert!(resp_value.is_instance_of::<Response>());
            js::log("reponse get .... ");
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            js::log("future from json value");
            // Convert this other `Promise` into a rust `Future`.
            JsFuture::from(json_value)
        })
        .and_then(|json| {

            js::log("get values ");

            // Use serde to parse the JSON into a struct.
            let mut account_info: AccountInfo = json.into_serde().unwrap();
            let mut avatar = account_info.avatar;

            //
            account_info.avatar = avatar.replace("http:", "https:");

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&account_info).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}

// ip info from taobao ip
// {
//   "code": 0,
//   "data": {
//     "ip": "210.75.225.254",
//     "country": "中国",
//     "area": "",
//     "region": "北京",
//     "city": "北京",
//     "county": "XX",
//     "isp": "科技网",
//     "country_id": "CN",
//     "area_id": "",
//     "region_id": "110000",
//     "city_id": "110100",
//     "county_id": "xx",
//     "isp_id": "1000114"
//   }
// }
// http://ip.taobao.com/service/getIpInfo.php?ip=210.75.225.254

#[derive(Debug, Serialize, Deserialize)]
pub struct IpInfo {
    #[serde(rename = "country")]
    country: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "city")]
    city: String,
    #[serde(rename = "ip")]
    ip: String,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct IpInfo<'a> {
//     #[serde(rename = "country")]
//      country: &'a str,
//     #[serde(rename = "region")]
//       region: &'a str,
//     #[serde(rename = "city")]
//       city: &'a str,
//     #[serde(rename = "ip")]
//       ip: &'a str,
// }

#[wasm_bindgen]
pub fn get_ip_info() -> Promise {


    // let accout_id = "210.75.225.254";
    // let end_point = format!(
    //     "{}{}",
    //     "http://ip.taobao.com/service/getIpInfo.php?ip=", accout_id
    // );

    let accout_id = "43e2a968859a4f27a641c3f3ac69b232";
    let url = format!(
        "{}{}",
        "https://yun.uzhujia.com/api/accounts/v1/accounts/", accout_id
    );



    let request_promise = fetch::Fetch::new(fetch::Method::Get, &url).send();

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            // `resp_value` is a `Response` object.
            js::log("reponse get .... ");
            //assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            let v = resp.json();

            v
        })
        .and_then(|json_value: Promise| {

            // Convert this other `Promise` into a rust `Future`.
            let f = JsFuture::from(json_value);
            f
        })
        .and_then(|json| {
            js::log("reponse get .... 3");
            // Use serde to parse the JSON into a struct.
            let ip_info: AccountInfo = json.into_serde().unwrap();

            // Send the `Branch` struct back to JS as an `Object`.
            future::ok(JsValue::from_serde(&ip_info).unwrap())
        });

    // Convert this Rust `Future` back into a JS `Promise`.
    future_to_promise(future)
}

// fetch impl

// https://github.com/AlexNav73/DotNetCore2018/blob/7c61687168a8662c7bd3175cd8e293a3f1fe098c/DotNetCore2018.WebApi/src/ui/src/fetch.rs
