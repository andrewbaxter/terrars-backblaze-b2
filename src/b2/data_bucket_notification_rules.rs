#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataBucketNotificationRulesData_ {
    #[serde(skip_serializing_if = "std::vec::Vec::is_empty")]
    depends_on: std::vec::Vec<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    provider: std::option::Option<std::string::String>,
    #[serde(skip_serializing_if = "terrars::SerdeSkipDefault::is_default")]
    lifecycle: terrars::DatasourceLifecycle,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    for_each: std::option::Option<std::string::String>,
    bucket_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
}

struct DataBucketNotificationRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBucketNotificationRulesData_>,
}

#[derive(Clone)]
pub struct DataBucketNotificationRules(std::rc::Rc<DataBucketNotificationRules_>);

impl DataBucketNotificationRules {
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

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_rules` after provisioning.\nAn array of Event Notification Rules."]
    pub fn notification_rules(&self) -> terrars::ListRef<DataBucketNotificationRules_NotificationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_rules", self.extract_ref()))
    }
}

impl terrars::Referable for DataBucketNotificationRules {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBucketNotificationRules { }

impl terrars::ToListMappable for DataBucketNotificationRules {
    type O = terrars::ListRef<DataBucketNotificationRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBucketNotificationRules_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "b2_bucket_notification_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBucketNotificationRules {
    pub tf_id: std::string::String,
    #[doc = "The ID of the bucket."]
    pub bucket_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBucketNotificationRules {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBucketNotificationRules {
        let out = DataBucketNotificationRules(std::rc::Rc::new(DataBucketNotificationRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBucketNotificationRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                bucket_id: self.bucket_id,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBucketNotificationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketNotificationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBucketNotificationRulesRef {
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `notification_rules` after provisioning.\nAn array of Event Notification Rules."]
    pub fn notification_rules(&self) -> terrars::ListRef<DataBucketNotificationRules_NotificationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_rules", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    value: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `value`.\n"]
    pub fn set_value(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.value = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    type O =
        terrars::BlockAssignable<DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {}

impl BuildDataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    pub fn build(self) -> DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
        DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
            name: core::default::Default::default(),
            value: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
        DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\n"]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketNotificationRules_NotificationRules_TargetConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers: std
    ::option
    ::Option<terrars::ListField<DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hmac_sha256_signing_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    url: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBucketNotificationRules_NotificationRules_TargetConfiguration {
    #[doc = "Set the field `custom_headers`.\n"]
    pub fn set_custom_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::ListField<DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>,
                    >,
    ) -> Self {
        self.custom_headers = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `hmac_sha256_signing_secret`.\n"]
    pub fn set_hmac_sha256_signing_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hmac_sha256_signing_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_type`.\n"]
    pub fn set_target_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.target_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `url`.\n"]
    pub fn set_url(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.url = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBucketNotificationRules_NotificationRules_TargetConfiguration {
    type O = terrars::BlockAssignable<DataBucketNotificationRules_NotificationRules_TargetConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketNotificationRules_NotificationRules_TargetConfiguration {}

impl BuildDataBucketNotificationRules_NotificationRules_TargetConfiguration {
    pub fn build(self) -> DataBucketNotificationRules_NotificationRules_TargetConfiguration {
        DataBucketNotificationRules_NotificationRules_TargetConfiguration {
            custom_headers: core::default::Default::default(),
            hmac_sha256_signing_secret: core::default::Default::default(),
            target_type: core::default::Default::default(),
            url: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBucketNotificationRules_NotificationRules_TargetConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketNotificationRules_NotificationRules_TargetConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBucketNotificationRules_NotificationRules_TargetConfigurationRef {
        DataBucketNotificationRules_NotificationRules_TargetConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketNotificationRules_NotificationRules_TargetConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(
        &self,
    ) -> terrars::ListRef<DataBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers", self.base))
    }

    #[doc = "Get a reference to the value of field `hmac_sha256_signing_secret` after provisioning.\n"]
    pub fn hmac_sha256_signing_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hmac_sha256_signing_secret", self.base))
    }

    #[doc = "Get a reference to the value of field `target_type` after provisioning.\n"]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\n"]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketNotificationRules_NotificationRules {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    event_types: std::option::Option<terrars::ListField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_suspended: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    suspension_reason: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_configuration: std
    ::option
    ::Option<terrars::ListField<DataBucketNotificationRules_NotificationRules_TargetConfiguration>>,
}

impl DataBucketNotificationRules_NotificationRules {
    #[doc = "Set the field `event_types`.\n"]
    pub fn set_event_types(
        mut self,
        v: impl std::convert::Into<terrars::ListField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.event_types = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_enabled`.\n"]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `is_suspended`.\n"]
    pub fn set_is_suspended(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_suspended = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `name`.\n"]
    pub fn set_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `object_name_prefix`.\n"]
    pub fn set_object_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.object_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `suspension_reason`.\n"]
    pub fn set_suspension_reason(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.suspension_reason = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::ListField<DataBucketNotificationRules_NotificationRules_TargetConfiguration>>,
    ) -> Self {
        self.target_configuration = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBucketNotificationRules_NotificationRules {
    type O = terrars::BlockAssignable<DataBucketNotificationRules_NotificationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketNotificationRules_NotificationRules {}

impl BuildDataBucketNotificationRules_NotificationRules {
    pub fn build(self) -> DataBucketNotificationRules_NotificationRules {
        DataBucketNotificationRules_NotificationRules {
            event_types: core::default::Default::default(),
            is_enabled: core::default::Default::default(),
            is_suspended: core::default::Default::default(),
            name: core::default::Default::default(),
            object_name_prefix: core::default::Default::default(),
            suspension_reason: core::default::Default::default(),
            target_configuration: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBucketNotificationRules_NotificationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketNotificationRules_NotificationRulesRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBucketNotificationRules_NotificationRulesRef {
        DataBucketNotificationRules_NotificationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketNotificationRules_NotificationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `event_types` after provisioning.\n"]
    pub fn event_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }

    #[doc = "Get a reference to the value of field `is_enabled` after provisioning.\n"]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }

    #[doc = "Get a reference to the value of field `is_suspended` after provisioning.\n"]
    pub fn is_suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_suspended", self.base))
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\n"]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `object_name_prefix` after provisioning.\n"]
    pub fn object_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_name_prefix", self.base))
    }

    #[doc = "Get a reference to the value of field `suspension_reason` after provisioning.\n"]
    pub fn suspension_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspension_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> terrars::ListRef<DataBucketNotificationRules_NotificationRules_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.base))
    }
}
