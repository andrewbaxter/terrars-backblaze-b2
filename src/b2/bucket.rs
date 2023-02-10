use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct BucketData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_info: Option<RecField<PrimField<String>>>,
    bucket_name: PrimField<String>,
    bucket_type: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_rules: Option<Vec<BucketCorsRulesEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_server_side_encryption: Option<Vec<BucketDefaultServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_lock_configuration: Option<Vec<BucketFileLockConfigurationEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lifecycle_rules: Option<Vec<BucketLifecycleRulesEl>>,
    dynamic: BucketDynamic,
}

struct Bucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BucketData>,
}

#[derive(Clone)]
pub struct Bucket(Rc<Bucket_>);

impl Bucket {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Referable) -> Self {
        self.0.data.borrow_mut().depends_on.push(dep.extract_ref());
        self
    }

    pub fn set_provider(self, provider: &ProviderB2) -> Self {
        self.0.data.borrow_mut().provider = Some(provider.provider_ref());
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
        self.0.data.borrow_mut().lifecycle.ignore_changes = Some(IgnoreChanges::All(IgnoreChangesAll::All));
        self
    }

    pub fn ignore_changes_to_attr(self, attr: impl ToString) -> Self {
        {
            let mut d = self.0.data.borrow_mut();
            if match &mut d.lifecycle.ignore_changes {
                Some(i) => match i {
                    IgnoreChanges::All(_) => {
                        true
                    },
                    IgnoreChanges::Refs(r) => {
                        r.push(attr.to_string());
                        false
                    },
                },
                None => true,
            } {
                d.lifecycle.ignore_changes = Some(IgnoreChanges::Refs(vec![attr.to_string()]));
            }
        }
        self
    }

    pub fn replace_triggered_by_resource(self, r: &impl Resource) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(r.extract_ref());
        self
    }

    pub fn replace_triggered_by_attr(self, attr: impl ToString) -> Self {
        self.0.data.borrow_mut().lifecycle.replace_triggered_by.push(attr.to_string());
        self
    }

    #[doc= "Set the field `bucket_info`.\nUser-defined information to be stored with the bucket."]
    pub fn set_bucket_info(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().bucket_info = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `cors_rules`.\n"]
    pub fn set_cors_rules(self, v: impl Into<BlockAssignable<BucketCorsRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().cors_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.cors_rules = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `default_server_side_encryption`.\n"]
    pub fn set_default_server_side_encryption(
        self,
        v: impl Into<BlockAssignable<BucketDefaultServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().default_server_side_encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.default_server_side_encryption = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `file_lock_configuration`.\n"]
    pub fn set_file_lock_configuration(self, v: impl Into<BlockAssignable<BucketFileLockConfigurationEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().file_lock_configuration = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.file_lock_configuration = Some(d);
            },
        }
        self
    }

    #[doc= "Set the field `lifecycle_rules`.\n"]
    pub fn set_lifecycle_rules(self, v: impl Into<BlockAssignable<BucketLifecycleRulesEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().lifecycle_rules = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.lifecycle_rules = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nAccount ID that the bucket belongs to."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_info` after provisioning.\nUser-defined information to be stored with the bucket."]
    pub fn bucket_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.bucket_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the bucket."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_type` after provisioning.\nThe bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub fn bucket_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_rules` after provisioning.\n"]
    pub fn cors_rules(&self) -> ListRef<BucketCorsRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_server_side_encryption` after provisioning.\n"]
    pub fn default_server_side_encryption(&self) -> ListRef<BucketDefaultServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_lock_configuration` after provisioning.\n"]
    pub fn file_lock_configuration(&self) -> ListRef<BucketFileLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rules` after provisioning.\n"]
    pub fn lifecycle_rules(&self) -> ListRef<BucketLifecycleRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }
}

impl Referable for Bucket {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Resource for Bucket { }

impl ToListMappable for Bucket {
    type O = ListRef<BucketRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for Bucket_ {
    fn extract_resource_type(&self) -> String {
        "b2_bucket".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBucket {
    pub tf_id: String,
    #[doc= "The name of the bucket."]
    pub bucket_name: PrimField<String>,
    #[doc= "The bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub bucket_type: PrimField<String>,
}

impl BuildBucket {
    pub fn build(self, stack: &mut Stack) -> Bucket {
        let out = Bucket(Rc::new(Bucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BucketData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_info: core::default::Default::default(),
                bucket_name: self.bucket_name,
                bucket_type: self.bucket_type,
                id: core::default::Default::default(),
                cors_rules: core::default::Default::default(),
                default_server_side_encryption: core::default::Default::default(),
                file_lock_configuration: core::default::Default::default(),
                lifecycle_rules: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BucketRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BucketRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `account_id` after provisioning.\nAccount ID that the bucket belongs to."]
    pub fn account_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.account_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_info` after provisioning.\nUser-defined information to be stored with the bucket."]
    pub fn bucket_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.bucket_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_name` after provisioning.\nThe name of the bucket."]
    pub fn bucket_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_type` after provisioning.\nThe bucket type. Either 'allPublic', meaning that files in this bucket can be downloaded by anybody, or 'allPrivate'."]
    pub fn bucket_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `cors_rules` after provisioning.\n"]
    pub fn cors_rules(&self) -> ListRef<BucketCorsRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_server_side_encryption` after provisioning.\n"]
    pub fn default_server_side_encryption(&self) -> ListRef<BucketDefaultServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_lock_configuration` after provisioning.\n"]
    pub fn file_lock_configuration(&self) -> ListRef<BucketFileLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rules` after provisioning.\n"]
    pub fn lifecycle_rules(&self) -> ListRef<BucketLifecycleRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BucketCorsRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<ListField<PrimField<String>>>,
    allowed_operations: ListField<PrimField<String>>,
    allowed_origins: ListField<PrimField<String>>,
    cors_rule_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    max_age_seconds: PrimField<f64>,
}

impl BucketCorsRulesEl {
    #[doc= "Set the field `allowed_headers`.\nIf present, this is a list of headers that are allowed in a pre-flight OPTIONS's request's Access-Control-Request-Headers header value."]
    pub fn set_allowed_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\nIf present, this is a list of headers that may be exposed to an application inside the client."]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }
}

impl ToListMappable for BucketCorsRulesEl {
    type O = BlockAssignable<BucketCorsRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketCorsRulesEl {
    #[doc= "A list specifying which operations the rule allows."]
    pub allowed_operations: ListField<PrimField<String>>,
    #[doc= "A non-empty list specifying which origins the rule covers. "]
    pub allowed_origins: ListField<PrimField<String>>,
    #[doc= "A name for humans to recognize the rule in a user interface."]
    pub cors_rule_name: PrimField<String>,
    #[doc= "This specifies the maximum number of seconds that a browser may cache the response to a preflight request."]
    pub max_age_seconds: PrimField<f64>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BucketCorsRulesElRef {
    fn new(shared: StackShared, base: String) -> BucketCorsRulesElRef {
        BucketCorsRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketCorsRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_headers` after provisioning.\nIf present, this is a list of headers that are allowed in a pre-flight OPTIONS's request's Access-Control-Request-Headers header value."]
    pub fn allowed_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_operations` after provisioning.\nA list specifying which operations the rule allows."]
    pub fn allowed_operations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_operations", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_origins` after provisioning.\nA non-empty list specifying which origins the rule covers. "]
    pub fn allowed_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `cors_rule_name` after provisioning.\nA name for humans to recognize the rule in a user interface."]
    pub fn cors_rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cors_rule_name", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\nIf present, this is a list of headers that may be exposed to an application inside the client."]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age_seconds` after provisioning.\nThis specifies the maximum number of seconds that a browser may cache the response to a preflight request."]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketDefaultServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl BucketDefaultServerSideEncryptionEl {
    #[doc= "Set the field `algorithm`.\nServer-side encryption algorithm. AES256 is the only one supported."]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\nServer-side encryption mode."]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for BucketDefaultServerSideEncryptionEl {
    type O = BlockAssignable<BucketDefaultServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
    shared: StackShared,
    base: String,
}

impl Ref for BucketDefaultServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> BucketDefaultServerSideEncryptionElRef {
        BucketDefaultServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketDefaultServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\nServer-side encryption algorithm. AES256 is the only one supported."]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nServer-side encryption mode."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    duration: PrimField<f64>,
    unit: PrimField<String>,
}

impl BucketFileLockConfigurationElDefaultRetentionElPeriodEl { }

impl ToListMappable for BucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    type O = BlockAssignable<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    #[doc= "Duration"]
    pub duration: PrimField<f64>,
    #[doc= "Unit for duration (days|years)"]
    pub unit: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn new(shared: StackShared, base: String) -> BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
        BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nDuration"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `unit` after provisioning.\nUnit for duration (days|years)"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileLockConfigurationElDefaultRetentionElDynamic {
    period: Option<DynamicBlock<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationElDefaultRetentionEl {
    mode: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Vec<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
    dynamic: BucketFileLockConfigurationElDefaultRetentionElDynamic,
}

impl BucketFileLockConfigurationElDefaultRetentionEl {
    #[doc= "Set the field `period`.\n"]
    pub fn set_period(
        mut self,
        v: impl Into<BlockAssignable<BucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.period = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.period = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BucketFileLockConfigurationElDefaultRetentionEl {
    type O = BlockAssignable<BucketFileLockConfigurationElDefaultRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileLockConfigurationElDefaultRetentionEl {
    #[doc= "Default retention mode (compliance|governance|none)."]
    pub mode: PrimField<String>,
}

impl BuildBucketFileLockConfigurationElDefaultRetentionEl {
    pub fn build(self) -> BucketFileLockConfigurationElDefaultRetentionEl {
        BucketFileLockConfigurationElDefaultRetentionEl {
            mode: self.mode,
            period: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BucketFileLockConfigurationElDefaultRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileLockConfigurationElDefaultRetentionElRef {
    fn new(shared: StackShared, base: String) -> BucketFileLockConfigurationElDefaultRetentionElRef {
        BucketFileLockConfigurationElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElDefaultRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\nDefault retention mode (compliance|governance|none)."]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> ListRef<BucketFileLockConfigurationElDefaultRetentionElPeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileLockConfigurationElDynamic {
    default_retention: Option<DynamicBlock<BucketFileLockConfigurationElDefaultRetentionEl>>,
}

#[derive(Serialize)]
pub struct BucketFileLockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_file_lock_enabled: Option<PrimField<bool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_retention: Option<Vec<BucketFileLockConfigurationElDefaultRetentionEl>>,
    dynamic: BucketFileLockConfigurationElDynamic,
}

impl BucketFileLockConfigurationEl {
    #[doc= "Set the field `is_file_lock_enabled`.\nIf present, the boolean value specifies whether bucket is File Lock-enabled. Defaults to `false`."]
    pub fn set_is_file_lock_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_file_lock_enabled = Some(v.into());
        self
    }

    #[doc= "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl Into<BlockAssignable<BucketFileLockConfigurationElDefaultRetentionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.default_retention = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.default_retention = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BucketFileLockConfigurationEl {
    type O = BlockAssignable<BucketFileLockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
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
            dynamic: Default::default(),
        }
    }
}

pub struct BucketFileLockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileLockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> BucketFileLockConfigurationElRef {
        BucketFileLockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileLockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `is_file_lock_enabled` after provisioning.\nIf present, the boolean value specifies whether bucket is File Lock-enabled. Defaults to `false`."]
    pub fn is_file_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_file_lock_enabled", self.base))
    }

    #[doc= "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> ListRef<BucketFileLockConfigurationElDefaultRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }
}

#[derive(Serialize)]
pub struct BucketLifecycleRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_from_hiding_to_deleting: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_from_uploading_to_hiding: Option<PrimField<f64>>,
    file_name_prefix: PrimField<String>,
}

impl BucketLifecycleRulesEl {
    #[doc= "Set the field `days_from_hiding_to_deleting`.\nIt says how long to keep file versions that are not the current version."]
    pub fn set_days_from_hiding_to_deleting(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_from_hiding_to_deleting = Some(v.into());
        self
    }

    #[doc= "Set the field `days_from_uploading_to_hiding`.\nIt causes files to be hidden automatically after the given number of days."]
    pub fn set_days_from_uploading_to_hiding(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_from_uploading_to_hiding = Some(v.into());
        self
    }
}

impl ToListMappable for BucketLifecycleRulesEl {
    type O = BlockAssignable<BucketLifecycleRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketLifecycleRulesEl {
    #[doc= "It specifies which files in the bucket it applies to."]
    pub file_name_prefix: PrimField<String>,
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
    shared: StackShared,
    base: String,
}

impl Ref for BucketLifecycleRulesElRef {
    fn new(shared: StackShared, base: String) -> BucketLifecycleRulesElRef {
        BucketLifecycleRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketLifecycleRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `days_from_hiding_to_deleting` after provisioning.\nIt says how long to keep file versions that are not the current version."]
    pub fn days_from_hiding_to_deleting(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days_from_hiding_to_deleting", self.base))
    }

    #[doc= "Get a reference to the value of field `days_from_uploading_to_hiding` after provisioning.\nIt causes files to be hidden automatically after the given number of days."]
    pub fn days_from_uploading_to_hiding(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days_from_uploading_to_hiding", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name_prefix` after provisioning.\nIt specifies which files in the bucket it applies to."]
    pub fn file_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_prefix", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketDynamic {
    cors_rules: Option<DynamicBlock<BucketCorsRulesEl>>,
    default_server_side_encryption: Option<DynamicBlock<BucketDefaultServerSideEncryptionEl>>,
    file_lock_configuration: Option<DynamicBlock<BucketFileLockConfigurationEl>>,
    lifecycle_rules: Option<DynamicBlock<BucketLifecycleRulesEl>>,
}
