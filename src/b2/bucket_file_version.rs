use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct BucketFileVersionData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    lifecycle: ResourceLifecycle,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_info: Option<RecField<PrimField<String>>>,
    file_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    source: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<Vec<BucketFileVersionServerSideEncryptionEl>>,
    dynamic: BucketFileVersionDynamic,
}

struct BucketFileVersion_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<BucketFileVersionData>,
}

#[derive(Clone)]
pub struct BucketFileVersion(Rc<BucketFileVersion_>);

impl BucketFileVersion {
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

    #[doc= "Set the field `content_type`.\nContent type. If not set, it will be set based on the file extension."]
    pub fn set_content_type(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `file_info`.\nThe custom information that is uploaded with the file."]
    pub fn set_file_info(self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.0.data.borrow_mut().file_info = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        self,
        v: impl Into<BlockAssignable<BucketFileVersionServerSideEncryptionEl>>,
    ) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.0.data.borrow_mut().server_side_encryption = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.0.data.borrow_mut().dynamic.server_side_encryption = Some(d);
            },
        }
        self
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nOne of 'start', 'upload', 'hide', 'folder', or other values added in the future."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_md5` after provisioning.\nMD5 sum of the content."]
    pub fn content_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_md5", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha1` after provisioning.\nSHA1 hash of the content."]
    pub fn content_sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nContent type. If not set, it will be set based on the file extension."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_id` after provisioning.\nThe unique identifier for this version of this file."]
    pub fn file_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_info` after provisioning.\nThe custom information that is uploaded with the file."]
    pub fn file_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.file_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe name of the B2 file."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nThe file size."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nPath to the local file."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upload_timestamp` after provisioning.\nThis is a UTC time when this file was uploaded."]
    pub fn upload_timestamp(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<BucketFileVersionServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }
}

impl Resource for BucketFileVersion {
    fn extract_ref(&self) -> String {
        format!("{}.{}", self.0.extract_resource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for BucketFileVersion {
    fn extract_ref(&self) -> String {
        Resource::extract_ref(self)
    }
}

impl ToListMappable for BucketFileVersion {
    type O = ListRef<BucketFileVersionRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Resource::extract_ref(self))
    }
}

impl Resource_ for BucketFileVersion_ {
    fn extract_resource_type(&self) -> String {
        "b2_bucket_file_version".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildBucketFileVersion {
    pub tf_id: String,
    #[doc= "The ID of the bucket."]
    pub bucket_id: PrimField<String>,
    #[doc= "The name of the B2 file."]
    pub file_name: PrimField<String>,
    #[doc= "Path to the local file."]
    pub source: PrimField<String>,
}

impl BuildBucketFileVersion {
    pub fn build(self, stack: &mut Stack) -> BucketFileVersion {
        let out = BucketFileVersion(Rc::new(BucketFileVersion_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(BucketFileVersionData {
                depends_on: core::default::Default::default(),
                provider: None,
                lifecycle: core::default::Default::default(),
                for_each: None,
                bucket_id: self.bucket_id,
                content_type: core::default::Default::default(),
                file_info: core::default::Default::default(),
                file_name: self.file_name,
                id: core::default::Default::default(),
                source: self.source,
                server_side_encryption: core::default::Default::default(),
                dynamic: Default::default(),
            }),
        }));
        stack.add_resource(out.0.clone());
        out
    }
}

pub struct BucketFileVersionRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileVersionRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl BucketFileVersionRef {
    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\nOne of 'start', 'upload', 'hide', 'folder', or other values added in the future."]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_md5` after provisioning.\nMD5 sum of the content."]
    pub fn content_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_md5", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_sha1` after provisioning.\nSHA1 hash of the content."]
    pub fn content_sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha1", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\nContent type. If not set, it will be set based on the file extension."]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_id` after provisioning.\nThe unique identifier for this version of this file."]
    pub fn file_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_info` after provisioning.\nThe custom information that is uploaded with the file."]
    pub fn file_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.file_info", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe name of the B2 file."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\nThe file size."]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `source` after provisioning.\nPath to the local file."]
    pub fn source(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.source", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `upload_timestamp` after provisioning.\nThis is a UTC time when this file was uploaded."]
    pub fn upload_timestamp(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_timestamp", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<BucketFileVersionServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct BucketFileVersionServerSideEncryptionElKeyEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    key_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secret_b64: Option<PrimField<String>>,
}

