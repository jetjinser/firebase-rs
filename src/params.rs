use crate::constants::{
    END_AT, EQUAL_TO, EXPORT, FORMAT, LIMIT_TO_FIRST, LIMIT_TO_LAST, ORDER_BY, SHALLOW, START_AT,
};

/// A trait for adding parameters to a URL.
pub trait Paramable
where
    Self: Sized,
{
    /// Adds a parameter to the URL.
    fn add_param<T>(&self, key: &str, value: T) -> Self
    where
        T: ToString;

    /// Sets the order of results by the given key.
    fn order_by(&self, key: &str) -> Self {
        self.add_param(ORDER_BY, key)
    }

    /// Limits the results to the first `count` items.
    fn limit_to_first(&self, count: u32) -> Self {
        self.add_param(LIMIT_TO_FIRST, count)
    }

    /// Limits the results to the last `count` items.
    fn limit_to_last(&self, count: u32) -> Self {
        self.add_param(LIMIT_TO_LAST, count)
    }

    /// Starts the results at the item with the given index.
    fn start_at(&self, index: u32) -> Self {
        self.add_param(START_AT, index)
    }

    /// Ends the results at the item with the given index.
    fn end_at(&self, index: u32) -> Self {
        self.add_param(END_AT, index)
    }

    /// Limits the results to items equal to the given value.
    fn equal_to(&self, value: u32) -> Self {
        self.add_param(EQUAL_TO, value)
    }

    /// Sets whether to include shallow items.
    fn shallow(&self, flag: bool) -> Self {
        self.add_param(SHALLOW, flag)
    }

    /// Sets the format of the returned data.
    fn format(&self) -> Self {
        self.add_param(FORMAT, EXPORT)
    }
}
