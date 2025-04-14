/* Wrappers for paramGetValue.

Since this is variadic, it can't be wrapped directly in Rust but
requires C code to translate.
*/

typedef int OfxStatus;
typedef void* OfxParamHandle;
typedef OfxStatus (*getValueFn)(OfxParamHandle  paramHandle, ...);


OfxStatus param_get_value_1(getValueFn param_get_value, OfxParamHandle param,
                            void *value) {
  return param_get_value(param, value);
}

OfxStatus param_get_value_2(getValueFn param_get_value, OfxParamHandle param,
                            void *value1, void *value2) {
  return param_get_value(param, value1, value2);
}

OfxStatus param_get_value_3(getValueFn param_get_value, OfxParamHandle param,
                            void *value1, void *value2, void *value3) {
  return param_get_value(param, value1, value2, value3);
}

OfxStatus param_get_value_4(getValueFn param_get_value, OfxParamHandle param,
                            void *value1, void *value2, void *value3, void *value4) {
  return param_get_value(param, value1, value2, value3, value4);
}
