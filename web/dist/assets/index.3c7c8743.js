var O=(n,e)=>()=>(e||n((e={exports:{}}).exports,e),e.exports);var B=O((q,L)=>{const W=function(){const e=document.createElement("link").relList;if(e&&e.supports&&e.supports("modulepreload"))return;for(const o of document.querySelectorAll('link[rel="modulepreload"]'))r(o);new MutationObserver(o=>{for(const u of o)if(u.type==="childList")for(const s of u.addedNodes)s.tagName==="LINK"&&s.rel==="modulepreload"&&r(s)}).observe(document,{childList:!0,subtree:!0});function t(o){const u={};return o.integrity&&(u.integrity=o.integrity),o.referrerpolicy&&(u.referrerPolicy=o.referrerpolicy),o.crossorigin==="use-credentials"?u.credentials="include":o.crossorigin==="anonymous"?u.credentials="omit":u.credentials="same-origin",u}function r(o){if(o.ep)return;o.ep=!0;const u=t(o);fetch(o.href,u)}};W();let _;const b=new Array(32).fill(void 0);b.push(void 0,null,!0,!1);function c(n){return b[n]}let l=b.length;function j(n){n<36||(b[n]=l,l=n)}function I(n){const e=c(n);return j(n),e}function i(n){l===b.length&&b.push(b.length+1);const e=l;return l=b[e],b[e]=n,e}const v=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});v.decode();let m;function g(){return m.byteLength===0&&(m=new Uint8Array(_.memory.buffer)),m}function d(n,e){return v.decode(g().subarray(n,n+e))}let y;function h(){return y.byteLength===0&&(y=new Int32Array(_.memory.buffer)),y}function x(n,e,t,r){try{const s=_.__wbindgen_add_to_stack_pointer(-16);_.match_bounds_js(s,n,e,t,r);var o=h()[s/4+0],u=h()[s/4+1];if(u)throw I(o)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function A(n){return n==null}function a(n,e){try{return n.apply(this,e)}catch(t){_.__wbindgen_export_0(i(t))}}let E=0;const p=new TextEncoder("utf-8"),M=typeof p.encodeInto=="function"?function(n,e){return p.encodeInto(n,e)}:function(n,e){const t=p.encode(n);return e.set(t),{read:n.length,written:t.length}};function S(n,e,t){if(t===void 0){const f=p.encode(n),w=e(f.length);return g().subarray(w,w+f.length).set(f),E=f.length,w}let r=n.length,o=e(r);const u=g();let s=0;for(;s<r;s++){const f=n.charCodeAt(s);if(f>127)break;u[o+s]=f}if(s!==r){s!==0&&(n=n.slice(s)),o=t(o,r,r=s+n.length*3);const f=g().subarray(o+s,o+r);s+=M(n,f).written}return E=s,o}function T(n,e){return g().subarray(n/1,n/1+e)}async function U(n,e){if(typeof Response=="function"&&n instanceof Response){if(typeof WebAssembly.instantiateStreaming=="function")try{return await WebAssembly.instantiateStreaming(n,e)}catch(r){if(n.headers.get("Content-Type")!="application/wasm")console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",r);else throw r}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}else{const t=await WebAssembly.instantiate(n,e);return t instanceof WebAssembly.Instance?{instance:t,module:n}:t}}function C(){const n={};return n.wbg={},n.wbg.__wbindgen_object_drop_ref=function(e){I(e)},n.wbg.__wbg_instanceof_Window_a2a08d3918d7d4d0=function(e){return c(e)instanceof Window},n.wbg.__wbg_document_14a383364c173445=function(e){const t=c(e).document;return A(t)?0:i(t)},n.wbg.__wbg_body_36a11f2467926b2b=function(e){const t=c(e).body;return A(t)?0:i(t)},n.wbg.__wbg_createElement_2d8b75cffbd32c70=function(){return a(function(e,t,r){const o=c(e).createElement(d(t,r));return i(o)},arguments)},n.wbg.__wbg_settextContent_ce0ac980cbb8c820=function(e,t,r){c(e).textContent=t===0?void 0:d(t,r)},n.wbg.__wbg_appendChild_e9d52952defb480f=function(){return a(function(e,t){const r=c(e).appendChild(c(t));return i(r)},arguments)},n.wbg.__wbg_new_693216e109162396=function(){const e=new Error;return i(e)},n.wbg.__wbg_stack_0ddaca5d1abfb52f=function(e,t){const r=c(t).stack,o=S(r,_.__wbindgen_export_2,_.__wbindgen_export_3),u=E;h()[e/4+1]=u,h()[e/4+0]=o},n.wbg.__wbg_error_09919627ac0992f5=function(e,t){try{console.error(d(e,t))}finally{_.__wbindgen_export_1(e,t)}},n.wbg.__wbg_randomFillSync_91e2b39becca6147=function(){return a(function(e,t,r){c(e).randomFillSync(T(t,r))},arguments)},n.wbg.__wbg_getRandomValues_b14734aa289bc356=function(){return a(function(e,t){c(e).getRandomValues(c(t))},arguments)},n.wbg.__wbg_process_e56fd54cf6319b6c=function(e){const t=c(e).process;return i(t)},n.wbg.__wbindgen_is_object=function(e){const t=c(e);return typeof t=="object"&&t!==null},n.wbg.__wbg_versions_77e21455908dad33=function(e){const t=c(e).versions;return i(t)},n.wbg.__wbg_node_0dd25d832e4785d5=function(e){const t=c(e).node;return i(t)},n.wbg.__wbindgen_is_string=function(e){return typeof c(e)=="string"},n.wbg.__wbg_static_accessor_NODE_MODULE_26b231378c1be7dd=function(){return i(L)},n.wbg.__wbg_require_0db1598d9ccecb30=function(){return a(function(e,t,r){const o=c(e).require(d(t,r));return i(o)},arguments)},n.wbg.__wbg_crypto_b95d7173266618a9=function(e){const t=c(e).crypto;return i(t)},n.wbg.__wbg_msCrypto_5a86d77a66230f81=function(e){const t=c(e).msCrypto;return i(t)},n.wbg.__wbg_newnoargs_fc5356289219b93b=function(e,t){const r=new Function(d(e,t));return i(r)},n.wbg.__wbg_call_4573f605ca4b5f10=function(){return a(function(e,t){const r=c(e).call(c(t));return i(r)},arguments)},n.wbg.__wbindgen_object_clone_ref=function(e){const t=c(e);return i(t)},n.wbg.__wbg_self_ba1ddafe9ea7a3a2=function(){return a(function(){const e=self.self;return i(e)},arguments)},n.wbg.__wbg_window_be3cc430364fd32c=function(){return a(function(){const e=window.window;return i(e)},arguments)},n.wbg.__wbg_globalThis_56d9c9f814daeeee=function(){return a(function(){const e=globalThis.globalThis;return i(e)},arguments)},n.wbg.__wbg_global_8c35aeee4ac77f2b=function(){return a(function(){const e=global.global;return i(e)},arguments)},n.wbg.__wbindgen_is_undefined=function(e){return c(e)===void 0},n.wbg.__wbg_buffer_de1150f91b23aa89=function(e){const t=c(e).buffer;return i(t)},n.wbg.__wbg_new_97cf52648830a70d=function(e){const t=new Uint8Array(c(e));return i(t)},n.wbg.__wbg_set_a0172b213e2469e9=function(e,t,r){c(e).set(c(t),r>>>0)},n.wbg.__wbg_length_e09c0b925ab8de5d=function(e){return c(e).length},n.wbg.__wbg_newwithlength_e833b89f9db02732=function(e){const t=new Uint8Array(e>>>0);return i(t)},n.wbg.__wbg_subarray_9482ae5cd5cd99d3=function(e,t,r){const o=c(e).subarray(t>>>0,r>>>0);return i(o)},n.wbg.__wbindgen_throw=function(e,t){throw new Error(d(e,t))},n.wbg.__wbindgen_memory=function(){const e=_.memory;return i(e)},n}function R(n,e){return _=n.exports,k.__wbindgen_wasm_module=e,y=new Int32Array(_.memory.buffer),m=new Uint8Array(_.memory.buffer),_}async function k(n){typeof n=="undefined"&&(n="/food/assets/food_web_bg.wasm");const e=C();(typeof n=="string"||typeof Request=="function"&&n instanceof Request||typeof URL=="function"&&n instanceof URL)&&(n=fetch(n));const{instance:t,module:r}=await U(await n,e);return R(t,r)}function F(){let n=+document.getElementById("kcal_lb").value,e=+document.getElementById("kcal_ub").value,t=+document.getElementById("daily_meals").value,r=+document.getElementById("total_days").value;x(n,e,t,r)}k().then(()=>{console.log("init wasm-pack"),document.getElementById("gen_btn").addEventListener("click",F)})});export default B();
