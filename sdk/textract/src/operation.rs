// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AnalyzeDocument`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`analyze_document`](crate::client::Client::analyze_document).
///
/// See [`crate::client::fluent_builders::AnalyzeDocument`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AnalyzeDocument {
    _private: (),
}
impl AnalyzeDocument {
    /// Creates a new builder-style object to manufacture [`AnalyzeDocumentInput`](crate::input::AnalyzeDocumentInput).
    pub fn builder() -> crate::input::analyze_document_input::Builder {
        crate::input::analyze_document_input::Builder::default()
    }
    /// Creates a new `AnalyzeDocument` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AnalyzeDocument {
    type Output = std::result::Result<
        crate::output::AnalyzeDocumentOutput,
        crate::error::AnalyzeDocumentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_analyze_document_error(response)
        } else {
            crate::operation_deser::parse_analyze_document_response(response)
        }
    }
}

/// Operation shape for `AnalyzeExpense`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`analyze_expense`](crate::client::Client::analyze_expense).
///
/// See [`crate::client::fluent_builders::AnalyzeExpense`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AnalyzeExpense {
    _private: (),
}
impl AnalyzeExpense {
    /// Creates a new builder-style object to manufacture [`AnalyzeExpenseInput`](crate::input::AnalyzeExpenseInput).
    pub fn builder() -> crate::input::analyze_expense_input::Builder {
        crate::input::analyze_expense_input::Builder::default()
    }
    /// Creates a new `AnalyzeExpense` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AnalyzeExpense {
    type Output =
        std::result::Result<crate::output::AnalyzeExpenseOutput, crate::error::AnalyzeExpenseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_analyze_expense_error(response)
        } else {
            crate::operation_deser::parse_analyze_expense_response(response)
        }
    }
}

/// Operation shape for `AnalyzeID`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`analyze_id`](crate::client::Client::analyze_id).
///
/// See [`crate::client::fluent_builders::AnalyzeID`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AnalyzeID {
    _private: (),
}
impl AnalyzeID {
    /// Creates a new builder-style object to manufacture [`AnalyzeIdInput`](crate::input::AnalyzeIdInput).
    pub fn builder() -> crate::input::analyze_id_input::Builder {
        crate::input::analyze_id_input::Builder::default()
    }
    /// Creates a new `AnalyzeID` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AnalyzeID {
    type Output = std::result::Result<crate::output::AnalyzeIdOutput, crate::error::AnalyzeIDError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_analyze_id_error(response)
        } else {
            crate::operation_deser::parse_analyze_id_response(response)
        }
    }
}

/// Operation shape for `DetectDocumentText`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`detect_document_text`](crate::client::Client::detect_document_text).
///
/// See [`crate::client::fluent_builders::DetectDocumentText`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DetectDocumentText {
    _private: (),
}
impl DetectDocumentText {
    /// Creates a new builder-style object to manufacture [`DetectDocumentTextInput`](crate::input::DetectDocumentTextInput).
    pub fn builder() -> crate::input::detect_document_text_input::Builder {
        crate::input::detect_document_text_input::Builder::default()
    }
    /// Creates a new `DetectDocumentText` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DetectDocumentText {
    type Output = std::result::Result<
        crate::output::DetectDocumentTextOutput,
        crate::error::DetectDocumentTextError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_detect_document_text_error(response)
        } else {
            crate::operation_deser::parse_detect_document_text_response(response)
        }
    }
}

/// Operation shape for `GetDocumentAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_document_analysis`](crate::client::Client::get_document_analysis).
///
/// See [`crate::client::fluent_builders::GetDocumentAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDocumentAnalysis {
    _private: (),
}
impl GetDocumentAnalysis {
    /// Creates a new builder-style object to manufacture [`GetDocumentAnalysisInput`](crate::input::GetDocumentAnalysisInput).
    pub fn builder() -> crate::input::get_document_analysis_input::Builder {
        crate::input::get_document_analysis_input::Builder::default()
    }
    /// Creates a new `GetDocumentAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDocumentAnalysis {
    type Output = std::result::Result<
        crate::output::GetDocumentAnalysisOutput,
        crate::error::GetDocumentAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_document_analysis_error(response)
        } else {
            crate::operation_deser::parse_get_document_analysis_response(response)
        }
    }
}

