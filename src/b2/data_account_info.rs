use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataAccountInfoData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataAccountInfo_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataAccountInfoData>,
}

#[derive(Clone)]
pub struct DataAccountInfo(Rc<DataAccountInfo_>);

impl DataAccountInfo {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(&self, provider: &ProviderB2) -> &Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `account_auth_token` after provisioning.\nAn authorization token to use with all calls, other than b2_authorize_account, that need an Authorization header. This authorization token is valid for at most 24 hours."]
    pub fn account_auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_auth_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe identifier for the account."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed` after provisioning.\nAn object containing the capabilities of this auth token, and any restrictions on using it."]
    pub fn allowed(&self) -> ListRef<DataAccountInfoAllowedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to use for all API calls except for uploading and downloading files."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `download_url` after provisioning.\nThe base URL to use for downloading files."]
    pub fn download_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.download_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_api_url` after provisioning.\nThe base URL to use for S3-compatible API calls."]
    pub fn s3_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_api_url", self.extract_ref()))
    }
}

impl Datasource for DataAccountInfo {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataAccountInfo {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataAccountInfo {
    type O = ListRef<DataAccountInfoRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataAccountInfo_ {
    fn extract_datasource_type(&self) -> String {
        "b2_account_info".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataAccountInfo {
    pub tf_id: String,
}

impl BuildDataAccountInfo {
    pub fn build(self, stack: &mut Stack) -> DataAccountInfo {
        let out = DataAccountInfo(Rc::new(DataAccountInfo_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataAccountInfoData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataAccountInfoRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataAccountInfoRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataAccountInfoRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `account_auth_token` after provisioning.\nAn authorization token to use with all calls, other than b2_authorize_account, that need an Authorization header. This authorization token is valid for at most 24 hours."]
    pub fn account_auth_token(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_auth_token", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nThe identifier for the account."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `allowed` after provisioning.\nAn object containing the capabilities of this auth token, and any restrictions on using it."]
    pub fn allowed(&self) -> ListRef<DataAccountInfoAllowedElRef> {
        ListRef::new(self.shared().clone(), format!("{}.allowed", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `api_url` after provisioning.\nThe base URL to use for all API calls except for uploading and downloading files."]
    pub fn api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.api_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `download_url` after provisioning.\nThe base URL to use for downloading files."]
    pub fn download_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.download_url", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `s3_api_url` after provisioning.\nThe base URL to use for S3-compatible API calls."]
    pub fn s3_api_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.s3_api_url", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataAccountInfoAllowedEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<SetField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
}

impl DataAccountInfoAllowedEl {
    #[doc= "Set the field `bucket_id`.\n"]
    pub fn set_bucket_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_id = Some(v.into());
        self
    }

    #[doc= "Set the field `bucket_name`.\n"]
    pub fn set_bucket_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_name = Some(v.into());
        self
    }

    #[doc= "Set the field `capabilities`.\n"]
    pub fn set_capabilities(mut self, v: impl Into<SetField<PrimField<String>>>) -> Self {
        self.capabilities = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\n"]
    pub fn set_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.name_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataAccountInfoAllowedEl {
    type O = BlockAssignable<DataAccountInfoAllowedEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for DataAccountInfoAllowedElRef {
    fn new(shared: StackShared, base: String) -> DataAccountInfoAllowedElRef {
        DataAccountInfoAllowedElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataAccountInfoAllowedElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\n"]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.base))
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\n"]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.base))
    }

    #[doc= "Get a reference to the value of field `capabilities` after provisioning.\n"]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capabilities", self.base))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\n"]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.base))
    }
}
