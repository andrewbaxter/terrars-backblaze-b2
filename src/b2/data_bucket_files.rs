#![allow(non_camel_case_types)]

use serde::Serialize;
use terrars::{
    Referable as _,
    Datasource_ as _,
    Ref as _,
};
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataBucketFilesData_ {
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
    folder_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    recursive: std::option::Option<terrars::PrimField<bool>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    show_versions: std::option::Option<terrars::PrimField<bool>>,
}

struct DataBucketFiles_ {
    shared: terrars::StackShared,
    tf_id: std::string::String,
    data: std::cell::RefCell<DataBucketFilesData_>,
}

#[derive(Clone)]
pub struct DataBucketFiles(std::rc::Rc<DataBucketFiles_>);

impl DataBucketFiles {
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

    #[doc = "Set the field `folder_name`.\nThe folder name (B2 file name prefix)."]
    pub fn set_folder_name(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().folder_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `id`.\n"]
    pub fn set_id(self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.0.data.borrow_mut().id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `recursive`.\nRecursive mode."]
    pub fn set_recursive(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().recursive = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `show_versions`.\nShow all file versions."]
    pub fn set_show_versions(self, v: impl std::convert::Into<terrars::PrimField<bool>>) -> Self {
        self.0.data.borrow_mut().show_versions = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `file_versions` after provisioning.\nFile versions in the folder."]
    pub fn file_versions(&self) -> terrars::ListRef<DataBucketFiles_FileVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder_name` after provisioning.\nThe folder name (B2 file name prefix)."]
    pub fn folder_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recursive` after provisioning.\nRecursive mode."]
    pub fn recursive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_versions` after provisioning.\nShow all file versions."]
    pub fn show_versions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_versions", self.extract_ref()))
    }
}

impl terrars::Referable for DataBucketFiles {
    fn extract_ref(&self) -> std::string::String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl terrars::Datasource for DataBucketFiles { }

impl terrars::ToListMappable for DataBucketFiles {
    type O = terrars::ListRef<DataBucketFilesRef>;

    fn do_map(self, base: std::string::String) -> Self::O {
        self.0.data.borrow_mut().for_each = std::option::Option::Some(format!("${{{}}}", base));
        terrars::ListRef::new(self.0.shared.clone(), self.extract_ref())
    }
}

impl terrars::Datasource_ for DataBucketFiles_ {
    fn extract_datasource_type(&self) -> std::string::String {
        "b2_bucket_files".into()
    }

    fn extract_tf_id(&self) -> std::string::String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBucketFiles {
    pub tf_id: std::string::String,
    #[doc = "The ID of the bucket."]
    pub bucket_id: terrars::PrimField<std::string::String>,
}

impl BuildDataBucketFiles {
    pub fn build(self, stack: &mut terrars::Stack) -> DataBucketFiles {
        let out = DataBucketFiles(std::rc::Rc::new(DataBucketFiles_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: std::cell::RefCell::new(DataBucketFilesData_ {
                depends_on: core::default::Default::default(),
                provider: std::option::Option::None,
                lifecycle: core::default::Default::default(),
                for_each: std::option::Option::None,
                bucket_id: self.bucket_id,
                folder_name: core::default::Default::default(),
                id: core::default::Default::default(),
                recursive: core::default::Default::default(),
                show_versions: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBucketFilesRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketFilesRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBucketFilesRef {
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

    #[doc = "Get a reference to the value of field `file_versions` after provisioning.\nFile versions in the folder."]
    pub fn file_versions(&self) -> terrars::ListRef<DataBucketFiles_FileVersionsRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.file_versions", self.extract_ref()))
    }

    #[doc =
        "Get a reference to the value of field `folder_name` after provisioning.\nThe folder name (B2 file name prefix)."]
    pub fn folder_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.folder_name", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `recursive` after provisioning.\nRecursive mode."]
    pub fn recursive(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.recursive", self.extract_ref()))
    }

    #[doc = "Get a reference to the value of field `show_versions` after provisioning.\nShow all file versions."]
    pub fn show_versions(&self) -> terrars::PrimExpr<bool> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.show_versions", self.extract_ref()))
    }
}

#[derive(Serialize)]
pub struct DataBucketFiles_FileVersions_ServerSideEncryption {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    algorithm: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    mode: std::option::Option<terrars::PrimField<std::string::String>>,
}

impl DataBucketFiles_FileVersions_ServerSideEncryption {
    #[doc = "Set the field `algorithm`.\n"]
    pub fn set_algorithm(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.algorithm = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `mode`.\n"]
    pub fn set_mode(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.mode = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBucketFiles_FileVersions_ServerSideEncryption {
    type O = terrars::BlockAssignable<DataBucketFiles_FileVersions_ServerSideEncryption>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFiles_FileVersions_ServerSideEncryption {}

impl BuildDataBucketFiles_FileVersions_ServerSideEncryption {
    pub fn build(self) -> DataBucketFiles_FileVersions_ServerSideEncryption {
        DataBucketFiles_FileVersions_ServerSideEncryption {
            algorithm: core::default::Default::default(),
            mode: core::default::Default::default(),
        }
    }
}

#[allow(dead_code)]
pub struct DataBucketFiles_FileVersions_ServerSideEncryptionRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketFiles_FileVersions_ServerSideEncryptionRef {
    fn new(
        shared: terrars::StackShared,
        base: std::string::String,
    ) -> DataBucketFiles_FileVersions_ServerSideEncryptionRef {
        DataBucketFiles_FileVersions_ServerSideEncryptionRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFiles_FileVersions_ServerSideEncryptionRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `algorithm` after provisioning.\n"]
    pub fn algorithm(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.algorithm", self.base))
    }

    #[doc = "Get a reference to the value of field `mode` after provisioning.\n"]
    pub fn mode(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.mode", self.base))
    }
}

#[derive(Serialize)]
pub struct DataBucketFiles_FileVersions {
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    action: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    bucket_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_md5: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_sha1: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    content_type: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_id: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_info: std::option::Option<terrars::RecField<terrars::PrimField<std::string::String>>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    file_name: std::option::Option<terrars::PrimField<std::string::String>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    server_side_encryption: std::option::Option<terrars::ListField<DataBucketFiles_FileVersions_ServerSideEncryption>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    size: std::option::Option<terrars::PrimField<f64>>,
    #[serde(skip_serializing_if = "std::option::Option::is_none")]
    upload_timestamp: std::option::Option<terrars::PrimField<f64>>,
}

impl DataBucketFiles_FileVersions {
    #[doc = "Set the field `action`.\n"]
    pub fn set_action(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.action = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `bucket_id`.\n"]
    pub fn set_bucket_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.bucket_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_md5`.\n"]
    pub fn set_content_md5(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_md5 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_sha1`.\n"]
    pub fn set_content_sha1(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_sha1 = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `content_type`.\n"]
    pub fn set_content_type(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.content_type = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_id`.\n"]
    pub fn set_file_id(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_id = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_info`.\n"]
    pub fn set_file_info(
        mut self,
        v: impl std::convert::Into<terrars::RecField<terrars::PrimField<std::string::String>>>,
    ) -> Self {
        self.file_info = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `file_name`.\n"]
    pub fn set_file_name(mut self, v: impl std::convert::Into<terrars::PrimField<std::string::String>>) -> Self {
        self.file_name = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `server_side_encryption`.\n"]
    pub fn set_server_side_encryption(
        mut self,
        v: impl std::convert::Into<terrars::ListField<DataBucketFiles_FileVersions_ServerSideEncryption>>,
    ) -> Self {
        self.server_side_encryption = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `size`.\n"]
    pub fn set_size(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.size = std::option::Option::Some(v.into());
        self
    }

    #[doc = "Set the field `upload_timestamp`.\n"]
    pub fn set_upload_timestamp(mut self, v: impl std::convert::Into<terrars::PrimField<f64>>) -> Self {
        self.upload_timestamp = std::option::Option::Some(v.into());
        self
    }
}

impl terrars::ToListMappable for DataBucketFiles_FileVersions {
    type O = terrars::BlockAssignable<DataBucketFiles_FileVersions>;

    fn do_map(self, base: std::string::String) -> Self::O {
        terrars::BlockAssignable::Dynamic(terrars::DynamicBlock {
            for_each: format!("${{{}}}", base),
            iterator: "each".into(),
            content: self,
        })
    }
}

pub struct BuildDataBucketFiles_FileVersions {}

impl BuildDataBucketFiles_FileVersions {
    pub fn build(self) -> DataBucketFiles_FileVersions {
        DataBucketFiles_FileVersions {
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

#[allow(dead_code)]
pub struct DataBucketFiles_FileVersionsRef {
    shared: terrars::StackShared,
    base: std::string::String,
}

impl terrars::Ref for DataBucketFiles_FileVersionsRef {
    fn new(shared: terrars::StackShared, base: std::string::String) -> DataBucketFiles_FileVersionsRef {
        DataBucketFiles_FileVersionsRef {
            shared: shared,
            base: base.to_string(),
        }
    }
}

impl DataBucketFiles_FileVersionsRef {
    #[allow(dead_code)]
    fn shared(&self) -> &terrars::StackShared {
        &self.shared
    }

    #[doc = "Get a reference to the value of field `action` after provisioning.\n"]
    pub fn action(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.action", self.base))
    }

    #[doc = "Get a reference to the value of field `bucket_id` after provisioning.\n"]
    pub fn bucket_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.base))
    }

    #[doc = "Get a reference to the value of field `content_md5` after provisioning.\n"]
    pub fn content_md5(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_md5", self.base))
    }

    #[doc = "Get a reference to the value of field `content_sha1` after provisioning.\n"]
    pub fn content_sha1(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_sha1", self.base))
    }

    #[doc = "Get a reference to the value of field `content_type` after provisioning.\n"]
    pub fn content_type(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.content_type", self.base))
    }

    #[doc = "Get a reference to the value of field `file_id` after provisioning.\n"]
    pub fn file_id(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_id", self.base))
    }

    #[doc = "Get a reference to the value of field `file_info` after provisioning.\n"]
    pub fn file_info(&self) -> terrars::RecRef<terrars::PrimExpr<std::string::String>> {
        terrars::RecRef::new(self.shared().clone(), format!("{}.file_info", self.base))
    }

    #[doc = "Get a reference to the value of field `file_name` after provisioning.\n"]
    pub fn file_name(&self) -> terrars::PrimExpr<std::string::String> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.base))
    }

    #[doc = "Get a reference to the value of field `server_side_encryption` after provisioning.\n"]
    pub fn server_side_encryption(&self) -> terrars::ListRef<DataBucketFiles_FileVersions_ServerSideEncryptionRef> {
        terrars::ListRef::new(self.shared().clone(), format!("{}.server_side_encryption", self.base))
    }

    #[doc = "Get a reference to the value of field `size` after provisioning.\n"]
    pub fn size(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.size", self.base))
    }

    #[doc = "Get a reference to the value of field `upload_timestamp` after provisioning.\n"]
    pub fn upload_timestamp(&self) -> terrars::PrimExpr<f64> {
        terrars::PrimExpr::new(self.shared().clone(), format!("{}.upload_timestamp", self.base))
    }
}
