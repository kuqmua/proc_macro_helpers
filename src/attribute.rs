#[derive(
    Debug,
    strum_macros::Display,
    PartialEq,
    Eq,
    Clone,
    Hash
)]
pub enum Attribute {
    Tvfrr100Continue,
    Tvfrr101SwitchingProtocols,
    Tvfrr102Processing,
    Tvfrr200Ok,
    Tvfrr201Created,
    Tvfrr202Accepted,
    Tvfrr203NonAuthoritativeInformation,
    Tvfrr204NoContent,
    Tvfrr205ResetContent,
    Tvfrr206PartialContent,
    Tvfrr207MultiStatus,
    Tvfrr208AlreadyReported,
    Tvfrr226ImUsed,
    Tvfrr300MultipleChoices,
    Tvfrr301MovedPermanently,
    Tvfrr302Found,
    Tvfrr303SeeOther,
    Tvfrr304NotModified,
    Tvfrr305UseProxy,
    Tvfrr307TemporaryRedirect,
    Tvfrr308PermanentRedirect,
    Tvfrr400BadRequest,
    Tvfrr401Unauthorized,
    Tvfrr402PaymentRequired,
    Tvfrr403Forbidden,
    Tvfrr404NotFound,
    Tvfrr405MethodNotAllowed,
    Tvfrr406NotAcceptable,
    Tvfrr407ProxyAuthenticationRequired,
    Tvfrr408RequestTimeout,
    Tvfrr409Conflict,
    Tvfrr410Gone,
    Tvfrr411LengthRequired,
    Tvfrr412PreconditionFailed,
    Tvfrr413PayloadTooLarge,
    Tvfrr414UriTooLong,
    Tvfrr415UnsupportedMediaType,
    Tvfrr416RangeNotSatisfiable,
    Tvfrr417ExpectationFailed,
    Tvfrr418ImATeapot,
    Tvfrr421MisdirectedRequest,
    Tvfrr422UnprocessableEntity,
    Tvfrr423Locked,
    Tvfrr424FailedDependency,
    Tvfrr426UpgradeRequired,
    Tvfrr428PreconditionRequired,
    Tvfrr429TooManyRequests,
    Tvfrr431RequestHeaderFieldsTooLarge,
    Tvfrr451UnavailableForLegalReasons,
    Tvfrr500InternalServerError,
    Tvfrr501NotImplemented,
    Tvfrr502BadGateway,
    Tvfrr503ServiceUnavailable,
    Tvfrr504GatewayTimeout,
    Tvfrr505HttpVersionNotSupported,
    Tvfrr506VariantAlsoNegotiates,
    Tvfrr507InsufficientStorage,
    Tvfrr508LoopDetected,
    Tvfrr510NotExtended,
    Tvfrr511NetworkAuthenticationRequired,
}

