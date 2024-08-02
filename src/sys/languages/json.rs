use js_sys::Object;
use wasm_bindgen::prelude::*;

str_enum! {
    pub enum SeverityLevel {
        Error = "error",
        Warning = "warning",
        Ignore = "ignore",
    }
}

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/editor.js", js_namespace = ["languages", "json"]))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/editor.js", js_namespace = ["languages", "json"]))]
extern "C" {

    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type DiagnosticsOptions;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "allowComments", getter = allowComments)]
    pub fn allow_comments(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "comments", getter = comments)]
    pub fn comments(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "enableSchemaRequest", getter = enableSchemaRequest)]
    pub fn enable_schema_request(this: &DiagnosticsOptions) -> bool;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "schemaRequest", getter = schemaRequest)]
    pub fn schema_request(this: &DiagnosticsOptions) -> SeverityLevel;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "schemaValidation", getter = schemaValidation)]
    pub fn schema_validation(this: &DiagnosticsOptions) -> SeverityLevel;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "schemas", getter = schemas)]
    pub fn schemas(this: &DiagnosticsOptions) -> Object;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "trailingCommas", getter = trailingCommas)]
    pub fn trailing_commas(this: &DiagnosticsOptions) -> SeverityLevel;

    #[wasm_bindgen(method, js_class = "DiagnosticOptions", js_name = "validate", getter = validate)]
    pub fn validate(this: &DiagnosticsOptions) -> bool;
}

impl Default for DiagnosticsOptions {
    fn default() -> Self {
        Object::new().unchecked_into()
    }
}

impl DiagnosticsOptions {
    pub fn new(
        allow_comments: Option<bool>,
        comment_severity_level: Option<SeverityLevel>,
        enable_schema_request: Option<bool>,
        schema_request: Option<SeverityLevel>,
        schema_validation: Option<SeverityLevel>,
        schemas: Option<JsValue>,
        trailing_commas: Option<SeverityLevel>,
        validate: Option<bool>,
    ) -> Self {
        let diagnostics = DiagnosticsOptions::default();
        let process_severity_level =
            |severity_level: SeverityLevel| severity_level.to_value().to_lowercase();
        object_set_optional!(diagnostics.allowComments = allow_comments);
        object_set_optional!(
            diagnostics.comments = comment_severity_level.map(process_severity_level)
        );
        object_set_optional!(diagnostics.enableSchemaRequest = enable_schema_request);
        object_set_optional!(
            diagnostics.schemaRequest = schema_request.map(process_severity_level)
        );
        object_set_optional!(
            diagnostics.schemaValidation = schema_validation.map(process_severity_level)
        );
        object_set_optional!(diagnostics.schemas = schemas);
        object_set_optional!(
            diagnostics.trailingCommas = trailing_commas.map(process_severity_level)
        );
        object_set_optional!(diagnostics.validate = validate);
        diagnostics
    }
}

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/editor.js", js_namespace = ["languages", "json"]))]
#[cfg_attr(not(debug_assertions), wasm_bindgen(module = "/js/release/editor.js", js_namespace = ["languages", "json"]))]
extern "C" {

    #[derive(Debug)]
    #[wasm_bindgen(extends = Object)]
    pub type LanguageServiceDefaults;

    #[derive(Debug)]
    #[wasm_bindgen(js_name = "jsonDefaults")]
    pub static JSON_DEFAULTS: LanguageServiceDefaults;

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "diagnosticsOptions", getter = diagnosticsOptions)]
    pub fn diagnostic_options(this: &LanguageServiceDefaults) -> DiagnosticsOptions;

    #[wasm_bindgen(method, js_class = "LanguageServiceDefaults", js_name = "languageId", getter = languageId)]
    pub fn language_id(this: &LanguageServiceDefaults) -> String;

    #[wasm_bindgen(
        method,
        js_class = "LanguageServiceDefaults",
        js_name = "setDiagnosticsOptions"
    )]
    pub fn set_diagnostic_options(this: &LanguageServiceDefaults, options: &DiagnosticsOptions);

}
