use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataBucketData {
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataBucket_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBucketData>,
}

#[derive(Clone)]
pub struct DataBucket(Rc<DataBucket_>);

impl DataBucket {
    fn shared(&self) -> &StackShared {
        &self.0.shared
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

    #[doc= "Get a reference to the value of field `cors_rules` after provisioning.\nThe initial list of CORS rules for this bucket."]
    pub fn cors_rules(&self) -> ListRef<DataBucketCorsRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_server_side_encryption` after provisioning.\nThe default server-side encryption settings of this bucket."]
    pub fn default_server_side_encryption(&self) -> ListRef<DataBucketDefaultServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_lock_configuration` after provisioning.\nThe default File Lock retention settings for this bucket."]
    pub fn file_lock_configuration(&self) -> ListRef<DataBucketFileLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rules` after provisioning.\nThe initial list of lifecycle rules for this bucket."]
    pub fn lifecycle_rules(&self) -> ListRef<DataBucketLifecycleRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }
}

impl Datasource for DataBucket {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl ToListMappable for DataBucket {
    type O = ListRef<DataBucketRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Datasource_ for DataBucket_ {
    fn extract_datasource_type(&self) -> String {
        "b2_bucket".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBucket {
    pub tf_id: String,
    #[doc= "The name of the bucket."]
    pub bucket_name: PrimField<String>,
}

impl BuildDataBucket {
    pub fn build(self, stack: &mut Stack) -> DataBucket {
        let out = DataBucket(Rc::new(DataBucket_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBucketData {
                provider: None,
                for_each: None,
                bucket_name: self.bucket_name,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBucketRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBucketRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
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

    #[doc= "Get a reference to the value of field `cors_rules` after provisioning.\nThe initial list of CORS rules for this bucket."]
    pub fn cors_rules(&self) -> ListRef<DataBucketCorsRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.cors_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `default_server_side_encryption` after provisioning.\nThe default server-side encryption settings of this bucket."]
    pub fn default_server_side_encryption(&self) -> ListRef<DataBucketDefaultServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_server_side_encryption", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_lock_configuration` after provisioning.\nThe default File Lock retention settings for this bucket."]
    pub fn file_lock_configuration(&self) -> ListRef<DataBucketFileLockConfigurationElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_lock_configuration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `lifecycle_rules` after provisioning.\nThe initial list of lifecycle rules for this bucket."]
    pub fn lifecycle_rules(&self) -> ListRef<DataBucketLifecycleRulesElRef> {
        ListRef::new(self.shared().clone(), format!("{}.lifecycle_rules", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of bucket options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `revision` after provisioning.\nBucket revision."]
    pub fn revision(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.revision", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBucketCorsRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_operations: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_origins: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cors_rule_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expose_headers: Option<ListField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_age_seconds: Option<PrimField<f64>>,
}

impl DataBucketCorsRulesEl {
    #[doc= "Set the field `allowed_headers`.\n"]
    pub fn set_allowed_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_operations`.\n"]
    pub fn set_allowed_operations(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_operations = Some(v.into());
        self
    }

    #[doc= "Set the field `allowed_origins`.\n"]
    pub fn set_allowed_origins(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.allowed_origins = Some(v.into());
        self
    }

    #[doc= "Set the field `cors_rule_name`.\n"]
    pub fn set_cors_rule_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.cors_rule_name = Some(v.into());
        self
    }

    #[doc= "Set the field `expose_headers`.\n"]
    pub fn set_expose_headers(mut self, v: impl Into<ListField<PrimField<String>>>) -> Self {
        self.expose_headers = Some(v.into());
        self
    }

    #[doc= "Set the field `max_age_seconds`.\n"]
    pub fn set_max_age_seconds(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.max_age_seconds = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketCorsRulesEl {
    type O = BlockAssignable<DataBucketCorsRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketCorsRulesEl {}

impl BuildDataBucketCorsRulesEl {
    pub fn build(self) -> DataBucketCorsRulesEl {
        DataBucketCorsRulesEl {
            allowed_headers: core::default::Default::default(),
            allowed_operations: core::default::Default::default(),
            allowed_origins: core::default::Default::default(),
            cors_rule_name: core::default::Default::default(),
            expose_headers: core::default::Default::default(),
            max_age_seconds: core::default::Default::default(),
        }
    }
}

pub struct DataBucketCorsRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketCorsRulesElRef {
    fn new(shared: StackShared, base: String) -> DataBucketCorsRulesElRef {
        DataBucketCorsRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketCorsRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `allowed_headers` after provisioning.\n"]
    pub fn allowed_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_operations` after provisioning.\n"]
    pub fn allowed_operations(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_operations", self.base))
    }

    #[doc= "Get a reference to the value of field `allowed_origins` after provisioning.\n"]
    pub fn allowed_origins(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.allowed_origins", self.base))
    }

    #[doc= "Get a reference to the value of field `cors_rule_name` after provisioning.\n"]
    pub fn cors_rule_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.cors_rule_name", self.base))
    }

    #[doc= "Get a reference to the value of field `expose_headers` after provisioning.\n"]
    pub fn expose_headers(&self) -> ListRef<PrimExpr<String>> {
        ListRef::new(self.shared().clone(), format!("{}.expose_headers", self.base))
    }

    #[doc= "Get a reference to the value of field `max_age_seconds` after provisioning.\n"]
    pub fn max_age_seconds(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.max_age_seconds", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketDefaultServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataBucketDefaultServerSideEncryptionEl {
    #[doc= "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.algorithm = Some(v.into());
        self
    }

    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketDefaultServerSideEncryptionEl {
    type O = BlockAssignable<DataBucketDefaultServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketDefaultServerSideEncryptionEl {}

impl BuildDataBucketDefaultServerSideEncryptionEl {
    pub fn build(self) -> DataBucketDefaultServerSideEncryptionEl {
        DataBucketDefaultServerSideEncryptionEl {
            algorithm: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct DataBucketDefaultServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketDefaultServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataBucketDefaultServerSideEncryptionElRef {
        DataBucketDefaultServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketDefaultServerSideEncryptionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit: Option<PrimField<String>>,
}

impl DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    #[doc= "Set the field `duration`.\n"]
    pub fn set_duration(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.duration = Some(v.into());
        self
    }

    #[doc= "Set the field `unit`.\n"]
    pub fn set_unit(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.unit = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    type O = BlockAssignable<DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {}

impl BuildDataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
    pub fn build(self) -> DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
        DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl {
            duration: core::default::Default::default(),
            unit: core::default::Default::default(),
        }
    }
}

pub struct DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn new(shared: StackShared, base: String) -> DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
        DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\n"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.base))
    }

    #[doc= "Get a reference to the value of field `unit` after provisioning.\n"]
    pub fn unit(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.unit", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketFileLockConfigurationElDefaultRetentionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<ListField<DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
}

impl DataBucketFileLockConfigurationElDefaultRetentionEl {
    #[doc= "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.mode = Some(v.into());
        self
    }

    #[doc= "Set the field `period`.\n"]
    pub fn set_period(
        mut self,
        v: impl Into<ListField<DataBucketFileLockConfigurationElDefaultRetentionElPeriodEl>>,
    ) -> Self {
        self.period = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketFileLockConfigurationElDefaultRetentionEl {
    type O = BlockAssignable<DataBucketFileLockConfigurationElDefaultRetentionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFileLockConfigurationElDefaultRetentionEl {}

impl BuildDataBucketFileLockConfigurationElDefaultRetentionEl {
    pub fn build(self) -> DataBucketFileLockConfigurationElDefaultRetentionEl {
        DataBucketFileLockConfigurationElDefaultRetentionEl {
            mode: core::default::Default::default(),
            period: core::default::Default::default(),
        }
    }
}

pub struct DataBucketFileLockConfigurationElDefaultRetentionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileLockConfigurationElDefaultRetentionElRef {
    fn new(shared: StackShared, base: String) -> DataBucketFileLockConfigurationElDefaultRetentionElRef {
        DataBucketFileLockConfigurationElDefaultRetentionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFileLockConfigurationElDefaultRetentionElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }

    #[doc= "Get a reference to the value of field `period` after provisioning.\n"]
    pub fn period(&self) -> ListRef<DataBucketFileLockConfigurationElDefaultRetentionElPeriodElRef> {
        ListRef::new(self.shared().clone(), format!("{}.period", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketFileLockConfigurationEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    default_retention: Option<ListField<DataBucketFileLockConfigurationElDefaultRetentionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_file_lock_enabled: Option<PrimField<bool>>,
}

impl DataBucketFileLockConfigurationEl {
    #[doc= "Set the field `default_retention`.\n"]
    pub fn set_default_retention(
        mut self,
        v: impl Into<ListField<DataBucketFileLockConfigurationElDefaultRetentionEl>>,
    ) -> Self {
        self.default_retention = Some(v.into());
        self
    }

    #[doc= "Set the field `is_file_lock_enabled`.\n"]
    pub fn set_is_file_lock_enabled(mut self, v: impl Into<PrimField<bool>>) -> Self {
        self.is_file_lock_enabled = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketFileLockConfigurationEl {
    type O = BlockAssignable<DataBucketFileLockConfigurationEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFileLockConfigurationEl {}

impl BuildDataBucketFileLockConfigurationEl {
    pub fn build(self) -> DataBucketFileLockConfigurationEl {
        DataBucketFileLockConfigurationEl {
            default_retention: core::default::Default::default(),
            is_file_lock_enabled: core::default::Default::default(),
        }
    }
}

pub struct DataBucketFileLockConfigurationElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileLockConfigurationElRef {
    fn new(shared: StackShared, base: String) -> DataBucketFileLockConfigurationElRef {
        DataBucketFileLockConfigurationElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFileLockConfigurationElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `default_retention` after provisioning.\n"]
    pub fn default_retention(&self) -> ListRef<DataBucketFileLockConfigurationElDefaultRetentionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.default_retention", self.base))
    }

    #[doc= "Get a reference to the value of field `is_file_lock_enabled` after provisioning.\n"]
    pub fn is_file_lock_enabled(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.is_file_lock_enabled", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketLifecycleRulesEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    days_from_hiding_to_deleting: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    days_from_uploading_to_hiding: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name_prefix: Option<PrimField<String>>,
}

impl DataBucketLifecycleRulesEl {
    #[doc= "Set the field `days_from_hiding_to_deleting`.\n"]
    pub fn set_days_from_hiding_to_deleting(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_from_hiding_to_deleting = Some(v.into());
        self
    }

    #[doc= "Set the field `days_from_uploading_to_hiding`.\n"]
    pub fn set_days_from_uploading_to_hiding(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.days_from_uploading_to_hiding = Some(v.into());
        self
    }

    #[doc= "Set the field `file_name_prefix`.\n"]
    pub fn set_file_name_prefix(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name_prefix = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketLifecycleRulesEl {
    type O = BlockAssignable<DataBucketLifecycleRulesEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketLifecycleRulesEl {}

impl BuildDataBucketLifecycleRulesEl {
    pub fn build(self) -> DataBucketLifecycleRulesEl {
        DataBucketLifecycleRulesEl {
            days_from_hiding_to_deleting: core::default::Default::default(),
            days_from_uploading_to_hiding: core::default::Default::default(),
            file_name_prefix: core::default::Default::default(),
        }
    }
}

pub struct DataBucketLifecycleRulesElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketLifecycleRulesElRef {
    fn new(shared: StackShared, base: String) -> DataBucketLifecycleRulesElRef {
        DataBucketLifecycleRulesElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketLifecycleRulesElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `days_from_hiding_to_deleting` after provisioning.\n"]
    pub fn days_from_hiding_to_deleting(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days_from_hiding_to_deleting", self.base))
    }

    #[doc= "Get a reference to the value of field `days_from_uploading_to_hiding` after provisioning.\n"]
    pub fn days_from_uploading_to_hiding(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.days_from_uploading_to_hiding", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name_prefix` after provisioning.\n"]
    pub fn file_name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name_prefix", self.base))
    }
}