impl BucketFileVersionServerSideEncryptionElKeyEl {
    #[doc= "Set the field `key_id`.\nKey identifier stored in file info metadata"]
    pub fn set_key_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.key_id = Some(v.into());
        self
    }

    #[doc= "Set the field `secret_b64`.\nSecret key value, in standard Base 64 encoding (RFC 4648)"]
    pub fn set_secret_b64(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.secret_b64 = Some(v.into());
        self
    }
}

impl ToListMappable for BucketFileVersionServerSideEncryptionElKeyEl {
    type O = BlockAssignable<BucketFileVersionServerSideEncryptionElKeyEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileVersionServerSideEncryptionElKeyEl {}

impl BuildBucketFileVersionServerSideEncryptionElKeyEl {
    pub fn build(self) -> BucketFileVersionServerSideEncryptionElKeyEl {
        BucketFileVersionServerSideEncryptionElKeyEl {
            key_id: core::default::Default::default(),
            secret_b64: core::default::Default::default(),
        }
    }
}

pub struct BucketFileVersionServerSideEncryptionElKeyElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileVersionServerSideEncryptionElKeyElRef {
    fn new(shared: StackShared, base: String) -> BucketFileVersionServerSideEncryptionElKeyElRef {
        BucketFileVersionServerSideEncryptionElKeyElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileVersionServerSideEncryptionElKeyElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `key_id` after provisioning.\nKey identifier stored in file info metadata"]
    pub fn key_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.key_id", self.base))
    }

    #[doc= "Get a reference to the value of field `secret_b64` after provisioning.\nSecret key value, in standard Base 64 encoding (RFC 4648)"]
    pub fn secret_b64(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.secret_b64", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileVersionServerSideEncryptionElDynamic {
    key: Option<DynamicBlock<BucketFileVersionServerSideEncryptionElKeyEl>>,
}

#[derive(Serialize)]
pub struct BucketFileVersionServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Vec<BucketFileVersionServerSideEncryptionElKeyEl>>,
    dynamic: BucketFileVersionServerSideEncryptionElDynamic,
}

impl BucketFileVersionServerSideEncryptionEl {
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

    #[doc= "Set the field `key`.\n"]
    pub fn set_key(mut self, v: impl Into<BlockAssignable<BucketFileVersionServerSideEncryptionElKeyEl>>) -> Self {
        match v.into() {
            BlockAssignable::Literal(v) => {
                self.key = Some(v);
            },
            BlockAssignable::Dynamic(d) => {
                self.dynamic.key = Some(d);
            },
        }
        self
    }
}

impl ToListMappable for BucketFileVersionServerSideEncryptionEl {
    type O = BlockAssignable<BucketFileVersionServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildBucketFileVersionServerSideEncryptionEl {}

impl BuildBucketFileVersionServerSideEncryptionEl {
    pub fn build(self) -> BucketFileVersionServerSideEncryptionEl {
        BucketFileVersionServerSideEncryptionEl {
            algorithm: core::default::Default::default(),
            mode: core::default::Default::default(),
            key: core::default::Default::default(),
            dynamic: Default::default(),
        }
    }
}

pub struct BucketFileVersionServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for BucketFileVersionServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> BucketFileVersionServerSideEncryptionElRef {
        BucketFileVersionServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl BucketFileVersionServerSideEncryptionElRef {
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

    #[doc= "Get a reference to the value of field `key` after provisioning.\n"]
    pub fn key(&self) -> ListRef<BucketFileVersionServerSideEncryptionElKeyElRef> {
        ListRef::new(self.shared().clone(), format!("{}.key", self.base))
    }
}

#[derive(Serialize, Default)]
struct BucketFileVersionDynamic {
    server_side_encryption: Option<DynamicBlock<BucketFileVersionServerSideEncryptionEl>>,
}
