use http::{Method, Response};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::{
    clients::HttpClient,
    constants::{
        END_AT, EQUAL_TO, EXPORT, FORMAT, LIMIT_TO_FIRST, LIMIT_TO_LAST, ORDER_BY, SHALLOW,
        START_AT,
    },
    queries::WithQueries,
    request::Requestable,
    types,
    utils::make_request,
    Firebase,
};

pub struct WithParams<'fb> {
    pub firebase: &'fb Firebase,
    pub uri: Url,
}

impl<'fb> WithParams<'fb> {
    fn add_param<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: ToString,
    {
        self.uri
            .query_pairs_mut()
            .append_pair(key, &value.to_string());

        self
    }

    pub fn order_by(&mut self, key: &str) -> &mut Self {
        self.add_param(ORDER_BY, key)
    }

    pub fn limit_to_first(&mut self, count: u32) -> &mut Self {
        self.add_param(LIMIT_TO_FIRST, count)
    }

    pub fn limit_to_last(&mut self, count: u32) -> &mut Self {
        self.add_param(LIMIT_TO_LAST, count)
    }

    pub fn start_at(&mut self, index: u32) -> &mut Self {
        self.add_param(START_AT, index)
    }

    pub fn end_at(&mut self, index: u32) -> &mut Self {
        self.add_param(END_AT, index)
    }

    pub fn equal_to(&mut self, value: u32) -> &mut Self {
        self.add_param(EQUAL_TO, value)
    }

    pub fn shallow(&mut self, flag: bool) -> &mut Self {
        self.add_param(SHALLOW, flag)
    }

    pub fn format(&mut self) -> &mut Self {
        self.add_param(FORMAT, EXPORT)
    }

    pub fn finish(self) -> &'fb Firebase {
        self.firebase
    }
}

impl WithParams<'_> {
    pub fn with_queries(&self) -> WithQueries {
        WithQueries {
            firebase: self.firebase,
            uri: self.uri.clone(),
        }
    }
}

impl Requestable<'_> for WithParams<'_> {
    fn request<'life0, 'async_trait, Resp>(
        &'life0 self,
        method: Method,
        data: Option<Value>,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = types::Result<Response<Resp>>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        Resp: for<'a> Deserialize<'a>,
        Resp: 'async_trait,
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async {
            let req = make_request(&self.uri, method, data);

            self.firebase.client.send(req).await
        })
    }
}

#[cfg(test)]
mod tests {
    // use crate::params::Params;
    // use std::collections::HashMap;
    // use url::Url;

    #[test]
    fn check_params() {
        // let mut params = HashMap::new();
        // params.insert("param_1", "value_1");
        // params.insert("param_2", "value_2");

        // let mut param = Params::new(Url::parse("https://github.com/emreyalvac").unwrap());

        // for (k, v) in params {
        //     param.add_param(&k, v);
        // }
        // param.set_params();

        // assert_eq!(
        //     param.uri.as_str(),
        //     "https://github.com/emreyalvac?param_1=value_1&param_2=value_2"
        // )
    }
}
