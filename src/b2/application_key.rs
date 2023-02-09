use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct ApplicationKeyData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id: Option<PrimField<String>>,
    capabilities: SetField<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    key_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name_prefix: Option<PrimField<String>>,
}

struct ApplicationKey_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<ApplicationKeyData>,
}

#[derive(Clone)]
pub struct ApplicationKey(Rc<ApplicationKey_>);

impl ApplicationKey {
    fn shared(&self) -> &StackShared {
        &self.0.shared
    }

    pub fn depends_on(self, dep: &impl Dependable) -> Self {
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

    #[doc= "Set the field `bucket_id`.\nWhen present, restricts access to one bucket."]
    pub fn set_bucket_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().bucket_id = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `name_prefix`.\nWhen present, restricts access to files whose names start with the prefix."]
    pub fn set_name_prefix(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().name_prefix = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `application_key` after provisioning.\nThe key."]
    pub fn application_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application_key_id` after provisioning.\nThe ID of the newly created key."]
    pub fn application_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nWhen present, restricts access to one bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `capabilities` after provisioning.\nA set of strings, each one naming a capability the key has."]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_name` after provisioning.\nThe name of the key."]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nWhen present, restricts access to files whose names start with the prefix."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of application key options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}

impl Resource for ApplicationKey {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for ApplicationKey {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for ApplicationKey {
    type O = ListRef<ApplicationKeyRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl Resource_ for ApplicationKey_ {
    fn extract_resource_type(&self) -> String {
        "b2_application_key".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildApplicationKey {
    pub tf_id: String,
    #[doc= "A set of strings, each one naming a capability the key has."]
    pub capabilities: SetField<PrimField<String>>,
    #[doc= "The name of the key."]
    pub key_name: PrimField<String>,
}

impl BuildApplicationKey {
    pub fn build(self, stack: &mut Stack) -> ApplicationKey {
        let out = ApplicationKey(Rc::new(ApplicationKey_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(ApplicationKeyData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_id: core::default::Default::default(),
                capabilities: self.capabilities,
                id: core::default::Default::default(),
                key_name: self.key_name,
                name_prefix: core::default::Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct ApplicationKeyRef {
    shared: StackShared,
    base: String,
}

impl Ref for ApplicationKeyRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl ApplicationKeyRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `application_key` after provisioning.\nThe key."]
    pub fn application_key(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_key", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `application_key_id` after provisioning.\nThe ID of the newly created key."]
    pub fn application_key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.application_key_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nWhen present, restricts access to one bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `capabilities` after provisioning.\nA set of strings, each one naming a capability the key has."]
    pub fn capabilities(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.capabilities", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `key_name` after provisioning.\nThe name of the key."]
    pub fn key_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `name_prefix` after provisioning.\nWhen present, restricts access to files whose names start with the prefix."]
    pub fn name_prefix(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.name_prefix", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `options` after provisioning.\nList of application key options."]
    pub fn options(&self) -> SetRef<PrimExpr<String>> {
        SetRef::new(self.shared().clone(), format!("{}.options", self.extract_ref()))
    }
}
