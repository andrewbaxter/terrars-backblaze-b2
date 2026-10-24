use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataAccountInfoData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataAccountInfo_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataAccountInfoData>,
}

#[derive(Clone)]
pub struct DataAccountInfo(std::rc::Rc<DataAccountInfo_>);

impl DataAccountInfo {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderB2) -> &Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn add_lifecycle_precondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.precondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    pub fn add_lifecycle_postcondition(
        self,
        condition: impl std::convert::Into<terrars::PrimField<bool>>,
        error_message: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.0.data.borrow_mut().lifecycle.postcondition.push(terrars::LifecycleCondition {
            condition: condition.into(),
            error_message: error_message.into(),
        });
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Get a reference to the value of field `account_auth_token` after provisioning.\nAn authorization token to use with all calls, other than b2_authorize_account, that need an Authorization header. This authorization token is valid for at most 24 hours."]
    pub fn account_auth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_auth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe identifier for the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed` after provisioning.\nAn object containing the capabilities of this auth token, and any restrictions on using it."]
    pub fn allowed(&self) -> terrars::ListRef<DataAccountInfoAllowedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to use for all API calls except for uploading and downloading files."]
    pub fn api_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `download_url` after provisioning.\nThe base URL to use for downloading files."]
    pub fn download_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.download_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `s3_api_url` after provisioning.\nThe base URL to use for S3-compatible API calls."]
    pub fn s3_api_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_api_url", self.extract_ref()))
    }
}

impl terrars::Referable for DataAccountInfo {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataAccountInfo { }

impl terrars::ToListMappable for DataAccountInfo {
    type O = terrars::ListRef<DataAccountInfoRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataAccountInfo_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "b2_account_info".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountInfo {
    pub tf_id: std::string::String,
}

impl BuildDataAccountInfo {
    pub fn build(self, stack: &mut terrars::Stack) -> DataAccountInfo {
        let out = DataAccountInfo(std::rc::Rc::new(DataAccountInfo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataAccountInfoData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountInfoRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountInfoRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountInfoRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc =
        "Get a reference to the value of field `account_auth_token` after provisioning.\nAn authorization token to use with all calls, other than b2_authorize_account, that need an Authorization header. This authorization token is valid for at most 24 hours."]
    pub fn account_auth_token(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_auth_token", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `account_id` after provisioning.\nThe identifier for the account."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `allowed` after provisioning.\nAn object containing the capabilities of this auth token, and any restrictions on using it."]
    pub fn allowed(&self) -> terrars::ListRef<DataAccountInfoAllowedElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to use for all API calls except for uploading and downloading files."]
    pub fn api_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `download_url` after provisioning.\nThe base URL to use for downloading files."]
    pub fn download_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.download_url", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `s3_api_url` after provisioning.\nThe base URL to use for S3-compatible API calls."]
    pub fn s3_api_url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.s3_api_url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountInfoAllowedEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    capabilities: std::option::Option<terrars::SetField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataAccountInfoAllowedEl {
    #[doc = "Set the field `bucket_id`.\n"]
    pub fn set_bucket_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `capabilities`.\n"]
    pub fn set_capabilities(
        mut self,
        v: impl std::convert::Into<terrars::SetField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.capabilities = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name_prefix = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataAccountInfoAllowedEl {
    type O = terrars::BlockAssignable<DataAccountInfoAllowedEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataAccountInfoAllowedEl {}

impl BuildDataAccountInfoAllowedEl {
    pub fn build(self) -> DataAccountInfoAllowedEl {
        DataAccountInfoAllowedEl {
            bucket_id: core::default::Default::default(),
            bucket_name: core::default::Default::default(),
            capabilities: core::default::Default::default(),
            name_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataAccountInfoAllowedElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataAccountInfoAllowedElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataAccountInfoAllowedElRef {
        DataAccountInfoAllowedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountInfoAllowedElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\n"]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc = "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc = "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }
}
