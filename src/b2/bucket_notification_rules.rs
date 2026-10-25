#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Resource_ as _,
    Ref as _,
};
use super::provider::ProviderB2;

#[derive(Serialize)]
struct BucketNotificationRulesData_ {
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
    bucket_id: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    notification_rules: std::option::Option<std::vec::Vec<BucketNotificationRules_NotificationRules>>,
    dynamic: BucketNotificationRulesDynamic,
}

struct BucketNotificationRules_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<BucketNotificationRulesData_>,
}

#[derive(Clone)]
pub struct BucketNotificationRules(std::rc::Rc<BucketNotificationRules_>);

impl BucketNotificationRules {
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

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `notification_rules`.\n"]
    pub fn set_notification_rules(
        self,
        v: impl std::convert::Into<terrars::BlockAssignable<BucketNotificationRules_NotificationRules>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().notification_rules = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.notification_rules = std::option::Option::Some(d);
            },
        }
        self
    }

    #[doc =
        "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket. **Modifying this attribute will force creation of a new resource.**"]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_rules` after provisioning.\n"]
    pub fn notification_rules(&self) -> terrars::ListRef<BucketNotificationRules_NotificationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_rules", self.extract_ref()))
    }
}

impl terrars::Referable for BucketNotificationRules {
    fn extract_ref(&self) -> std::string::String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Resource for BucketNotificationRules { }

impl terrars::ToListMappable for BucketNotificationRules {
    type O = terrars::ListRef<BucketNotificationRulesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Resource_ for BucketNotificationRules_ {
    fn extract_resource_type(&self) -> std::string::String {
        "b2_bucket_notification_rules".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBucketNotificationRules {
    pub tf_id: std::string::String,
    #[doc = "The ID of the bucket. **Modifying this attribute will force creation of a new resource.**"]
    pub bucket_id: terrars::PrimField<std::string::String>,
}

impl BuildBucketNotificationRules {
    pub fn build(self, stack: &mut terrars::Stack) -> BucketNotificationRules {
        let out = BucketNotificationRules(std::rc::Rc::new(BucketNotificationRules_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(BucketNotificationRulesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                provisioner: core::default::Default::default(),
                bucket_id: self.bucket_id,
                id: core::default::Default::default(),
                notification_rules: core::default::Default::default(),
                dynamic: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BucketNotificationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketNotificationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BucketNotificationRulesRef {
    fn extract_ref(&self) -> std::string::String {
        self.base.clone()
    }

    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket. **Modifying this attribute will force creation of a new resource.**"]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `notification_rules` after provisioning.\n"]
    pub fn notification_rules(&self) -> terrars::ListRef<BucketNotificationRules_NotificationRulesRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.notification_rules", self.extract_ref()))
    }
}

#[derive(Serialize, Default)]
struct BucketNotificationRulesDynamic {
    notification_rules: std::option::Option<terrars::DynamicBlock<BucketNotificationRules_NotificationRules>>,
}

#[derive(Serialize)]
pub struct BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    name: terrars::PrimField<std::string::String>,
    value: terrars::PrimField<std::string::String>,
}

impl BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders { }

impl terrars::ToListMappable for BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    type O = terrars::BlockAssignable<BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    #[doc = "Name of the header."]
    pub name: terrars::PrimField<std::string::String>,
    #[doc = "Value of the header."]
    pub value: terrars::PrimField<std::string::String>,
}

impl BuildBucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
    pub fn build(self) -> BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
        BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders {
            name: self.name,
            value: self.value,
        }
    }
}

#[allow(dead_code)]
pub struct BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
        BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `name` after provisioning.\nName of the header."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc = "Get a reference to the value of field `value` after provisioning.\nValue of the header."]
    pub fn value(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.value", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketNotificationRules_NotificationRules_TargetConfigurationDynamic {
    custom_headers: std
    ::option
    ::Option<terrars::DynamicBlock<BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>>,
}

#[derive(Serialize)]
pub struct BucketNotificationRules_NotificationRules_TargetConfiguration {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    hmac_sha256_signing_secret: std::option::Option<terrars::PrimField<std::string::String>>,
    target_type: terrars::PrimField<std::string::String>,
    url: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    custom_headers: std
    ::option
    ::Option<std::vec::Vec<BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>>,
    dynamic: BucketNotificationRules_NotificationRules_TargetConfigurationDynamic,
}

impl BucketNotificationRules_NotificationRules_TargetConfiguration {
    #[doc =
        "Set the field `hmac_sha256_signing_secret`.\nThe signing secret for use in verifying the X-Bz-Event-Notification-Signature."]
    pub fn set_hmac_sha256_signing_secret(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.hmac_sha256_signing_secret = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `custom_headers`.\n"]
    pub fn set_custom_headers(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<
                        terrars
                        ::BlockAssignable<BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeaders>,
                    >,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.custom_headers = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.custom_headers = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BucketNotificationRules_NotificationRules_TargetConfiguration {
    type O = terrars::BlockAssignable<BucketNotificationRules_NotificationRules_TargetConfiguration>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketNotificationRules_NotificationRules_TargetConfiguration {
    #[doc = "The type of the target configuration, currently \"webhook\" only."]
    pub target_type: terrars::PrimField<std::string::String>,
    #[doc = "The URL for the webhook."]
    pub url: terrars::PrimField<std::string::String>,
}

impl BuildBucketNotificationRules_NotificationRules_TargetConfiguration {
    pub fn build(self) -> BucketNotificationRules_NotificationRules_TargetConfiguration {
        BucketNotificationRules_NotificationRules_TargetConfiguration {
            hmac_sha256_signing_secret: core::default::Default::default(),
            target_type: self.target_type,
            url: self.url,
            custom_headers: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BucketNotificationRules_NotificationRules_TargetConfigurationRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketNotificationRules_NotificationRules_TargetConfigurationRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> BucketNotificationRules_NotificationRules_TargetConfigurationRef {
        BucketNotificationRules_NotificationRules_TargetConfigurationRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketNotificationRules_NotificationRules_TargetConfigurationRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `hmac_sha256_signing_secret` after provisioning.\nThe signing secret for use in verifying the X-Bz-Event-Notification-Signature."]
    pub fn hmac_sha256_signing_secret(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.hmac_sha256_signing_secret", self.base))
    }

    #[doc =
        "Get a reference to the value of field `target_type` after provisioning.\nThe type of the target configuration, currently \"webhook\" only."]
    pub fn target_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.target_type", self.base))
    }

    #[doc = "Get a reference to the value of field `url` after provisioning.\nThe URL for the webhook."]
    pub fn url(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.url", self.base))
    }

    #[doc = "Get a reference to the value of field `custom_headers` after provisioning.\n"]
    pub fn custom_headers(
        &self,
    ) -> terrars::ListRef<BucketNotificationRules_NotificationRules_TargetConfiguration_CustomHeadersRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.custom_headers", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketNotificationRules_NotificationRulesDynamic {
    target_configuration: std
    ::option
    ::Option<terrars::DynamicBlock<BucketNotificationRules_NotificationRules_TargetConfiguration>>,
}

#[derive(Serialize)]
pub struct BucketNotificationRules_NotificationRules {
    event_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    is_enabled: std::option::Option<terrars::PrimField<bool>>,
    name: terrars::PrimField<std::string::String>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    object_name_prefix: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    target_configuration: std
    ::option
    ::Option<std::vec::Vec<BucketNotificationRules_NotificationRules_TargetConfiguration>>,
    dynamic: BucketNotificationRules_NotificationRulesDynamic,
}

impl BucketNotificationRules_NotificationRules {
    #[doc = "Set the field `is_enabled`.\nWhether the event notification rule is enabled. Defaults to `true`."]
    pub fn set_is_enabled(mut self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.is_enabled = std::option::Option::Some(v.into());
        self
    }

    #[doc =
        "Set the field `object_name_prefix`.\nSpecifies which object(s) in the bucket the event notification rule applies to."]
    pub fn set_object_name_prefix(
        mut self,
        v: impl std::convert::Into<terrars::PrimField<std::string::String>>,
    ) -> Self {
        self.object_name_prefix = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `target_configuration`.\n"]
    pub fn set_target_configuration(
        mut self,
        v:
            impl

                    std
                    ::convert
                    ::Into<terrars::BlockAssignable<BucketNotificationRules_NotificationRules_TargetConfiguration>>,
    ) -> Self {
        match v.into() {
            terrars::BlockAssignable::Literal(v) => {
                self.target_configuration = std::option::Option::Some(v);
            },
            terrars::BlockAssignable::Dynamic(d) => {
                self.dynamic.target_configuration = std::option::Option::Some(d);
            },
        }
        self
    }
}

impl terrars::ToListMappable for BucketNotificationRules_NotificationRules {
    type O = terrars::BlockAssignable<BucketNotificationRules_NotificationRules>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketNotificationRules_NotificationRules {
    #[doc = "The list of event types for the event notification rule."]
    pub event_types: terrars::ListField<terrars::PrimField<std::string::String>>,
    #[doc = "A name for the event notification rule. The name must be unique among the bucket's notification rules."]
    pub name: terrars::PrimField<std::string::String>,
}

impl BuildBucketNotificationRules_NotificationRules {
    pub fn build(self) -> BucketNotificationRules_NotificationRules {
        BucketNotificationRules_NotificationRules {
            event_types: self.event_types,
            is_enabled: core::default::Default::default(),
            name: self.name,
            object_name_prefix: core::default::Default::default(),
            target_configuration: core::default::Default::default(),
            dynamic: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct BucketNotificationRules_NotificationRulesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for BucketNotificationRules_NotificationRulesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> BucketNotificationRules_NotificationRulesRef {
        BucketNotificationRules_NotificationRulesRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketNotificationRules_NotificationRulesRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc =
        "Get a reference to the value of field `event_types` after provisioning.\nThe list of event types for the event notification rule."]
    pub fn event_types(&self) -> terrars::ListRef<terrars::PrimExpr<std::string::String>> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.event_types", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_enabled` after provisioning.\nWhether the event notification rule is enabled. Defaults to `true`."]
    pub fn is_enabled(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_enabled", self.base))
    }

    #[doc =
        "Get a reference to the value of field `is_suspended` after provisioning.\nWhether the event notification rule is suspended."]
    pub fn is_suspended(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.is_suspended", self.base))
    }

    #[doc =
        "Get a reference to the value of field `name` after provisioning.\nA name for the event notification rule. The name must be unique among the bucket's notification rules."]
    pub fn name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.name", self.base))
    }

    #[doc =
        "Get a reference to the value of field `object_name_prefix` after provisioning.\nSpecifies which object(s) in the bucket the event notification rule applies to."]
    pub fn object_name_prefix(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.object_name_prefix", self.base))
    }

    #[doc =
        "Get a reference to the value of field `suspension_reason` after provisioning.\nA brief description of why the event notification rule was suspended."]
    pub fn suspension_reason(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.suspension_reason", self.base))
    }

    #[doc = "Get a reference to the value of field `target_configuration` after provisioning.\n"]
    pub fn target_configuration(
        &self,
    ) -> terrars::ListRef<BucketNotificationRules_NotificationRules_TargetConfigurationRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.target_configuration", self.base))
    }
}
