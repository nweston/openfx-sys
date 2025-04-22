#![allow(non_snake_case, non_upper_case_globals)]
use std::ffi::c_void;

include!("openfx_all.rs");

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
