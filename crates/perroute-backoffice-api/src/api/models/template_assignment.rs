use validator::Validate;

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct CreateTemplateAssignmentRequest {
    // #[validate(required)]
    // #[validate(custom = "perroute_commons::types::name::validate")]
    // name: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // subject: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // html: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // text: Option<String>,
}

#[derive(Debug, serde::Deserialize, Clone, Validate, Default)]
#[serde(default)]
pub struct UpdateTemplateAssignmentRequest {
    // #[validate(required)]
    // #[validate(custom = "perroute_commons::types::name::validate")]
    // name: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // subject: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // html: Option<String>,

    // #[validate(custom = "perroute_commons::types::template::handlebars::Handlebars::validate")]
    // text: Option<String>,
}