/// Operation shape for `GetDocumentTextDetection`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_document_text_detection`](crate::client::Client::get_document_text_detection).
///
/// See [`crate::client::fluent_builders::GetDocumentTextDetection`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDocumentTextDetection {
    _private: (),
}
impl GetDocumentTextDetection {
    /// Creates a new builder-style object to manufacture [`GetDocumentTextDetectionInput`](crate::input::GetDocumentTextDetectionInput).
    pub fn builder() -> crate::input::get_document_text_detection_input::Builder {
        crate::input::get_document_text_detection_input::Builder::default()
    }
    /// Creates a new `GetDocumentTextDetection` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDocumentTextDetection {
    type Output = std::result::Result<
        crate::output::GetDocumentTextDetectionOutput,
        crate::error::GetDocumentTextDetectionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_document_text_detection_error(response)
        } else {
            crate::operation_deser::parse_get_document_text_detection_response(response)
        }
    }
}

/// Operation shape for `GetExpenseAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_expense_analysis`](crate::client::Client::get_expense_analysis).
///
/// See [`crate::client::fluent_builders::GetExpenseAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetExpenseAnalysis {
    _private: (),
}
impl GetExpenseAnalysis {
    /// Creates a new builder-style object to manufacture [`GetExpenseAnalysisInput`](crate::input::GetExpenseAnalysisInput).
    pub fn builder() -> crate::input::get_expense_analysis_input::Builder {
        crate::input::get_expense_analysis_input::Builder::default()
    }
    /// Creates a new `GetExpenseAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetExpenseAnalysis {
    type Output = std::result::Result<
        crate::output::GetExpenseAnalysisOutput,
        crate::error::GetExpenseAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_expense_analysis_error(response)
        } else {
            crate::operation_deser::parse_get_expense_analysis_response(response)
        }
    }
}

/// Operation shape for `StartDocumentAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_document_analysis`](crate::client::Client::start_document_analysis).
///
/// See [`crate::client::fluent_builders::StartDocumentAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDocumentAnalysis {
    _private: (),
}
impl StartDocumentAnalysis {
    /// Creates a new builder-style object to manufacture [`StartDocumentAnalysisInput`](crate::input::StartDocumentAnalysisInput).
    pub fn builder() -> crate::input::start_document_analysis_input::Builder {
        crate::input::start_document_analysis_input::Builder::default()
    }
    /// Creates a new `StartDocumentAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDocumentAnalysis {
    type Output = std::result::Result<
        crate::output::StartDocumentAnalysisOutput,
        crate::error::StartDocumentAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_document_analysis_error(response)
        } else {
            crate::operation_deser::parse_start_document_analysis_response(response)
        }
    }
}

/// Operation shape for `StartDocumentTextDetection`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_document_text_detection`](crate::client::Client::start_document_text_detection).
///
/// See [`crate::client::fluent_builders::StartDocumentTextDetection`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartDocumentTextDetection {
    _private: (),
}
impl StartDocumentTextDetection {
    /// Creates a new builder-style object to manufacture [`StartDocumentTextDetectionInput`](crate::input::StartDocumentTextDetectionInput).
    pub fn builder() -> crate::input::start_document_text_detection_input::Builder {
        crate::input::start_document_text_detection_input::Builder::default()
    }
    /// Creates a new `StartDocumentTextDetection` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartDocumentTextDetection {
    type Output = std::result::Result<
        crate::output::StartDocumentTextDetectionOutput,
        crate::error::StartDocumentTextDetectionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_document_text_detection_error(response)
        } else {
            crate::operation_deser::parse_start_document_text_detection_response(response)
        }
    }
}

/// Operation shape for `StartExpenseAnalysis`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_expense_analysis`](crate::client::Client::start_expense_analysis).
///
/// See [`crate::client::fluent_builders::StartExpenseAnalysis`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartExpenseAnalysis {
    _private: (),
}
impl StartExpenseAnalysis {
    /// Creates a new builder-style object to manufacture [`StartExpenseAnalysisInput`](crate::input::StartExpenseAnalysisInput).
    pub fn builder() -> crate::input::start_expense_analysis_input::Builder {
        crate::input::start_expense_analysis_input::Builder::default()
    }
    /// Creates a new `StartExpenseAnalysis` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartExpenseAnalysis {
    type Output = std::result::Result<
        crate::output::StartExpenseAnalysisOutput,
        crate::error::StartExpenseAnalysisError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_expense_analysis_error(response)
        } else {
            crate::operation_deser::parse_start_expense_analysis_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
