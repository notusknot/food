(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[1],{

/***/ "./rust/pkg/food_web.js":
/*!******************************!*\
  !*** ./rust/pkg/food_web.js ***!
  \******************************/
/*! exports provided: run, __wbindgen_object_drop_ref, __wbg_instanceof_Window_a2a08d3918d7d4d0, __wbg_document_14a383364c173445, __wbg_body_36a11f2467926b2b, __wbg_createElement_2d8b75cffbd32c70, __wbg_settextContent_ce0ac980cbb8c820, __wbg_appendChild_e9d52952defb480f, __wbg_newnoargs_fc5356289219b93b, __wbg_call_4573f605ca4b5f10, __wbindgen_object_clone_ref, __wbg_self_ba1ddafe9ea7a3a2, __wbg_window_be3cc430364fd32c, __wbg_globalThis_56d9c9f814daeeee, __wbg_global_8c35aeee4ac77f2b, __wbindgen_is_undefined, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./food_web_bg.wasm */ \"./rust/pkg/food_web_bg.wasm\");\n/* harmony import */ var _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./food_web_bg.js */ \"./rust/pkg/food_web_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"run\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_a2a08d3918d7d4d0\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_instanceof_Window_a2a08d3918d7d4d0\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_14a383364c173445\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_document_14a383364c173445\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_body_36a11f2467926b2b\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_body_36a11f2467926b2b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_createElement_2d8b75cffbd32c70\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_createElement_2d8b75cffbd32c70\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_settextContent_ce0ac980cbb8c820\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_settextContent_ce0ac980cbb8c820\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_appendChild_e9d52952defb480f\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_appendChild_e9d52952defb480f\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_fc5356289219b93b\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_newnoargs_fc5356289219b93b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_4573f605ca4b5f10\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_call_4573f605ca4b5f10\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_clone_ref\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_ba1ddafe9ea7a3a2\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_self_ba1ddafe9ea7a3a2\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_be3cc430364fd32c\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_window_be3cc430364fd32c\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_56d9c9f814daeeee\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_globalThis_56d9c9f814daeeee\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_8c35aeee4ac77f2b\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_global_8c35aeee4ac77f2b\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_is_undefined\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return _food_web_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_rethrow\"]; });\n\n\n\n_food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_start\"]();\n\n\n//# sourceURL=webpack:///./rust/pkg/food_web.js?");

/***/ }),

/***/ "./rust/pkg/food_web_bg.js":
/*!*********************************!*\
  !*** ./rust/pkg/food_web_bg.js ***!
  \*********************************/
