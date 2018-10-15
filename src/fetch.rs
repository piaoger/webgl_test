

// Borrowed from
// https://github.com/AlexNav73/DotNetCore2018/blob/master/DotNetCore2018.WebApi/src/ui/src/fetch.rs

use js_sys::Promise;
use web_sys::{Request, RequestInit, RequestMode};
use serde::ser::Serialize;
use serde_urlencoded::to_string;

use std::fmt;
use std::borrow::Cow;

impl AsRef<str> for Method {
    fn as_ref(&self) -> &'static str {
        match self {
        	Method::Head => "HEAD",
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Delete => "DELETE",
        }
    }
}

pub enum Method {
	Head,
    Get,
    Post,
    Delete,
}

pub struct Fetch<'a> {
    uri: Cow<'a, str>,
    method: Method,
}

impl<'a> Fetch<'a> {
    pub fn new<U>(method: Method, uri: U) -> Self
        where U: Into<Cow<'a, str>>
    {
        Self { method, uri: uri.into() }
    }

    pub fn with_query_params<T: Serialize>(mut self, params: T) -> Self {
        self.uri = format!("{}?{}", self.uri, to_string(params).expect("invalid url params")).into();
        self
    }

     pub fn with_param<T: fmt::Display>(mut self, param: T) -> Self {
        self.uri = format!("{}/{}", self.uri, param).into();
        self
	}

	//
    // request.headers().set("Accept", "application/json").unwrap();
    pub fn with_headers<T: Serialize>(mut self, params: T) -> Self {
        panic!("not implemented");
    }

	//
    // request.headers().set("Accept", "application/json").unwrap();
    pub fn with_header<T: Serialize>(mut self, key: T, val: T) -> Self {
        panic!("not implemented");
    }

    pub fn send(self) -> Promise {
        let mut opts = RequestInit::new();
        opts.method(self.method.as_ref());
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&self.uri, &opts).unwrap();

        let window = web_sys::window().unwrap();
        let request_promise = window.fetch_with_request(&request);

        request_promise
    }
}

