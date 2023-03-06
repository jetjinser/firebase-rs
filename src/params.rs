use crate::constants::{
    END_AT, EQUAL_TO, EXPORT, FORMAT, LIMIT_TO_FIRST, LIMIT_TO_LAST, ORDER_BY, SHALLOW, START_AT,
};

pub trait Paramable
where
    Self: Sized,
{
    fn add_param<T>(&self, key: &str, value: T) -> Self
    where
        T: ToString;

    fn order_by(&self, key: &str) -> Self {
        self.add_param(ORDER_BY, key)
    }

    fn limit_to_first(&self, count: u32) -> Self {
        self.add_param(LIMIT_TO_FIRST, count)
    }

    fn limit_to_last(&self, count: u32) -> Self {
        self.add_param(LIMIT_TO_LAST, count)
    }

    fn start_at(&self, index: u32) -> Self {
        self.add_param(START_AT, index)
    }

    fn end_at(&self, index: u32) -> Self {
        self.add_param(END_AT, index)
    }

    fn equal_to(&self, value: u32) -> Self {
        self.add_param(EQUAL_TO, value)
    }

    fn shallow(&self, flag: bool) -> Self {
        self.add_param(SHALLOW, flag)
    }

    fn format(&self) -> Self {
        self.add_param(FORMAT, EXPORT)
    }
}