impl Attribute {
    pub fn to_axum_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100Continue => quote::quote! {axum::http::StatusCode::CONTINUE},
            Attribute::Tvfrr101SwitchingProtocols => {
                quote::quote! {axum::http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Attribute::Tvfrr102Processing => quote::quote! {axum::http::StatusCode::PROCESSING},
            Attribute::Tvfrr200Ok => quote::quote! {axum::http::StatusCode::OK},
            Attribute::Tvfrr201Created => quote::quote! {axum::http::StatusCode::CREATED},
            Attribute::Tvfrr202Accepted => quote::quote! {axum::http::StatusCode::ACCEPTED},
            Attribute::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {axum::http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Attribute::Tvfrr204NoContent => quote::quote! {axum::http::StatusCode::NO_CONTENT},
            Attribute::Tvfrr205ResetContent => quote::quote! {axum::http::StatusCode::RESET_CONTENT},
            Attribute::Tvfrr206PartialContent => {
                quote::quote! {axum::http::StatusCode::PARTIAL_CONTENT}
            }
            Attribute::Tvfrr207MultiStatus => quote::quote! {axum::http::StatusCode::MULTI_STATUS},
            Attribute::Tvfrr208AlreadyReported => {
                quote::quote! {axum::http::StatusCode::ALREADY_REPORTED}
            }
            Attribute::Tvfrr226ImUsed => quote::quote! {axum::http::StatusCode::IM_USED},
            Attribute::Tvfrr300MultipleChoices => {
                quote::quote! {axum::http::StatusCode::MULTIPLE_CHOICES}
            }
            Attribute::Tvfrr301MovedPermanently => {
                quote::quote! {axum::http::StatusCode::MOVED_PERMANENTLY}
            }
            Attribute::Tvfrr302Found => quote::quote! {axum::http::StatusCode::FOUND},
            Attribute::Tvfrr303SeeOther => quote::quote! {axum::http::StatusCode::SEE_OTHER},
            Attribute::Tvfrr304NotModified => quote::quote! {axum::http::StatusCode::NOT_MODIFIED},
            Attribute::Tvfrr305UseProxy => quote::quote! {axum::http::StatusCode::USE_PROXY},
            Attribute::Tvfrr307TemporaryRedirect => {
                quote::quote! {axum::http::StatusCode::TEMPORARY_REDIRECT}
            }
            Attribute::Tvfrr308PermanentRedirect => {
                quote::quote! {axum::http::StatusCode::PERMANENT_REDIRECT}
            }
            Attribute::Tvfrr400BadRequest => quote::quote! {axum::http::StatusCode::BAD_REQUEST},
            Attribute::Tvfrr401Unauthorized => quote::quote! {axum::http::StatusCode::UNAUTHORIZED},
            Attribute::Tvfrr402PaymentRequired => {
                quote::quote! {axum::http::StatusCode::PAYMENT_REQUIRED}
            }
            Attribute::Tvfrr403Forbidden => quote::quote! {axum::http::StatusCode::FORBIDDEN},
            Attribute::Tvfrr404NotFound => quote::quote! {axum::http::StatusCode::NOT_FOUND},
            Attribute::Tvfrr405MethodNotAllowed => {
                quote::quote! {axum::http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Attribute::Tvfrr406NotAcceptable => quote::quote! {axum::http::StatusCode::NOT_ACCEPTABLE},
            Attribute::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Attribute::Tvfrr408RequestTimeout => {
                quote::quote! {axum::http::StatusCode::REQUEST_TIMEOUT}
            }
            Attribute::Tvfrr409Conflict => quote::quote! {axum::http::StatusCode::CONFLICT},
            Attribute::Tvfrr410Gone => quote::quote! {axum::http::StatusCode::GONE},
            Attribute::Tvfrr411LengthRequired => {
                quote::quote! {axum::http::StatusCode::LENGTH_REQUIRED}
            }
            Attribute::Tvfrr412PreconditionFailed => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_FAILED}
            }
            Attribute::Tvfrr413PayloadTooLarge => {
                quote::quote! {axum::http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Attribute::Tvfrr414UriTooLong => quote::quote! {axum::http::StatusCode::URI_TOO_LONG},
            Attribute::Tvfrr415UnsupportedMediaType => {
                quote::quote! {axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Attribute::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {axum::http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Attribute::Tvfrr417ExpectationFailed => {
                quote::quote! {axum::http::StatusCode::EXPECTATION_FAILED}
            }
            Attribute::Tvfrr418ImATeapot => quote::quote! {axum::http::StatusCode::IM_A_TEAPOT},
            Attribute::Tvfrr421MisdirectedRequest => {
                quote::quote! {axum::http::StatusCode::MISDIRECTED_REQUEST}
            }
            Attribute::Tvfrr422UnprocessableEntity => {
                quote::quote! {axum::http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Attribute::Tvfrr423Locked => quote::quote! {axum::http::StatusCode::LOCKED},
            Attribute::Tvfrr424FailedDependency => {
                quote::quote! {axum::http::StatusCode::FAILED_DEPENDENCY}
            }
            Attribute::Tvfrr426UpgradeRequired => {
                quote::quote! {axum::http::StatusCode::UPGRADE_REQUIRED}
            }
            Attribute::Tvfrr428PreconditionRequired => {
                quote::quote! {axum::http::StatusCode::PRECONDITION_REQUIRED}
            }
            Attribute::Tvfrr429TooManyRequests => {
                quote::quote! {axum::http::StatusCode::TOO_MANY_REQUESTS}
            }
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {axum::http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Attribute::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {axum::http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Attribute::Tvfrr500InternalServerError => {
                quote::quote! {axum::http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Attribute::Tvfrr501NotImplemented => {
                quote::quote! {axum::http::StatusCode::NOT_IMPLEMENTED}
            }
            Attribute::Tvfrr502BadGateway => quote::quote! {axum::http::StatusCode::BAD_GATEWAY},
            Attribute::Tvfrr503ServiceUnavailable => {
                quote::quote! {axum::http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Attribute::Tvfrr504GatewayTimeout => {
                quote::quote! {axum::http::StatusCode::GATEWAY_TIMEOUT}
            }
            Attribute::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {axum::http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Attribute::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {axum::http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Attribute::Tvfrr507InsufficientStorage => {
                quote::quote! {axum::http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Attribute::Tvfrr508LoopDetected => quote::quote! {axum::http::StatusCode::LOOP_DETECTED},
            Attribute::Tvfrr510NotExtended => quote::quote! {axum::http::StatusCode::NOT_EXTENDED},
            Attribute::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {axum::http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_http_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100Continue => quote::quote! {http::StatusCode::CONTINUE},
            Attribute::Tvfrr101SwitchingProtocols => {
                quote::quote! {http::StatusCode::SWITCHING_PROTOCOLS}
            }
            Attribute::Tvfrr102Processing => quote::quote! {http::StatusCode::PROCESSING},
            Attribute::Tvfrr200Ok => quote::quote! {http::StatusCode::OK},
            Attribute::Tvfrr201Created => quote::quote! {http::StatusCode::CREATED},
            Attribute::Tvfrr202Accepted => quote::quote! {http::StatusCode::ACCEPTED},
            Attribute::Tvfrr203NonAuthoritativeInformation => {
                quote::quote! {http::StatusCode::NON_AUTHORITATIVE_INFORMATION}
            }
            Attribute::Tvfrr204NoContent => quote::quote! {http::StatusCode::NO_CONTENT},
            Attribute::Tvfrr205ResetContent => quote::quote! {http::StatusCode::RESET_CONTENT},
            Attribute::Tvfrr206PartialContent => {
                quote::quote! {http::StatusCode::PARTIAL_CONTENT}
            }
            Attribute::Tvfrr207MultiStatus => quote::quote! {http::StatusCode::MULTI_STATUS},
            Attribute::Tvfrr208AlreadyReported => {
                quote::quote! {http::StatusCode::ALREADY_REPORTED}
            }
            Attribute::Tvfrr226ImUsed => quote::quote! {http::StatusCode::IM_USED},
            Attribute::Tvfrr300MultipleChoices => {
                quote::quote! {http::StatusCode::MULTIPLE_CHOICES}
            }
            Attribute::Tvfrr301MovedPermanently => {
                quote::quote! {http::StatusCode::MOVED_PERMANENTLY}
            }
            Attribute::Tvfrr302Found => quote::quote! {http::StatusCode::FOUND},
            Attribute::Tvfrr303SeeOther => quote::quote! {http::StatusCode::SEE_OTHER},
            Attribute::Tvfrr304NotModified => quote::quote! {http::StatusCode::NOT_MODIFIED},
            Attribute::Tvfrr305UseProxy => quote::quote! {http::StatusCode::USE_PROXY},
            Attribute::Tvfrr307TemporaryRedirect => {
                quote::quote! {http::StatusCode::TEMPORARY_REDIRECT}
            }
            Attribute::Tvfrr308PermanentRedirect => {
                quote::quote! {http::StatusCode::PERMANENT_REDIRECT}
            }
            Attribute::Tvfrr400BadRequest => quote::quote! {http::StatusCode::BAD_REQUEST},
            Attribute::Tvfrr401Unauthorized => quote::quote! {http::StatusCode::UNAUTHORIZED},
            Attribute::Tvfrr402PaymentRequired => {
                quote::quote! {http::StatusCode::PAYMENT_REQUIRED}
            }
            Attribute::Tvfrr403Forbidden => quote::quote! {http::StatusCode::FORBIDDEN},
            Attribute::Tvfrr404NotFound => quote::quote! {http::StatusCode::NOT_FOUND},
            Attribute::Tvfrr405MethodNotAllowed => {
                quote::quote! {http::StatusCode::METHOD_NOT_ALLOWED}
            }
            Attribute::Tvfrr406NotAcceptable => quote::quote! {http::StatusCode::NOT_ACCEPTABLE},
            Attribute::Tvfrr407ProxyAuthenticationRequired => {
                quote::quote! {http::StatusCode::PROXY_AUTHENTICATION_REQUIRED}
            }
            Attribute::Tvfrr408RequestTimeout => {
                quote::quote! {http::StatusCode::REQUEST_TIMEOUT}
            }
            Attribute::Tvfrr409Conflict => quote::quote! {http::StatusCode::CONFLICT},
            Attribute::Tvfrr410Gone => quote::quote! {http::StatusCode::GONE},
            Attribute::Tvfrr411LengthRequired => {
                quote::quote! {http::StatusCode::LENGTH_REQUIRED}
            }
            Attribute::Tvfrr412PreconditionFailed => {
                quote::quote! {http::StatusCode::PRECONDITION_FAILED}
            }
            Attribute::Tvfrr413PayloadTooLarge => {
                quote::quote! {http::StatusCode::PAYLOAD_TOO_LARGE}
            }
            Attribute::Tvfrr414UriTooLong => quote::quote! {http::StatusCode::URI_TOO_LONG},
            Attribute::Tvfrr415UnsupportedMediaType => {
                quote::quote! {http::StatusCode::UNSUPPORTED_MEDIA_TYPE}
            }
            Attribute::Tvfrr416RangeNotSatisfiable => {
                quote::quote! {http::StatusCode::RANGE_NOT_SATISFIABLE}
            }
            Attribute::Tvfrr417ExpectationFailed => {
                quote::quote! {http::StatusCode::EXPECTATION_FAILED}
            }
            Attribute::Tvfrr418ImATeapot => quote::quote! {http::StatusCode::IM_A_TEAPOT},
            Attribute::Tvfrr421MisdirectedRequest => {
                quote::quote! {http::StatusCode::MISDIRECTED_REQUEST}
            }
            Attribute::Tvfrr422UnprocessableEntity => {
                quote::quote! {http::StatusCode::UNPROCESSABLE_ENTITY}
            }
            Attribute::Tvfrr423Locked => quote::quote! {http::StatusCode::LOCKED},
            Attribute::Tvfrr424FailedDependency => {
                quote::quote! {http::StatusCode::FAILED_DEPENDENCY}
            }
            Attribute::Tvfrr426UpgradeRequired => {
                quote::quote! {http::StatusCode::UPGRADE_REQUIRED}
            }
            Attribute::Tvfrr428PreconditionRequired => {
                quote::quote! {http::StatusCode::PRECONDITION_REQUIRED}
            }
            Attribute::Tvfrr429TooManyRequests => {
                quote::quote! {http::StatusCode::TOO_MANY_REQUESTS}
            }
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => {
                quote::quote! {http::StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE}
            }
            Attribute::Tvfrr451UnavailableForLegalReasons => {
                quote::quote! {http::StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS}
            }
            Attribute::Tvfrr500InternalServerError => {
                quote::quote! {http::StatusCode::INTERNAL_SERVER_ERROR}
            }
            Attribute::Tvfrr501NotImplemented => {
                quote::quote! {http::StatusCode::NOT_IMPLEMENTED}
            }
            Attribute::Tvfrr502BadGateway => quote::quote! {http::StatusCode::BAD_GATEWAY},
            Attribute::Tvfrr503ServiceUnavailable => {
                quote::quote! {http::StatusCode::SERVICE_UNAVAILABLE}
            }
            Attribute::Tvfrr504GatewayTimeout => {
                quote::quote! {http::StatusCode::GATEWAY_TIMEOUT}
            }
            Attribute::Tvfrr505HttpVersionNotSupported => {
                quote::quote! {http::StatusCode::HTTP_VERSION_NOT_SUPPORTED}
            }
            Attribute::Tvfrr506VariantAlsoNegotiates => {
                quote::quote! {http::StatusCode::VARIANT_ALSO_NEGOTIATES}
            }
            Attribute::Tvfrr507InsufficientStorage => {
                quote::quote! {http::StatusCode::INSUFFICIENT_STORAGE}
            }
            Attribute::Tvfrr508LoopDetected => quote::quote! {http::StatusCode::LOOP_DETECTED},
            Attribute::Tvfrr510NotExtended => quote::quote! {http::StatusCode::NOT_EXTENDED},
            Attribute::Tvfrr511NetworkAuthenticationRequired => {
                quote::quote! {http::StatusCode::NETWORK_AUTHENTICATION_REQUIRED}
            }
        }
    }
    pub fn to_status_code_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100Continue => quote::quote! {100},
            Attribute::Tvfrr101SwitchingProtocols => quote::quote! {101},
            Attribute::Tvfrr102Processing => quote::quote! {102},
            Attribute::Tvfrr200Ok => quote::quote! {200},
            Attribute::Tvfrr201Created => quote::quote! {201},
            Attribute::Tvfrr202Accepted => quote::quote! {202},
            Attribute::Tvfrr203NonAuthoritativeInformation => quote::quote! {203},
            Attribute::Tvfrr204NoContent => quote::quote! {204},
            Attribute::Tvfrr205ResetContent => quote::quote! {205},
            Attribute::Tvfrr206PartialContent => quote::quote! {206},
            Attribute::Tvfrr207MultiStatus => quote::quote! {207},
            Attribute::Tvfrr208AlreadyReported => quote::quote! {208},
            Attribute::Tvfrr226ImUsed => quote::quote! {226},
            Attribute::Tvfrr300MultipleChoices => quote::quote! {300},
            Attribute::Tvfrr301MovedPermanently => quote::quote! {301},
            Attribute::Tvfrr302Found => quote::quote! {302},
            Attribute::Tvfrr303SeeOther => quote::quote! {303},
            Attribute::Tvfrr304NotModified => quote::quote! {304},
            Attribute::Tvfrr305UseProxy => quote::quote! {305},
            Attribute::Tvfrr307TemporaryRedirect => quote::quote! {307},
            Attribute::Tvfrr308PermanentRedirect => quote::quote! {308},
            Attribute::Tvfrr400BadRequest => quote::quote! {400},
            Attribute::Tvfrr401Unauthorized => quote::quote! {401},
            Attribute::Tvfrr402PaymentRequired => quote::quote! {402},
            Attribute::Tvfrr403Forbidden => quote::quote! {403},
            Attribute::Tvfrr404NotFound => quote::quote! {404},
            Attribute::Tvfrr405MethodNotAllowed => quote::quote! {405},
            Attribute::Tvfrr406NotAcceptable => quote::quote! {406},
            Attribute::Tvfrr407ProxyAuthenticationRequired => quote::quote! {407},
            Attribute::Tvfrr408RequestTimeout => quote::quote! {408},
            Attribute::Tvfrr409Conflict => quote::quote! {409},
            Attribute::Tvfrr410Gone => quote::quote! {410},
            Attribute::Tvfrr411LengthRequired => quote::quote! {411},
            Attribute::Tvfrr412PreconditionFailed => quote::quote! {412},
            Attribute::Tvfrr413PayloadTooLarge => quote::quote! {413},
            Attribute::Tvfrr414UriTooLong => quote::quote! {414},
            Attribute::Tvfrr415UnsupportedMediaType => quote::quote! {415},
            Attribute::Tvfrr416RangeNotSatisfiable => quote::quote! {416},
            Attribute::Tvfrr417ExpectationFailed => quote::quote! {417},
            Attribute::Tvfrr418ImATeapot => quote::quote! {418},
            Attribute::Tvfrr421MisdirectedRequest => quote::quote! {421},
            Attribute::Tvfrr422UnprocessableEntity => quote::quote! {422},
            Attribute::Tvfrr423Locked => quote::quote! {423},
            Attribute::Tvfrr424FailedDependency => quote::quote! {424},
            Attribute::Tvfrr426UpgradeRequired => quote::quote! {426},
            Attribute::Tvfrr428PreconditionRequired => quote::quote! {428},
            Attribute::Tvfrr429TooManyRequests => quote::quote! {429},
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => quote::quote! {431},
            Attribute::Tvfrr451UnavailableForLegalReasons => quote::quote! {451},
            Attribute::Tvfrr500InternalServerError => quote::quote! {500},
            Attribute::Tvfrr501NotImplemented => quote::quote! {501},
            Attribute::Tvfrr502BadGateway => quote::quote! {502},
            Attribute::Tvfrr503ServiceUnavailable => quote::quote! {503},
            Attribute::Tvfrr504GatewayTimeout => quote::quote! {504},
            Attribute::Tvfrr505HttpVersionNotSupported => quote::quote! {505},
            Attribute::Tvfrr506VariantAlsoNegotiates => quote::quote! {506},
            Attribute::Tvfrr507InsufficientStorage => quote::quote! {507},
            Attribute::Tvfrr508LoopDetected => quote::quote! {508},
            Attribute::Tvfrr510NotExtended => quote::quote! {510},
            Attribute::Tvfrr511NetworkAuthenticationRequired => quote::quote! {511},
        }
    }
    pub fn to_status_code_description_token_stream(&self) -> proc_macro2::TokenStream {
        match self {
            Attribute::Tvfrr100Continue => quote::quote! {"continue"},
            Attribute::Tvfrr101SwitchingProtocols => quote::quote! {"switching protocols"},
            Attribute::Tvfrr102Processing => quote::quote! {"processing"},
            Attribute::Tvfrr200Ok => quote::quote! {"ok"},
            Attribute::Tvfrr201Created => quote::quote! {"created"},
            Attribute::Tvfrr202Accepted => quote::quote! {"accepted"},
            Attribute::Tvfrr203NonAuthoritativeInformation => quote::quote! {"non authoritative information"},
            Attribute::Tvfrr204NoContent => quote::quote! {"no content"},
            Attribute::Tvfrr205ResetContent => quote::quote! {"reset content"},
            Attribute::Tvfrr206PartialContent => quote::quote! {"partial content"},
            Attribute::Tvfrr207MultiStatus => quote::quote! {"multi status"},
            Attribute::Tvfrr208AlreadyReported => quote::quote! {"already reported"},
            Attribute::Tvfrr226ImUsed => quote::quote! {"im used"},
            Attribute::Tvfrr300MultipleChoices => quote::quote! {"multiple choices"},
            Attribute::Tvfrr301MovedPermanently => quote::quote! {"moved permanently"},
            Attribute::Tvfrr302Found => quote::quote! {"found"},
            Attribute::Tvfrr303SeeOther => quote::quote! {"see other"},
            Attribute::Tvfrr304NotModified => quote::quote! {"not modified"},
            Attribute::Tvfrr305UseProxy => quote::quote! {"use proxy"},
            Attribute::Tvfrr307TemporaryRedirect => quote::quote! {"temporary redirect"},
            Attribute::Tvfrr308PermanentRedirect => quote::quote! {"permanent redirect"},
            Attribute::Tvfrr400BadRequest => quote::quote! {"bad request"},
            Attribute::Tvfrr401Unauthorized => quote::quote! {"unauthorized"},
            Attribute::Tvfrr402PaymentRequired => quote::quote! {"payment required"},
            Attribute::Tvfrr403Forbidden => quote::quote! {"forbidden"},
            Attribute::Tvfrr404NotFound => quote::quote! {"not found"},
            Attribute::Tvfrr405MethodNotAllowed => quote::quote! {"method not allowed"},
            Attribute::Tvfrr406NotAcceptable => quote::quote! {"not acceptable"},
            Attribute::Tvfrr407ProxyAuthenticationRequired => quote::quote! {"proxy authentication required"},
            Attribute::Tvfrr408RequestTimeout => quote::quote! {"request timeout"},
            Attribute::Tvfrr409Conflict => quote::quote! {"conflict"},
            Attribute::Tvfrr410Gone => quote::quote! {"gone"},
            Attribute::Tvfrr411LengthRequired => quote::quote! {"length required"},
            Attribute::Tvfrr412PreconditionFailed => quote::quote! {"precondition failed"},
            Attribute::Tvfrr413PayloadTooLarge => quote::quote! {"payload too large"},
            Attribute::Tvfrr414UriTooLong => quote::quote! {"uri too long"},
            Attribute::Tvfrr415UnsupportedMediaType => quote::quote! {"unsupported media type"},
            Attribute::Tvfrr416RangeNotSatisfiable => quote::quote! {"range not satisfiable"},
            Attribute::Tvfrr417ExpectationFailed => quote::quote! {"expectation failed"},
            Attribute::Tvfrr418ImATeapot => quote::quote! {"im a teapot"},
            Attribute::Tvfrr421MisdirectedRequest => quote::quote! {"misdirected request"},
            Attribute::Tvfrr422UnprocessableEntity => quote::quote! {"unprocessable entity"},
            Attribute::Tvfrr423Locked => quote::quote! {"locked"},
            Attribute::Tvfrr424FailedDependency => quote::quote! {"failed dependency"},
            Attribute::Tvfrr426UpgradeRequired => quote::quote! {"upgrade required"},
            Attribute::Tvfrr428PreconditionRequired => quote::quote! {"precondition required"},
            Attribute::Tvfrr429TooManyRequests => quote::quote! {"too many requests"},
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => quote::quote! {"request header fields too large"},
            Attribute::Tvfrr451UnavailableForLegalReasons => quote::quote! {"unavailable for legal reasons"},
            Attribute::Tvfrr500InternalServerError => quote::quote! {"internal server error"},
            Attribute::Tvfrr501NotImplemented => quote::quote! {"not implemented"},
            Attribute::Tvfrr502BadGateway => quote::quote! {"bad gateway"},
            Attribute::Tvfrr503ServiceUnavailable => quote::quote! {"service unavailable"},
            Attribute::Tvfrr504GatewayTimeout => quote::quote! {"gateway timeout"},
            Attribute::Tvfrr505HttpVersionNotSupported => quote::quote! {"http version not supported"},
            Attribute::Tvfrr506VariantAlsoNegotiates => quote::quote! {"variant also negotiates"},
            Attribute::Tvfrr507InsufficientStorage => quote::quote! {"insufficient storage"},
            Attribute::Tvfrr508LoopDetected => quote::quote! {"loop detected"},
            Attribute::Tvfrr510NotExtended => quote::quote! {"not extended"},
            Attribute::Tvfrr511NetworkAuthenticationRequired => quote::quote! {"network authentication required"},
        }
    }
    pub fn to_snake_case(&self) -> std::string::String {//todo use snake case trait instead
        match self {
            Attribute::Tvfrr100Continue => std::string::String::from("tvfrr_100_continue"),
            Attribute::Tvfrr101SwitchingProtocols => std::string::String::from("tvfrr_101_switching_protocols"),
            Attribute::Tvfrr102Processing => std::string::String::from("tvfrr_102_processing"),
            Attribute::Tvfrr200Ok => std::string::String::from("tvfrr_200_ok"),
            Attribute::Tvfrr201Created => std::string::String::from("tvfrr_201_created"),
            Attribute::Tvfrr202Accepted => std::string::String::from("tvfrr_202_accepted"),
            Attribute::Tvfrr203NonAuthoritativeInformation => std::string::String::from("tvfrr_203_non_authoritative_information"),
            Attribute::Tvfrr204NoContent => std::string::String::from("tvfrr_204_no_content"),
            Attribute::Tvfrr205ResetContent => std::string::String::from("tvfrr_205_reset_content"),
            Attribute::Tvfrr206PartialContent => std::string::String::from("tvfrr_206_partial_content"),
            Attribute::Tvfrr207MultiStatus => std::string::String::from("tvfrr_207_multi_status"),
            Attribute::Tvfrr208AlreadyReported => std::string::String::from("tvfrr_208_already_reported"),
            Attribute::Tvfrr226ImUsed => std::string::String::from("tvfrr_226_im_used"),
            Attribute::Tvfrr300MultipleChoices => std::string::String::from("tvfrr_300_multiple_choices"),
            Attribute::Tvfrr301MovedPermanently => std::string::String::from("tvfrr_301_moved_permanently"),
            Attribute::Tvfrr302Found => std::string::String::from("tvfrr_302_found"),
            Attribute::Tvfrr303SeeOther => std::string::String::from("tvfrr_303_see_other"),
            Attribute::Tvfrr304NotModified => std::string::String::from("tvfrr_304_not_modified"),
            Attribute::Tvfrr305UseProxy => std::string::String::from("tvfrr_305_use_proxy"),
            Attribute::Tvfrr307TemporaryRedirect => std::string::String::from("tvfrr_307_temporary_redirect"),
            Attribute::Tvfrr308PermanentRedirect => std::string::String::from("tvfrr_308_permanent_redirect"),
            Attribute::Tvfrr400BadRequest => std::string::String::from("tvfrr_400_bad_request"),
            Attribute::Tvfrr401Unauthorized => std::string::String::from("tvfrr_401_unauthorized"),
            Attribute::Tvfrr402PaymentRequired => std::string::String::from("tvfrr_402_payment_required"),
            Attribute::Tvfrr403Forbidden => std::string::String::from("tvfrr_403_forbidden"),
            Attribute::Tvfrr404NotFound => std::string::String::from("tvfrr_404_not_found"),
            Attribute::Tvfrr405MethodNotAllowed => std::string::String::from("tvfrr_405_method_not_allowed"),
            Attribute::Tvfrr406NotAcceptable => std::string::String::from("tvfrr_406_not_acceptable"),
            Attribute::Tvfrr407ProxyAuthenticationRequired => std::string::String::from("tvfrr_407_proxy_authentication_required"),
            Attribute::Tvfrr408RequestTimeout => std::string::String::from("tvfrr_408_request_timeout"),
            Attribute::Tvfrr409Conflict => std::string::String::from("tvfrr_409_conflict"),
            Attribute::Tvfrr410Gone => std::string::String::from("tvfrr_410_gone"),
            Attribute::Tvfrr411LengthRequired => std::string::String::from("tvfrr_411_length_required"),
            Attribute::Tvfrr412PreconditionFailed => std::string::String::from("tvfrr_412_precondition_failed"),
            Attribute::Tvfrr413PayloadTooLarge => std::string::String::from("tvfrr_413_payload_too_large"),
            Attribute::Tvfrr414UriTooLong => std::string::String::from("tvfrr_414_uri_too_long"),
            Attribute::Tvfrr415UnsupportedMediaType => std::string::String::from("tvfrr_415_unsupported_media_type"),
            Attribute::Tvfrr416RangeNotSatisfiable => std::string::String::from("tvfrr_416_range_not_satisfiable"),
            Attribute::Tvfrr417ExpectationFailed => std::string::String::from("tvfrr_417_expectation_failed"),
            Attribute::Tvfrr418ImATeapot => std::string::String::from("tvfrr_418_im_a_teapot"),
            Attribute::Tvfrr421MisdirectedRequest => std::string::String::from("tvfrr_421_misdirected_request"),
            Attribute::Tvfrr422UnprocessableEntity => std::string::String::from("tvfrr_422_unprocessable_entity"),
            Attribute::Tvfrr423Locked => std::string::String::from("tvfrr_423_locked"),
            Attribute::Tvfrr424FailedDependency => std::string::String::from("tvfrr_424_failed_dependency"),
            Attribute::Tvfrr426UpgradeRequired => std::string::String::from("tvfrr_426_upgrade_required"),
            Attribute::Tvfrr428PreconditionRequired => std::string::String::from("tvfrr_428_precondition_required"),
            Attribute::Tvfrr429TooManyRequests => std::string::String::from("tvfrr_429_too_many_requests"),
            Attribute::Tvfrr431RequestHeaderFieldsTooLarge => std::string::String::from("tvfrr_431_request_header_fields_too_large"),
            Attribute::Tvfrr451UnavailableForLegalReasons => std::string::String::from("tvfrr_451_unavailable_for_legal_reasons"),
            Attribute::Tvfrr500InternalServerError => std::string::String::from("tvfrr_500_internal_server_error"),
            Attribute::Tvfrr501NotImplemented => std::string::String::from("tvfrr_501_not_implemented"),
            Attribute::Tvfrr502BadGateway => std::string::String::from("tvfrr_502_bad_gateway"),
            Attribute::Tvfrr503ServiceUnavailable => std::string::String::from("tvfrr_503_service_unavailable"),
            Attribute::Tvfrr504GatewayTimeout => std::string::String::from("tvfrr_504_gateway_timeout"),
            Attribute::Tvfrr505HttpVersionNotSupported => std::string::String::from("tvfrr_505_http_version_not_supported"),
            Attribute::Tvfrr506VariantAlsoNegotiates => std::string::String::from("tvfrr_506_variant_also_negotiates"),
            Attribute::Tvfrr507InsufficientStorage => std::string::String::from("tvfrr_507_insufficient_storage"),
            Attribute::Tvfrr508LoopDetected => std::string::String::from("tvfrr_508_loop_detected"),
            Attribute::Tvfrr510NotExtended => std::string::String::from("tvfrr_510_not_extended"),
            Attribute::Tvfrr511NetworkAuthenticationRequired => std::string::String::from("tvfrr_511_network_authentication_required"),
        }
    }
    pub fn to_attribute_view_token_stream(&self) -> proc_macro2::TokenStream {
        let value_stringified = format!("#[{}]", self.to_string());
        value_stringified.parse::<proc_macro2::TokenStream>()
        .unwrap_or_else(|_| panic!("{value_stringified} {}", crate::global_variables::hardcode::PARSE_PROC_MACRO2_TOKEN_STREAM_FAILED_MESSAGE))
    }
}

impl TryFrom<&syn::Variant> for Attribute {
    type Error = std::string::String;
    fn try_from(value: &syn::Variant) -> Result<Self, Self::Error> {
        let mut option_attribute: Option<Self> = None;
        for element in &value.attrs {
            if let true = element.path.segments.len() == 1 {
                match element.path.segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_attribute {
                                Some(value) => {
                                    return Err(format!("duplicated attributes {value} are not supported"));
                                },
                                None => {
                                    option_attribute = Some(value);
                                }
                            }
                        }
                    },
                    None => {
                        return Err(std::string::String::from("element.path.segments.first() is None"));
                    }
                }
            }
        }
        match option_attribute {
            Some(value) => Ok(value),
            None => Err(std::string::String::from("attribute not found")),
        }
    }
}

impl TryFrom<&&syn::Variant> for Attribute {
    type Error = std::string::String;
    fn try_from(value: &&syn::Variant) -> Result<Self, Self::Error> {
        let mut option_attribute: Option<Self> = None;
        for element in &value.attrs {
            if let true = element.path.segments.len() == 1 {
                match element.path.segments.first() {
                    Some(segment) => {
                        if let Ok(value) = Self::try_from(&segment.ident.to_string()) {
                            match option_attribute {
                                Some(value) => {
                                    return Err(format!("duplicated attributes {value} are not supported"));
                                },
                                None => {
                                    option_attribute = Some(value);
                                }
                            }
                        }
                    },
                    None => {
                        return Err(std::string::String::from("element.path.segments.first() is None"));
                    }
                }
            }
        }
        match option_attribute {
            Some(value) => Ok(value),
            None => Err(std::string::String::from("attribute not found")),
        }
    }
}

impl TryFrom<&std::string::String> for Attribute {
    type Error = ();
    fn try_from(value: &std::string::String) -> Result<Self, Self::Error> {
        if value == "tvfrr_100_continue" {
            Ok(Attribute::Tvfrr100Continue)
        } else if value == "tvfrr_101_switching_protocols" {
            Ok(Attribute::Tvfrr101SwitchingProtocols)
        } else if value == "tvfrr_102_processing" {
            Ok(Attribute::Tvfrr102Processing)
        } else if value == "tvfrr_200_ok" {
            Ok(Attribute::Tvfrr200Ok)
        } else if value == "tvfrr_201_created" {
            Ok(Attribute::Tvfrr201Created)
        } else if value == "tvfrr_202_accepted" {
            Ok(Attribute::Tvfrr202Accepted)
        } else if value == "tvfrr_203_non_authoritative_information" {
            Ok(Attribute::Tvfrr203NonAuthoritativeInformation)
        } else if value == "tvfrr_204_no_content" {
            Ok(Attribute::Tvfrr204NoContent)
        } else if value == "tvfrr_205_reset_content" {
            Ok(Attribute::Tvfrr205ResetContent)
        } else if value == "tvfrr_206_partial_content" {
            Ok(Attribute::Tvfrr206PartialContent)
        } else if value == "tvfrr_207_multi_status" {
            Ok(Attribute::Tvfrr207MultiStatus)
        } else if value == "tvfrr_208_already_reported" {
            Ok(Attribute::Tvfrr208AlreadyReported)
        } else if value == "tvfrr_226_im_used" {
            Ok(Attribute::Tvfrr226ImUsed)
        } else if value == "tvfrr_300_multiple_choices" {
            Ok(Attribute::Tvfrr300MultipleChoices)
        } else if value == "tvfrr_301_moved_permanently" {
            Ok(Attribute::Tvfrr301MovedPermanently)
        } else if value == "tvfrr_302_found" {
            Ok(Attribute::Tvfrr302Found)
        } else if value == "tvfrr_303_see_other" {
            Ok(Attribute::Tvfrr303SeeOther)
        } else if value == "tvfrr_304_not_modified" {
            Ok(Attribute::Tvfrr304NotModified)
        } else if value == "tvfrr_305_use_proxy" {
            Ok(Attribute::Tvfrr305UseProxy)
        } else if value == "tvfrr_307_temporary_redirect" {
            Ok(Attribute::Tvfrr307TemporaryRedirect)
        } else if value == "tvfrr_308_permanent_redirect" {
            Ok(Attribute::Tvfrr308PermanentRedirect)
        } else if value == "tvfrr_400_bad_request" {
            Ok(Attribute::Tvfrr400BadRequest)
        } else if value == "tvfrr_401_unauthorized" {
            Ok(Attribute::Tvfrr401Unauthorized)
        } else if value == "tvfrr_402_payment_required" {
            Ok(Attribute::Tvfrr402PaymentRequired)
        } else if value == "tvfrr_403_forbidden" {
            Ok(Attribute::Tvfrr403Forbidden)
        } else if value == "tvfrr_404_not_found" {
            Ok(Attribute::Tvfrr404NotFound)
        } else if value == "tvfrr_405_method_not_allowed" {
            Ok(Attribute::Tvfrr405MethodNotAllowed)
        } else if value == "tvfrr_406_not_acceptable" {
            Ok(Attribute::Tvfrr406NotAcceptable)
        } else if value == "tvfrr_407_proxy_authentication_required" {
            Ok(Attribute::Tvfrr407ProxyAuthenticationRequired)
        } else if value == "tvfrr_408_request_timeout" {
            Ok(Attribute::Tvfrr408RequestTimeout)
        } else if value == "tvfrr_409_conflict" {
            Ok(Attribute::Tvfrr409Conflict)
        } else if value == "tvfrr_410_gone" {
            Ok(Attribute::Tvfrr410Gone)
        } else if value == "tvfrr_411_length_required" {
            Ok(Attribute::Tvfrr411LengthRequired)
        } else if value == "tvfrr_412_precondition_failed" {
            Ok(Attribute::Tvfrr412PreconditionFailed)
        } else if value == "tvfrr_413_payload_too_large" {
            Ok(Attribute::Tvfrr413PayloadTooLarge)
        } else if value == "tvfrr_414_uri_too_long" {
            Ok(Attribute::Tvfrr414UriTooLong)
        } else if value == "tvfrr_415_unsupported_media_type" {
            Ok(Attribute::Tvfrr415UnsupportedMediaType)
        } else if value == "tvfrr_416_range_not_satisfiable" {
            Ok(Attribute::Tvfrr416RangeNotSatisfiable)
        } else if value == "tvfrr_417_expectation_failed" {
            Ok(Attribute::Tvfrr417ExpectationFailed)
        } else if value == "tvfrr_418_im_a_teapot" {
            Ok(Attribute::Tvfrr418ImATeapot)
        } else if value == "tvfrr_421_misdirected_request" {
            Ok(Attribute::Tvfrr421MisdirectedRequest)
        } else if value == "tvfrr_422_unprocessable_entity" {
            Ok(Attribute::Tvfrr422UnprocessableEntity)
        } else if value == "tvfrr_423_locked" {
            Ok(Attribute::Tvfrr423Locked)
        } else if value == "tvfrr_424_failed_dependency" {
            Ok(Attribute::Tvfrr424FailedDependency)
        } else if value == "tvfrr_426_upgrade_required" {
            Ok(Attribute::Tvfrr426UpgradeRequired)
        } else if value == "tvfrr_428_precondition_required" {
            Ok(Attribute::Tvfrr428PreconditionRequired)
        } else if value == "tvfrr_429_too_many_requests" {
            Ok(Attribute::Tvfrr429TooManyRequests)
        } else if value == "tvfrr_431_request_header_fields_too_large" {
            Ok(Attribute::Tvfrr431RequestHeaderFieldsTooLarge)
        } else if value == "tvfrr_451_unavailable_for_legal_reasons" {
            Ok(Attribute::Tvfrr451UnavailableForLegalReasons)
        } else if value == "tvfrr_500_internal_server_error" {
            Ok(Attribute::Tvfrr500InternalServerError)
        } else if value == "tvfrr_501_not_implemented" {
            Ok(Attribute::Tvfrr501NotImplemented)
        } else if value == "tvfrr_502_bad_gateway" {
            Ok(Attribute::Tvfrr502BadGateway)
        } else if value == "tvfrr_503_service_unavailable" {
            Ok(Attribute::Tvfrr503ServiceUnavailable)
        } else if value == "tvfrr_504_gateway_timeout" {
            Ok(Attribute::Tvfrr504GatewayTimeout)
        } else if value == "tvfrr_505_http_version_not_supported" {
            Ok(Attribute::Tvfrr505HttpVersionNotSupported)
        } else if value == "tvfrr_506_variant_also_negotiates" {
            Ok(Attribute::Tvfrr506VariantAlsoNegotiates)
        } else if value == "tvfrr_507_insufficient_storage" {
            Ok(Attribute::Tvfrr507InsufficientStorage)
        } else if value == "tvfrr_508_loop_detected" {
            Ok(Attribute::Tvfrr508LoopDetected)
        } else if value == "tvfrr_510_not_extended" {
            Ok(Attribute::Tvfrr510NotExtended)
        } else if value == "tvfrr_511_network_authentication_required" {
            Ok(Attribute::Tvfrr511NetworkAuthenticationRequired)
        } else {
            Err(())
        }
    }
}

pub fn get_only_one_attribute(
    variant: &syn::Variant,
    proc_macro_name_ident_stringified: &std::string::String
) -> Attribute {
    let mut option_attribute = None;
    variant.attrs.iter().for_each(|attr| {
        if let true = attr.path.segments.len() == 1 {
            if let Ok(named_attribute) = Attribute::try_from(&attr.path.segments[0].ident.to_string()) {
                if let true = option_attribute.is_some() {
                    panic!("{proc_macro_name_ident_stringified} duplicated attributes are not supported");
                } else {
                    option_attribute = Some(named_attribute);
                }
            }
        }
    });
    if let Some(attr) = option_attribute {
        attr
    } else {
        panic!("{proc_macro_name_ident_stringified} no supported attribute");
    }
}