use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataBucketFileData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_id: PrimField<String>,
    file_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_versions: Option<PrimField<bool>>,
}

struct DataBucketFile_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBucketFileData>,
}

#[derive(Clone)]
pub struct DataBucketFile(Rc<DataBucketFile_>);

impl DataBucketFile {
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

    #[doc= "Set the field `show_versions`.\nShow all file versions."]
    pub fn set_show_versions(self, v: impl Into<PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_versions = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe file name."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_versions` after provisioning.\nFile versions."]
    pub fn file_versions(&self) -> ListRef<DataBucketFileFileVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_versions` after provisioning.\nShow all file versions."]
    pub fn show_versions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_versions", self.extract_ref()))
    }
}

impl Datasource for DataBucketFile {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataBucketFile {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataBucketFile {
    type O = ListRef<DataBucketFileRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataBucketFile_ {
    fn extract_datasource_type(&self) -> String {
        "b2_bucket_file".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBucketFile {
    pub tf_id: String,
    #[doc= "The ID of the bucket."]
    pub bucket_id: PrimField<String>,
    #[doc= "The file name."]
    pub file_name: PrimField<String>,
}

impl BuildDataBucketFile {
    pub fn build(self, stack: &mut Stack) -> DataBucketFile {
        let out = DataBucketFile(Rc::new(DataBucketFile_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBucketFileData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket_id: self.bucket_id,
                file_name: self.file_name,
                id: core::default::Default::default(),
                show_versions: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBucketFileRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBucketFileRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    fn extract_ref(&self) -> String {
        self.base.clone()
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe file name."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_versions` after provisioning.\nFile versions."]
    pub fn file_versions(&self) -> ListRef<DataBucketFileFileVersionsElRef> {
        ListRef::new(self.shared().clone(), format!("{}.file_versions", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `show_versions` after provisioning.\nShow all file versions."]
    pub fn show_versions(&self) -> PrimExpr<bool> {
        PrimExpr::new(self.shared().clone(), format!("{}.show_versions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBucketFileFileVersionsElServerSideEncryptionEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    algorithm: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<PrimField<String>>,
}

impl DataBucketFileFileVersionsElServerSideEncryptionEl {
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

impl ToListMappable for DataBucketFileFileVersionsElServerSideEncryptionEl {
    type O = BlockAssignable<DataBucketFileFileVersionsElServerSideEncryptionEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFileFileVersionsElServerSideEncryptionEl {}

impl BuildDataBucketFileFileVersionsElServerSideEncryptionEl {
    pub fn build(self) -> DataBucketFileFileVersionsElServerSideEncryptionEl {
        DataBucketFileFileVersionsElServerSideEncryptionEl {
            algorithm: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

pub struct DataBucketFileFileVersionsElServerSideEncryptionElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileFileVersionsElServerSideEncryptionElRef {
    fn new(shared: StackShared, base: String) -> DataBucketFileFileVersionsElServerSideEncryptionElRef {
        DataBucketFileFileVersionsElServerSideEncryptionElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFileFileVersionsElServerSideEncryptionElRef {
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
pub struct DataBucketFileFileVersionsEl {
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bucket_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_md5: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_sha1: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content_type: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_id: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_info: Option<RecField<PrimField<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    file_name: Option<PrimField<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server_side_encryption: Option<ListField<DataBucketFileFileVersionsElServerSideEncryptionEl>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PrimField<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upload_timestamp: Option<PrimField<f64>>,
}

impl DataBucketFileFileVersionsEl {
    #[doc= "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.action = Some(v.into());
        self
    }

    #[doc= "Set the field `bucket_id`.\n"]
    pub fn set_bucket_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.bucket_id = Some(v.into());
        self
    }

    #[doc= "Set the field `content_md5`.\n"]
    pub fn set_content_md5(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_md5 = Some(v.into());
        self
    }

    #[doc= "Set the field `content_sha1`.\n"]
    pub fn set_content_sha1(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_sha1 = Some(v.into());
        self
    }

    #[doc= "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.content_type = Some(v.into());
        self
    }

    #[doc= "Set the field `file_id`.\n"]
    pub fn set_file_id(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_id = Some(v.into());
        self
    }

    #[doc= "Set the field `file_info`.\n"]
    pub fn set_file_info(mut self, v: impl Into<RecField<PrimField<String>>>) -> Self {
        self.file_info = Some(v.into());
        self
    }

    #[doc= "Set the field `file_name`.\n"]
    pub fn set_file_name(mut self, v: impl Into<PrimField<String>>) -> Self {
        self.file_name = Some(v.into());
        self
    }

    #[doc= "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        mut self,
        v: impl Into<ListField<DataBucketFileFileVersionsElServerSideEncryptionEl>>,
    ) -> Self {
        self.server_side_encryption = Some(v.into());
        self
    }

    #[doc= "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.size = Some(v.into());
        self
    }

    #[doc= "Set the field `upload_timestamp`.\n"]
    pub fn set_upload_timestamp(mut self, v: impl Into<PrimField<f64>>) -> Self {
        self.upload_timestamp = Some(v.into());
        self
    }
}

impl ToListMappable for DataBucketFileFileVersionsEl {
    type O = BlockAssignable<DataBucketFileFileVersionsEl>;

    fn do_map(self, base: String) -> Self::O {
        BlockAssignable::Dynamic(DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFileFileVersionsEl {}

impl BuildDataBucketFileFileVersionsEl {
    pub fn build(self) -> DataBucketFileFileVersionsEl {
        DataBucketFileFileVersionsEl {
            action: core::default::Default::default(),
            bucket_id: core::default::Default::default(),
            content_md5: core::default::Default::default(),
            content_sha1: core::default::Default::default(),
            content_type: core::default::Default::default(),
            file_id: core::default::Default::default(),
            file_info: core::default::Default::default(),
            file_name: core::default::Default::default(),
            server_side_encryption: core::default::Default::default(),
            size: core::default::Default::default(),
            upload_timestamp: core::default::Default::default(),
        }
    }
}

pub struct DataBucketFileFileVersionsElRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileFileVersionsElRef {
    fn new(shared: StackShared, base: String) -> DataBucketFileFileVersionsElRef {
        DataBucketFileFileVersionsElRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFileFileVersionsElRef {
    fn shared(&self) -> &StackShared {
        &self.shared
    }

    #[doc= "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\n"]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.base))
    }

    #[doc= "Get a reference to the value of field `content_md5` after provisioning.\n"]
    pub fn content_md5(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_md5", self.base))
    }

    #[doc= "Get a reference to the value of field `content_sha1` after provisioning.\n"]
    pub fn content_sha1(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_sha1", self.base))
    }

    #[doc= "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc= "Get a reference to the value of field `file_id` after provisioning.\n"]
    pub fn file_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_id", self.base))
    }

    #[doc= "Get a reference to the value of field `file_info` after provisioning.\n"]
    pub fn file_info(&self) -> RecRef<PrimExpr<String>> {
        RecRef::new(self.shared().clone(), format!("{}.file_info", self.base))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\n"]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc= "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> ListRef<DataBucketFileFileVersionsElServerSideEncryptionElRef> {
        ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.base))
    }

    #[doc= "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc= "Get a reference to the value of field `upload_timestamp` after provisioning.\n"]
    pub fn upload_timestamp(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.upload_timestamp", self.base))
    }
}
