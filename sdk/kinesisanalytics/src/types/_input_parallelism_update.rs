// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides updates to the parallelism count.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InputParallelismUpdate {
    /// <p>Number of in-application streams to create for the specified streaming source.</p>
    #[doc(hidden)]
    pub count_update: std::option::Option<i32>,
}
impl InputParallelismUpdate {
    /// <p>Number of in-application streams to create for the specified streaming source.</p>
    pub fn count_update(&self) -> std::option::Option<i32> {
        self.count_update
    }
}
impl InputParallelismUpdate {
    /// Creates a new builder-style object to manufacture [`InputParallelismUpdate`](crate::types::InputParallelismUpdate).
    pub fn builder() -> crate::types::builders::InputParallelismUpdateBuilder {
        crate::types::builders::InputParallelismUpdateBuilder::default()
    }
}

/// A builder for [`InputParallelismUpdate`](crate::types::InputParallelismUpdate).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct InputParallelismUpdateBuilder {
    pub(crate) count_update: std::option::Option<i32>,
}
impl InputParallelismUpdateBuilder {
    /// <p>Number of in-application streams to create for the specified streaming source.</p>
    pub fn count_update(mut self, input: i32) -> Self {
        self.count_update = Some(input);
        self
    }
    /// <p>Number of in-application streams to create for the specified streaming source.</p>
    pub fn set_count_update(mut self, input: std::option::Option<i32>) -> Self {
        self.count_update = input;
        self
    }
    /// Consumes the builder and constructs a [`InputParallelismUpdate`](crate::types::InputParallelismUpdate).
    pub fn build(self) -> crate::types::InputParallelismUpdate {
        crate::types::InputParallelismUpdate {
            count_update: self.count_update,
        }
    }
}
