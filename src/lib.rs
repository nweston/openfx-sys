#![allow(non_snake_case, non_upper_case_globals)]
use std::ffi::c_void;

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

include!("openfx_all.rs");

impl Into<*mut c_void> for OfxDrawContextHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxDrawContextHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxImageClipHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxImageClipHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxImageEffectHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxImageEffectHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxImageMemoryHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl Into<*mut c_void> for OfxInteractHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxInteractHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl From<*mut c_void> for OfxImageMemoryHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxMutexHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxMutexHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxParamHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxParamHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

impl Into<*mut c_void> for OfxParamSetHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxParamSetHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}
impl Into<*mut c_void> for OfxPropertySetHandle {
    fn into(self) -> *mut c_void {
        self.0 as _
    }
}

impl From<*mut c_void> for OfxPropertySetHandle {
    fn from(ptr: *mut c_void) -> Self {
        Self(ptr as _)
    }
}

pub mod ofxstatus {
    use crate::OfxStatus;
    pub const OK: OfxStatus = OfxStatus(0);
    pub const Failed: OfxStatus = OfxStatus(1);
    pub const ErrFatal: OfxStatus = OfxStatus(2);
    pub const ErrUnknown: OfxStatus = OfxStatus(3);
    pub const ErrMissingHostFeature: OfxStatus = OfxStatus(4);
    pub const ErrUnsupported: OfxStatus = OfxStatus(5);
    pub const ErrExists: OfxStatus = OfxStatus(6);
    pub const ErrFormat: OfxStatus = OfxStatus(7);
    pub const ErrMemory: OfxStatus = OfxStatus(8);
    pub const ErrBadHandle: OfxStatus = OfxStatus(9);
    pub const ErrBadIndex: OfxStatus = OfxStatus(10);
    pub const ErrValue: OfxStatus = OfxStatus(11);
    pub const ReplyYes: OfxStatus = OfxStatus(12);
    pub const ReplyNo: OfxStatus = OfxStatus(13);
    pub const ReplyDefault: OfxStatus = OfxStatus(14);
    pub const ErrImageFormat: OfxStatus = OfxStatus(1000);
    pub const GLOutOfMemory: OfxStatus = OfxStatus(1001);
    pub const GLRenderFailed: OfxStatus = OfxStatus(1002);
}

impl OfxStatus {
    pub fn failed(&self) -> bool {
        match *self {
            ofxstatus::OK | ofxstatus::ReplyDefault => false,
            _ => true,
        }
    }

    pub fn succeeded(&self) -> bool {
        !self.failed()
    }

    pub fn and_then<T, F>(self, op: F) -> Result<T, OfxStatus>
    where
        F: FnOnce() -> T,
    {
        if self.succeeded() {
            Ok(op())
        } else {
            Err(self)
        }
    }

    pub fn to_result(self) -> Result<(), OfxStatus> {
        self.and_then(|| ())
    }
}

fn status_to_str(status: &OfxStatus) -> Option<&'static str> {
    match *status {
        ofxstatus::OK => Some("OK"),
        ofxstatus::Failed => Some("Failed"),
        ofxstatus::ErrFatal => Some("ErrFatal"),
        ofxstatus::ErrUnknown => Some("ErrUnknown"),
        ofxstatus::ErrMissingHostFeature => Some("ErrMissingHostFeature"),
        ofxstatus::ErrUnsupported => Some("ErrUnsupported"),
        ofxstatus::ErrExists => Some("ErrExists"),
        ofxstatus::ErrFormat => Some("ErrFormat"),
        ofxstatus::ErrMemory => Some("ErrMemory"),
        ofxstatus::ErrBadHandle => Some("ErrBadHandle"),
        ofxstatus::ErrBadIndex => Some("ErrBadIndex"),
        ofxstatus::ErrValue => Some("ErrValue"),
        ofxstatus::ReplyYes => Some("ReplyYes"),
        ofxstatus::ReplyNo => Some("ReplyNo"),
        ofxstatus::ReplyDefault => Some("ReplyDefault"),
        ofxstatus::ErrImageFormat => Some("ErrImageFormat"),
        ofxstatus::GLOutOfMemory => Some("GLOutOfMemory"),
        ofxstatus::GLRenderFailed => Some("GLRenderFailed"),
        _ => None,
    }
}

impl std::fmt::Display for OfxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match status_to_str(self) {
            Some(s) => write!(f, "OfxStatus({})", s),
            None => write!(f, "OfxStatus({})", self.0),
        }
    }
}

impl std::fmt::Debug for OfxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

type ParamGetValueFn = unsafe extern "C" fn(paramHandle: OfxParamHandle, ...) -> OfxStatus;

extern "C" {
    pub fn param_get_value_1(
        paramGetValueFn: ParamGetValueFn,
        param: OfxParamHandle,
        value: *mut c_void,
    ) -> OfxStatus;

    pub fn param_get_value_2(
        paramGetValueFn: ParamGetValueFn,
        param: OfxParamHandle,
        value1: *mut c_void,
        value2: *mut c_void,
    ) -> OfxStatus;

    pub fn param_get_value_3(
        paramGetValueFn: ParamGetValueFn,
        param: OfxParamHandle,
        value1: *mut c_void,
        value2: *mut c_void,
        value3: *mut c_void,
    ) -> OfxStatus;

    pub fn param_get_value_4(
        paramGetValueFn: ParamGetValueFn,
        param: OfxParamHandle,
        value1: *mut c_void,
        value2: *mut c_void,
        value3: *mut c_void,
        value4: *mut c_void,
    ) -> OfxStatus;
}
