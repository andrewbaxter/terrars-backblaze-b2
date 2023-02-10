use serde::Serialize;
use std::cell::RefCell;
use std::rc::Rc;
use terrars::*;
use super::provider::ProviderB2;

#[derive(Serialize)]
struct DataBucketFileSignedUrlData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    depends_on: Vec<String>,
    #[serde(skip_serializing_if = "SerdeSkipDefault::is_default")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    for_each: Option<String>,
    bucket_id: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<PrimField<f64>>,
    file_name: PrimField<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<PrimField<String>>,
}

struct DataBucketFileSignedUrl_ {
    shared: StackShared,
    tf_id: String,
    data: RefCell<DataBucketFileSignedUrlData>,
}

#[derive(Clone)]
pub struct DataBucketFileSignedUrl(Rc<DataBucketFileSignedUrl_>);

impl DataBucketFileSignedUrl {
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

    #[doc= "Set the field `duration`.\nThe duration for which the presigned URL is valid"]
    pub fn set_duration(self, v: impl Into<PrimField<f64>>) -> Self {
        self.0.data.borrow_mut().duration = Some(v.into());
        self
    }

    #[doc= "Set the field `id`.\n"]
    pub fn set_id(self, v: impl Into<PrimField<String>>) -> Self {
        self.0.data.borrow_mut().id = Some(v.into());
        self
    }

    #[doc= "Get a reference to the value of field `bucket_id` after provisioning.\nThe ID of the bucket."]
    pub fn bucket_id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.bucket_id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nThe duration for which the presigned URL is valid"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe file name."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signed_url` after provisioning.\nThe signed URL for the given file"]
    pub fn signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signed_url", self.extract_ref()))
    }
}

impl Datasource for DataBucketFileSignedUrl {
    fn extract_ref(&self) -> String {
        format!("data.{}.{}", self.0.extract_datasource_type(), self.0.extract_tf_id())
    }
}

impl Dependable for DataBucketFileSignedUrl {
    fn extract_ref(&self) -> String {
        Datasource::extract_ref(self)
    }
}

impl ToListMappable for DataBucketFileSignedUrl {
    type O = ListRef<DataBucketFileSignedUrlRef>;

    fn do_map(self, base: String) -> Self::O {
        self.0.data.borrow_mut().for_each = Some(format!("${{{}}}", base));
        ListRef::new(self.0.shared.clone(), Datasource::extract_ref(self))
    }
}

impl Datasource_ for DataBucketFileSignedUrl_ {
    fn extract_datasource_type(&self) -> String {
        "b2_bucket_file_signed_url".into()
    }

    fn extract_tf_id(&self) -> String {
        self.tf_id.clone()
    }

    fn extract_value(&self) -> serde_json::Value {
        serde_json::to_value(&self.data).unwrap()
    }
}

pub struct BuildDataBucketFileSignedUrl {
    pub tf_id: String,
    #[doc= "The ID of the bucket."]
    pub bucket_id: PrimField<String>,
    #[doc= "The file name."]
    pub file_name: PrimField<String>,
}

impl BuildDataBucketFileSignedUrl {
    pub fn build(self, stack: &mut Stack) -> DataBucketFileSignedUrl {
        let out = DataBucketFileSignedUrl(Rc::new(DataBucketFileSignedUrl_ {
            shared: stack.shared.clone(),
            tf_id: self.tf_id,
            data: RefCell::new(DataBucketFileSignedUrlData {
                depends_on: core::default::Default::default(),
                provider: None,
                for_each: None,
                bucket_id: self.bucket_id,
                duration: core::default::Default::default(),
                file_name: self.file_name,
                id: core::default::Default::default(),
            }),
        }));
        stack.add_datasource(out.0.clone());
        out
    }
}

pub struct DataBucketFileSignedUrlRef {
    shared: StackShared,
    base: String,
}

impl Ref for DataBucketFileSignedUrlRef {
    fn new(shared: StackShared, base: String) -> Self {
        Self {
            shared: shared,
            base: base,
        }
    }
}

impl DataBucketFileSignedUrlRef {
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

    #[doc= "Get a reference to the value of field `duration` after provisioning.\nThe duration for which the presigned URL is valid"]
    pub fn duration(&self) -> PrimExpr<f64> {
        PrimExpr::new(self.shared().clone(), format!("{}.duration", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `file_name` after provisioning.\nThe file name."]
    pub fn file_name(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.file_name", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `id` after provisioning.\n"]
    pub fn id(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.id", self.extract_ref()))
    }

    #[doc= "Get a reference to the value of field `signed_url` after provisioning.\nThe signed URL for the given file"]
    pub fn signed_url(&self) -> PrimExpr<String> {
        PrimExpr::new(self.shared().clone(), format!("{}.signed_url", self.extract_ref()))
    }
}
