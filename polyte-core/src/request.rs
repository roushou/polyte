use std::marker::PhantomData;

/// Query parameter builder
pub trait QueryBuilder: Sized {
    /// Add a query parameter
    fn add_query(&mut self, key: String, value: String);

    /// Add a query parameter
    fn query(mut self, key: impl Into<String>, value: impl ToString) -> Self {
        self.add_query(key.into(), value.to_string());
        self
    }

    /// Add optional query parameter (only if Some)
    fn query_opt(mut self, key: impl Into<String>, value: Option<impl ToString>) -> Self {
        if let Some(v) = value {
            self.add_query(key.into(), v.to_string());
        }
        self
    }

    /// Add multiple query parameters with the same key
    fn query_many<I, V>(self, key: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: ToString,
    {
        let key = key.into();
        let mut result = self;
        for value in values {
            result.add_query(key.clone(), value.to_string());
        }
        result
    }

    /// Add multiple optional query parameters with the same key
    fn query_many_opt<I, V>(self, key: impl Into<String>, values: Option<I>) -> Self
    where
        I: IntoIterator<Item = V>,
        V: ToString,
    {
        if let Some(values) = values {
            self.query_many(key, values)
        } else {
            self
        }
    }
}

/// Type marker for deserializable responses
pub struct TypedRequest<T> {
    pub(crate) _marker: PhantomData<T>,
}

impl<T> TypedRequest<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for TypedRequest<T> {
    fn default() -> Self {
        Self::new()
    }
}
