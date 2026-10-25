#![allow(non_camel_case_types)]

use serde::Serialize;

#[derive(Serialize)]
struct ProviderB2Data_ {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    alias: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_key: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    application_key_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    endpoint: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct ProviderB2_ {
    data: std::cell::RefCell<ProviderB2Data_>,
}

pub struct ProviderB2(std::rc::Rc<ProviderB2_>);

impl ProviderB2 {
    pub fn provider_ref(&self) -> std::string::String {
        let data = self.0.data.borrow();
        if let std::option::Option::Some(alias) = &data.alias {
            format!("{}.{}", "b2", alias)
        } else {
            "b2".into()
        }
    }

    pub fn set_alias(self, alias: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().alias = std::option::Option::Some(alias.to_string());
        self
    }

    #[doc = "Set the field `application_key`.\nB2 Application Key (B2_APPLICATION_KEY env)."]
    pub fn set_application_key(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_key = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `application_key_id`.\nB2 Application Key ID (B2_APPLICATION_KEY_ID env)."]
    pub fn set_application_key_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().application_key_id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `endpoint`.\nB2 endpoint - the string 'production' or a custom B2 API URL (B2_ENDPOINT env). You should not need to set this unless you work at Backblaze. Defaults to `production`."]
    pub fn set_endpoint(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().endpoint = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::Provider for ProviderB2_ {
    fn extract_type_tf_id(&self) -> std::string::String {
        "b2".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "backblaze/b2",
            "version": "0.14.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderB2 {}

impl BuildProviderB2 {
    pub fn build(self, stack: &mut terrars::Stack) -> ProviderB2 {
        let out = ProviderB2(std::rc::Rc::new(ProviderB2_ { data: std::cell::RefCell::new(ProviderB2Data_ {
            alias: std::option::Option::None,
            application_key: core::default::Default::default(),
            application_key_id: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
