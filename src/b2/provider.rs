use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;

#[derive(Serialize)]
struct ProviderB2Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_key: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoint: Option<PrimField<String>>,
}

struct ProviderB2_ {
    data: RefCell<ProviderB2Data>,
}

pub struct ProviderB2(Rc<ProviderB2_>);

impl ProviderB2 {
    pub fn provider_ref(&self) -> String {
        let data = self.0.data.borrow();
        if let Some(alias) = &data.alias {
            format!("{}.{}", "b2", alias)
        } else {
            "b2".into()
        }
    }

    pub fn set_alias(self, alias: impl ToString) -> Self {
        self.0.data.borrow_mut().alias = Some(alias.to_string());
        self
    }

    #[doc= "Set the field `application_key`.\nB2 Application Key (B2_APPLICATION_KEY env)"]
    pub fn set_application_key(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_key = Some(v.into());
        self
    }

    #[doc= "Set the field `application_key_id`.\nB2 Application Key ID (B2_APPLICATION_KEY_ID env)"]
    pub fn set_application_key_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().application_key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `endpoint`.\nB2 endpoint - production or custom URL (B2_ENDPOINT env)"]
    pub fn set_endpoint(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().endpoint = Some(v.into());
        self
    }
}

impl Provider for ProviderB2_ {
    fn extract_type_tf_id(&self) -> String {
        "b2".into()
    }

    fn extract_provider_type(&self) -> serde_json::Value {
        serde_json::json!({
            "source": "backblaze/b2",
            "version": "0.8.1",
        })
    }

    fn extract_provider(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildProviderB2 {}

impl BuildProviderB2 {
    pub fn build(self, stack: &mut Stack) -> ProviderB2 {
        let out = ProviderB2(Rc::new(ProviderB2_ { data: RefCell::new(ProviderB2Data {
            alias: None,
            application_key: core::default::Default::default(),
            application_key_id: core::default::Default::default(),
            endpoint: core::default::Default::default(),
        }) }));
        stack.add_provider(out.0.clone());
        out
    }
}
