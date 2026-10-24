use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderB2;

#[derive(Serialize)]
struct BucketData {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::ResourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    provisioner: std::vec::Vec<terrars::Provisioner>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_info: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    bucket_name: terrars::PrimField<std::string::String>,
    bucket_type: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    cors_rules: std::option::Option<std::vec::Vec<BucketCorsRulesEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_server_side_encryption: std::option::Option<std::vec::Vec<BucketDefaultServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_lock_configuration: std::option::Option<std::vec::Vec<BucketFileLockConfigurationEl>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    lifecycle_rules: std::option::Option<std::vec::Vec<BucketLifecycleRulesEl>>,
    dynamic: BucketDynamic,
}

struct Bucket_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BucketData>,
}

#[derive(Clone)]
pub struct Bucket(std::rc::Rc<Bucket_>);

impl Bucket {
    fn shared(&self) -> &terrars::StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl terrars::Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderB2) -> Self {
        self.0.data.borrow_mut().provider = std::option::Option::Some(provider.provider_ref());
        self
    }

    pub fn set_create_before_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.create_before_destroy = v;
        self
    }

    pub fn set_prevent_destroy(self, v: bool) -> Self {
        self.0.data.borrow_mut().lifecycle.prevent_destroy = v;
        self
    }

    pub fn ignore_changes_to_all(self) -> Self {
        self.0.data.borrow_mut().lifecycle.ignore_changes =
            std::option::Option::Some(terrars::IgnoreChanges::All(terrars::IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl std::string::ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                std::option::Option::Some(i) => match i {
                    terrars::IgnoreChanges::All(_) => {
                        true
                    },
                    terrars::IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                std::option::Option::None => true,
            } {
                d.lifecycle.ignore_changes =
                    std::option::Option::Some(terrars::IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl terrars::Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl std::string::ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc = r" Add a provisioner (ex: `local-exec`, `remote-exec`, `file`) to this resource."]
    #[doc = r" Provisioners run in the order they're added."]
    pub fn add_provisioner(self, v: impl std::convert::Into<terrars::Provisioner>) -> Self {
        self.0.data.borrow_mut().provisioner.push(v.into());
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

    #[doc = "Set the field `bucket_info`.\nUser-defined information to be stored with the bucket."]
    pub fn set_bucket_info(
        self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.0.data.borrow_mut().bucket_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `cors_rules`.\n"]
    pub fn set_cors_rules(self, v: impl std::convert::Into<terrars::BlockAssignable<BucketCorsRulesEl>>) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `default_server_side_encryption`.\n"]
    pub fn set_default_server_side_encryption(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BucketDefaultServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_server_side_encryption = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_server_side_encryption = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `file_lock_configuration`.\n"]
    pub fn set_file_lock_configuration(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BucketFileLockConfigurationEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_lock_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_lock_configuration = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc = "Set the field `lifecycle_rules`.\n"]
    pub fn set_lifecycle_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BucketLifecycleRulesEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nAccount ID that the bucket belongs to."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket_info` after provisioning.\nUser-defined information to be stored with the bucket."]
    pub fn bucket_info(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.bucket_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket_type` after provisioning.\nThe bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub fn bucket_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rules` after provisioning.\n"]
    pub fn cors_rules(&self) -> terrars::ListRef<BucketCorsRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_server_side_encryption` after provisioning.\n"]
    pub fn default_server_side_encryption(&self) -> terrars::ListRef<BucketDefaultServerSideEncryptionElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_server_side_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_lock_configuration` after provisioning.\n"]
    pub fn file_lock_configuration(&self) -> terrars::ListRef<BucketFileLockConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rules` after provisioning.\n"]
    pub fn lifecycle_rules(&self) -> terrars::ListRef<BucketLifecycleRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }
}

impl terrars::Referable for Bucket {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for Bucket { }

impl terrars::ToListMappable for Bucket {
    type O = terrars::ListRef<BucketRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for Bucket_ {
    fn extract_resource_type(&self) -> std::string::String {
        "b2_bucket".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBucket {
    pub tf_id: std::string::String,
    #[doc = "The name of the bucket."]
    pub bucket_name: terrars::PrimField<std::string::String>,
    #[doc =
        "The bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub bucket_type: terrars::PrimField<std::string::String>,
}

impl BuildBucket {
    pub fn build(self, stack: &mut terrars::Stack) -> Bucket {
        let out = Bucket(std::rc::Rc::new(Bucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BucketData {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket_info: core::default::Default::default(),
                bucket_name: self.bucket_name,
                bucket_type: self.bucket_type,
                id: core::default::Default::default(),
                cors_rules: core::default::Default::default(),
                default_server_side_encryption: core::default::Default::default(),
                file_lock_configuration: core::default::Default::default(),
                lifecycle_rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BucketRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BucketRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `account_id` after provisioning.\nAccount ID that the bucket belongs to."]
    pub fn account_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket_info` after provisioning.\nUser-defined information to be stored with the bucket."]
    pub fn bucket_info(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.bucket_info", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the bucket."]
    pub fn bucket_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `bucket_type` after provisioning.\nThe bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub fn bucket_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_type", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> terrars::SetRef<terrars::PrimExpr<std::string::String>> {
        terrars::SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `cors_rules` after provisioning.\n"]
    pub fn cors_rules(&self) -> terrars::ListRef<BucketCorsRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `default_server_side_encryption` after provisioning.\n"]
    pub fn default_server_side_encryption(&self) -> terrars::ListRef<BucketDefaultServerSideEncryptionElRef> {
        terrars::ListRef::new(
            self.shared().clone(),
            format!("{}.default_server_side_encryption", self.extract_ref()),
        )
    }

    #[doc = "Get a reference to the value of field `file_lock_configuration` after provisioning.\n"]
    pub fn file_lock_configuration(&self) -> terrars::ListRef<BucketFileLockConfigurationElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `lifecycle_rules` after provisioning.\n"]
    pub fn lifecycle_rules(&self) -> terrars::ListRef<BucketLifecycleRulesElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BucketCorsRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    allowed_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    allowed_operations: terrars::ListField<terrars::PrimField<std::string::String>>,
    allowed_origins: terrars::ListField<terrars::PrimField<std::string::String>>,
    cors_rule_name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    expose_headers: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    max_age_seconds: terrars::PrimField<f64>,
}

impl BucketCorsRulesEl {
    #[doc =
        "Set the field `allowed_headers`.\nIf present, this is a list of headers that are allowed in a pre-flight OPTIONS's request's Access-Control-Request-Headers header value."]
    pub fn set_allowed_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.allowed_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `expose_headers`.\nIf present, this is a list of headers that may be exposed to an application inside the client."]
    pub fn set_expose_headers(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.expose_headers = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BucketCorsRulesEl {
    type O = terrars::BlockAssignable<BucketCorsRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketCorsRulesEl {
    #[doc = "A list specifying which operations the rule allows."]
    pub allowed_operations: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "A non-empty list specifying which origins the rule covers. "]
    pub allowed_origins: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "A name for humans to recognize the rule in a user interface."]
    pub cors_rule_name: terrars::PrimField<std::string::String>,
    #[doc =
        "This specifies the maximum number of seconds that a browser may cache the response to a preflight request."]
    pub max_age_seconds: terrars::PrimField<f64>,
}

impl BuildBucketCorsRulesEl {
    pub fn build(self) -> BucketCorsRulesEl {
        BucketCorsRulesEl {
            allowed_headers: core::default::Default::default(),
            allowed_operations: self.allowed_operations,
            allowed_origins: self.allowed_origins,
            cors_rule_name: self.cors_rule_name,
            expose_headers: core::default::Default::default(),
            max_age_seconds: self.max_age_seconds,
        }
    }
}

pub struct BucketCorsRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketCorsRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BucketCorsRulesElRef {
        BucketCorsRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketCorsRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `allowed_headers` after provisioning.\nIf present, this is a list of headers that are allowed in a pre-flight OPTIONS's request's Access-Control-Request-Headers header value."]
    pub fn allowed_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_operations` after provisioning.\nA list specifying which operations the rule allows."]
    pub fn allowed_operations(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_operations", self.base))
    }

    #[doc =
        "Get a reference to the value of field `allowed_origins` after provisioning.\nA non-empty list specifying which origins the rule covers. "]
    pub fn allowed_origins(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc =
        "Get a reference to the value of field `cors_rule_name` after provisioning.\nA name for humans to recognize the rule in a user interface."]
    pub fn cors_rule_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.cors_rule_name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `expose_headers` after provisioning.\nIf present, this is a list of headers that may be exposed to an application inside the client."]
    pub fn expose_headers(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc =
        "Get a reference to the value of field `max_age_seconds` after provisioning.\nThis specifies the maximum number of seconds that a browser may cache the response to a preflight request."]
    pub fn max_age_seconds(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketDefaultServerSideEncryptionEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl BucketDefaultServerSideEncryptionEl {
    #[doc = "Set the field `algorithm`.\nServer-side encryption algorithm. AES256 is the only one supported."]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\nServer-side encryption mode."]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BucketDefaultServerSideEncryptionEl {
    type O = terrars::BlockAssignable<BucketDefaultServerSideEncryptionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketDefaultServerSideEncryptionEl {}

impl BuildBucketDefaultServerSideEncryptionEl {
    pub fn build(self) -> BucketDefaultServerSideEncryptionEl {
        BucketDefaultServerSideEncryptionEl {
            algorithm: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct BucketDefaultServerSideEncryptionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketDefaultServerSideEncryptionElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BucketDefaultServerSideEncryptionElRef {
        BucketDefaultServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketDefaultServerSideEncryptionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `algorithm` after provisioning.\nServer-side encryption algorithm. AES256 is the only one supported."]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\nServer-side encryption mode."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    duration: terrars::PrimField<f64>,
    unit: terrars::PrimField<std::string::String>,
}

impl BucketFileLockConfigurationElDefaultRetentionElPeriodEl { }

impl terrars::ToListMappable for BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    type O = terrars::BlockAssignable<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    #[doc = "Duration"]
    pub duration: terrars::PrimField<f64>,
    #[doc = "Unit for duration (days|years)"]
    pub unit: terrars::PrimField<std::string::String>,
}

impl BuildBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    pub fn build(self) -> BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
        BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
            duration: self.duration,
            unit: self.unit,
        }
    }
}

pub struct BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
        BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `duration` after provisioning.\nDuration"]
    pub fn duration(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc = "Get a reference to the value of field `unit` after provisioning.\nUnit for duration (days|years)"]
    pub fn unit(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileLockConfigurationElDefaultRetentionElDynamic {
    period: std::option::Option<terrars::DynamicBlock<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationElDefaultRetentionEl {
    mode: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    period: std::option::Option<std::vec::Vec<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
    dynamic: BucketFileLockConfigurationElDefaultRetentionElDynamic,
}

impl BucketFileLockConfigurationElDefaultRetentionEl {
    #[doc = "Set the field `period`.\n"]
    pub fn set_period(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.period = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.period = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BucketFileLockConfigurationElDefaultRetentionEl {
    type O = terrars::BlockAssignable<BucketFileLockConfigurationElDefaultRetentionEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileLockConfigurationElDefaultRetentionEl {
    #[doc = "Default retention mode (compliance|governance|none)."]
    pub mode: terrars::PrimField<std::string::String>,
}

impl BuildBucketFileLockConfigurationElDefaultRetentionEl {
    pub fn build(self) -> BucketFileLockConfigurationElDefaultRetentionEl {
        BucketFileLockConfigurationElDefaultRetentionEl {
            mode: self.mode,
            period: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BucketFileLockConfigurationElDefaultRetentionElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketFileLockConfigurationElDefaultRetentionElRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BucketFileLockConfigurationElDefaultRetentionElRef {
        BucketFileLockConfigurationElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElDefaultRetentionElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `mode` after provisioning.\nDefault retention mode (compliance|governance|none)."]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc = "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> terrars::ListRef<BucketFileLockConfigurationElDefaultRetentionElPeriodElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileLockConfigurationElDynamic {
    default_retention: std::option::Option<terrars::DynamicBlock<BucketFileLockConfigurationElDefaultRetentionEl>>,
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_file_lock_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    default_retention: std::option::Option<std::vec::Vec<BucketFileLockConfigurationElDefaultRetentionEl>>,
    dynamic: BucketFileLockConfigurationElDynamic,
}

impl BucketFileLockConfigurationEl {
    #[doc =
        "Set the field `is_file_lock_enabled`.\nIf present, the boolean value specifies whether bucket is File Lock-enabled. Defaults to `false`."]
    pub fn set_is_file_lock_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_file_lock_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl std::convert::Into<terrars::BlockAssignable<BucketFileLockConfigurationElDefaultRetentionEl>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.default_retention = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BucketFileLockConfigurationEl {
    type O = terrars::BlockAssignable<BucketFileLockConfigurationEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileLockConfigurationEl {}

impl BuildBucketFileLockConfigurationEl {
    pub fn build(self) -> BucketFileLockConfigurationEl {
        BucketFileLockConfigurationEl {
            is_file_lock_enabled: core::default::Default::default(),
            default_retention: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

pub struct BucketFileLockConfigurationElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketFileLockConfigurationElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BucketFileLockConfigurationElRef {
        BucketFileLockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `is_file_lock_enabled` after provisioning.\nIf present, the boolean value specifies whether bucket is File Lock-enabled. Defaults to `false`."]
    pub fn is_file_lock_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_file_lock_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> terrars::ListRef<BucketFileLockConfigurationElDefaultRetentionElRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketLifecycleRulesEl {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_from_hiding_to_deleting: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    days_from_uploading_to_hiding: std::option::Option<terrars::PrimField<f64>>,
    file_name_prefix: terrars::PrimField<std::string::String>,
}

impl BucketLifecycleRulesEl {
    #[doc =
        "Set the field `days_from_hiding_to_deleting`.\nIt says how long to keep file versions that are not the current version."]
    pub fn set_days_from_hiding_to_deleting(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_from_hiding_to_deleting = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `days_from_uploading_to_hiding`.\nIt causes files to be hidden automatically after the given number of days."]
    pub fn set_days_from_uploading_to_hiding(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.days_from_uploading_to_hiding = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for BucketLifecycleRulesEl {
    type O = terrars::BlockAssignable<BucketLifecycleRulesEl>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketLifecycleRulesEl {
    #[doc = "It specifies which files in the bucket it applies to."]
    pub file_name_prefix: terrars::PrimField<std::string::String>,
}

impl BuildBucketLifecycleRulesEl {
    pub fn build(self) -> BucketLifecycleRulesEl {
        BucketLifecycleRulesEl {
            days_from_hiding_to_deleting: core::default::Default::default(),
            days_from_uploading_to_hiding: core::default::Default::default(),
            file_name_prefix: self.file_name_prefix,
        }
    }
}

pub struct BucketLifecycleRulesElRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketLifecycleRulesElRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BucketLifecycleRulesElRef {
        BucketLifecycleRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketLifecycleRulesElRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `days_from_hiding_to_deleting` after provisioning.\nIt says how long to keep file versions that are not the current version."]
    pub fn days_from_hiding_to_deleting(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_from_hiding_to_deleting", self.base))
    }

    #[doc =
        "Get a reference to the value of field `days_from_uploading_to_hiding` after provisioning.\nIt causes files to be hidden automatically after the given number of days."]
    pub fn days_from_uploading_to_hiding(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.days_from_uploading_to_hiding", self.base))
    }

    #[doc =
        "Get a reference to the value of field `file_name_prefix` after provisioning.\nIt specifies which files in the bucket it applies to."]
    pub fn file_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketDynamic {
    cors_rules: std::option::Option<terrars::DynamicBlock<BucketCorsRulesEl>>,
    default_server_side_encryption: std::option::Option<terrars::DynamicBlock<BucketDefaultServerSideEncryptionEl>>,
    file_lock_configuration: std::option::Option<terrars::DynamicBlock<BucketFileLockConfigurationEl>>,
    lifecycle_rules: std::option::Option<terrars::DynamicBlock<BucketLifecycleRulesEl>>,
}
