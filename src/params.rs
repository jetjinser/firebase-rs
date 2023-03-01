use crate::constants::{
    END_AT, EQUAL_TO, EXPORT, FORMAT, LIMIT_TO_FIRST, LIMIT_TO_LAST, ORDER_BY, SHALLOW, START_AT,
};

pub trait Paramable {
    fn add_param<T>(&mut self, key: &str, value: T) -> &mut Self
    where
        T: ToString;

    fn order_by(&mut self, key: &str) -> &mut Self {
        self.add_param(ORDER_BY, key)
    }

    fn limit_to_first(&mut self, count: u32) -> &mut Self {
        self.add_param(LIMIT_TO_FIRST, count)
    }

    fn limit_to_last(&mut self, count: u32) -> &mut Self {
        self.add_param(LIMIT_TO_LAST, count)
    }

    fn start_at(&mut self, index: u32) -> &mut Self {
        self.add_param(START_AT, index)
    }

    fn end_at(&mut self, index: u32) -> &mut Self {
        self.add_param(END_AT, index)
    }

    fn equal_to(&mut self, value: u32) -> &mut Self {
        self.add_param(EQUAL_TO, value)
    }

    fn shallow(&mut self, flag: bool) -> &mut Self {
        self.add_param(SHALLOW, flag)
    }

    fn format(&mut self) -> &mut Self {
        self.add_param(FORMAT, EXPORT)
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