/*! exports provided: run, __wbindgen_object_drop_ref, __wbg_instanceof_Window_a2a08d3918d7d4d0, __wbg_document_14a383364c173445, __wbg_body_36a11f2467926b2b, __wbg_createElement_2d8b75cffbd32c70, __wbg_settextContent_ce0ac980cbb8c820, __wbg_appendChild_e9d52952defb480f, __wbg_newnoargs_fc5356289219b93b, __wbg_call_4573f605ca4b5f10, __wbindgen_object_clone_ref, __wbg_self_ba1ddafe9ea7a3a2, __wbg_window_be3cc430364fd32c, __wbg_globalThis_56d9c9f814daeeee, __wbg_global_8c35aeee4ac77f2b, __wbindgen_is_undefined, __wbindgen_throw, __wbindgen_rethrow */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(TextDecoder, module, global) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"run\", function() { return run; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_instanceof_Window_a2a08d3918d7d4d0\", function() { return __wbg_instanceof_Window_a2a08d3918d7d4d0; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_document_14a383364c173445\", function() { return __wbg_document_14a383364c173445; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_body_36a11f2467926b2b\", function() { return __wbg_body_36a11f2467926b2b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_createElement_2d8b75cffbd32c70\", function() { return __wbg_createElement_2d8b75cffbd32c70; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_settextContent_ce0ac980cbb8c820\", function() { return __wbg_settextContent_ce0ac980cbb8c820; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_appendChild_e9d52952defb480f\", function() { return __wbg_appendChild_e9d52952defb480f; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_newnoargs_fc5356289219b93b\", function() { return __wbg_newnoargs_fc5356289219b93b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_call_4573f605ca4b5f10\", function() { return __wbg_call_4573f605ca4b5f10; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_clone_ref\", function() { return __wbindgen_object_clone_ref; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_self_ba1ddafe9ea7a3a2\", function() { return __wbg_self_ba1ddafe9ea7a3a2; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_window_be3cc430364fd32c\", function() { return __wbg_window_be3cc430364fd32c; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_globalThis_56d9c9f814daeeee\", function() { return __wbg_globalThis_56d9c9f814daeeee; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_global_8c35aeee4ac77f2b\", function() { return __wbg_global_8c35aeee4ac77f2b; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_is_undefined\", function() { return __wbindgen_is_undefined; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_rethrow\", function() { return __wbindgen_rethrow; });\n/* harmony import */ var _food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./food_web_bg.wasm */ \"./rust/pkg/food_web_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0;\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n*/\nfunction run() {\n    _food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"run\"]();\n}\n\nfunction isLikeNone(x) {\n    return x === undefined || x === null;\n}\n\nfunction handleError(f, args) {\n    try {\n        return f.apply(this, args);\n    } catch (e) {\n        _food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_exn_store\"](addHeapObject(e));\n    }\n}\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\nfunction __wbg_instanceof_Window_a2a08d3918d7d4d0(arg0) {\n    const ret = getObject(arg0) instanceof Window;\n    return ret;\n};\n\nfunction __wbg_document_14a383364c173445(arg0) {\n    const ret = getObject(arg0).document;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_body_36a11f2467926b2b(arg0) {\n    const ret = getObject(arg0).body;\n    return isLikeNone(ret) ? 0 : addHeapObject(ret);\n};\n\nfunction __wbg_createElement_2d8b75cffbd32c70() { return handleError(function (arg0, arg1, arg2) {\n    const ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_settextContent_ce0ac980cbb8c820(arg0, arg1, arg2) {\n    getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);\n};\n\nfunction __wbg_appendChild_e9d52952defb480f() { return handleError(function (arg0, arg1) {\n    const ret = getObject(arg0).appendChild(getObject(arg1));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_newnoargs_fc5356289219b93b(arg0, arg1) {\n    const ret = new Function(getStringFromWasm0(arg0, arg1));\n    return addHeapObject(ret);\n};\n\nfunction __wbg_call_4573f605ca4b5f10() { return handleError(function (arg0, arg1) {\n    const ret = getObject(arg0).call(getObject(arg1));\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_object_clone_ref(arg0) {\n    const ret = getObject(arg0);\n    return addHeapObject(ret);\n};\n\nfunction __wbg_self_ba1ddafe9ea7a3a2() { return handleError(function () {\n    const ret = self.self;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_window_be3cc430364fd32c() { return handleError(function () {\n    const ret = window.window;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_globalThis_56d9c9f814daeeee() { return handleError(function () {\n    const ret = globalThis.globalThis;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbg_global_8c35aeee4ac77f2b() { return handleError(function () {\n    const ret = global.global;\n    return addHeapObject(ret);\n}, arguments) };\n\nfunction __wbindgen_is_undefined(arg0) {\n    const ret = getObject(arg0) === undefined;\n    return ret;\n};\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\nfunction __wbindgen_rethrow(arg0) {\n    throw takeObject(arg0);\n};\n\ncachedUint8Memory0 = new Uint8Array(_food_web_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! text-encoding */ \"./node_modules/text-encoding/index.js\")[\"TextDecoder\"], __webpack_require__(/*! ./../../node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module), __webpack_require__(/*! ./../../node_modules/webpack/buildin/global.js */ \"./node_modules/webpack/buildin/global.js\")))\n\n//# sourceURL=webpack:///./rust/pkg/food_web_bg.js?");

/***/ }),

/***/ "./rust/pkg/food_web_bg.wasm":
/*!***********************************!*\
  !*** ./rust/pkg/food_web_bg.wasm ***!
  \***********************************/
/*! exports provided: memory, run, __wbindgen_exn_store, __wbindgen_start */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./food_web_bg.js */ \"./rust/pkg/food_web_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./rust/pkg/food_web_bg.wasm?");

/***/ })

}]);