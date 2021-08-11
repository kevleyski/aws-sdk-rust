// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Deletes an object at the specified path.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteObject {
    _private: (),
}
impl DeleteObject {
    /// Creates a new builder-style object to manufacture [`DeleteObjectInput`](crate::input::DeleteObjectInput)
    pub fn builder() -> crate::input::delete_object_input::Builder {
        crate::input::delete_object_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteObject {
    type Output =
        std::result::Result<crate::output::DeleteObjectOutput, crate::error::DeleteObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_object_error(response)
        } else {
            crate::operation_deser::parse_delete_object_response(response)
        }
    }
}

/// <p>Gets the headers for an object at the specified path.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeObject {
    _private: (),
}
impl DescribeObject {
    /// Creates a new builder-style object to manufacture [`DescribeObjectInput`](crate::input::DescribeObjectInput)
    pub fn builder() -> crate::input::describe_object_input::Builder {
        crate::input::describe_object_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DescribeObject {
    type Output =
        std::result::Result<crate::output::DescribeObjectOutput, crate::error::DescribeObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_object_error(response)
        } else {
            crate::operation_deser::parse_describe_object_response(response)
        }
    }
}

/// <p>Downloads the object at the specified path. If the object’s upload availability is set to <code>streaming</code>, AWS Elemental MediaStore downloads the object even if it’s still uploading the object.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetObject {
    _private: (),
}
impl GetObject {
    /// Creates a new builder-style object to manufacture [`GetObjectInput`](crate::input::GetObjectInput)
    pub fn builder() -> crate::input::get_object_input::Builder {
        crate::input::get_object_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseHttpResponse for GetObject {
    type Output = std::result::Result<crate::output::GetObjectOutput, crate::error::GetObjectError>;
    fn parse_unloaded(
        &self,
        response: &mut smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_object(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_object_error(response)
    }
}

/// <p>Provides a list of metadata entries about folders and objects in the specified
/// folder.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListItems {
    _private: (),
}
impl ListItems {
    /// Creates a new builder-style object to manufacture [`ListItemsInput`](crate::input::ListItemsInput)
    pub fn builder() -> crate::input::list_items_input::Builder {
        crate::input::list_items_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListItems {
    type Output = std::result::Result<crate::output::ListItemsOutput, crate::error::ListItemsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_items_error(response)
        } else {
            crate::operation_deser::parse_list_items_response(response)
        }
    }
}

/// <p>Uploads an object to the specified path. Object sizes are limited to 25 MB for standard upload availability and 10 MB for streaming upload availability.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutObject {
    _private: (),
}
impl PutObject {
    /// Creates a new builder-style object to manufacture [`PutObjectInput`](crate::input::PutObjectInput)
    pub fn builder() -> crate::input::put_object_input::Builder {
        crate::input::put_object_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PutObject {
    type Output = std::result::Result<crate::output::PutObjectOutput, crate::error::PutObjectError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_object_error(response)
        } else {
            crate::operation_deser::parse_put_object_response(response)
        }
    }
}
