let wasm_bindgen;!function(){const n={};let e,_;const t=new Array(32).fill(void 0);function b(n){return t[n]}t.push(void 0,null,!0,!1);let r=t.length;function c(n){r===t.length&&t.push(t.length+1);const e=r;return r=t[e],t[e]=n,e}function a(n){const e=b(n);return function(n){n<36||(t[n]=r,r=n)}(n),e}let o=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});o.decode();let i=null;function g(){return null!==i&&i.buffer===e.__wbindgen_export_0.buffer||(i=new Uint8Array(e.__wbindgen_export_0.buffer)),i}function w(n,e){return o.decode(g().slice(n,n+e))}let f=0,u=new TextEncoder("utf-8");function l(n,e,_){if(void 0===_){const _=u.encode(n),t=e(_.length);return g().subarray(t,t+_.length).set(_),f=_.length,t}let t=n.length,b=e(t);const r=g();let c=0;for(;c<t;c++){const e=n.charCodeAt(c);if(e>127)break;r[b+c]=e}if(c!==t){0!==c&&(n=n.slice(c)),b=_(b,t,t=c+3*n.length);c+=function(n,e){const _=u.encode(n);return e.set(_),{read:n.length,written:_.length}}(n,g().subarray(b+c,b+t)).written}return f=c,b}function d(n){return null==n}let s=null;function m(){return null!==s&&s.buffer===e.__wbindgen_export_0.buffer||(s=new Int32Array(e.__wbindgen_export_0.buffer)),s}function v(n){const e=typeof n;if("number"==e||"boolean"==e||null==n)return`${n}`;if("string"==e)return`"${n}"`;if("symbol"==e){const e=n.description;return null==e?"Symbol":`Symbol(${e})`}if("function"==e){const e=n.name;return"string"==typeof e&&e.length>0?`Function(${e})`:"Function"}if(Array.isArray(n)){const e=n.length;let _="[";e>0&&(_+=v(n[0]));for(let t=1;t<e;t++)_+=", "+v(n[t]);return _+="]",_}const _=/\[object ([^\]]+)\]/.exec(toString.call(n));let t;if(!(_.length>1))return toString.call(n);if(t=_[1],"Object"==t)try{return"Object("+JSON.stringify(n)+")"}catch(n){return"Object"}return n instanceof Error?`${n.name}: ${n.message}\n${n.stack}`:t}const p=new FinalizationRegistry((n=>{e.__wbindgen_export_3.get(n.dtor)(n.a,n.b)}));function h(n,_,t,b){const r={a:n,b:_,cnt:1,dtor:t},c=(...n)=>{r.cnt++;const _=r.a;r.a=0;try{return b(_,r.b,...n)}finally{0==--r.cnt?(e.__wbindgen_export_3.get(r.dtor)(_,r.b),p.unregister(r)):r.a=_}};return c.original=r,p.register(c,r,r),c}function y(n,_,t){e.wasm_bindgen__convert__closures__invoke1_mut__h08f43863f0f6abb9(n,_,c(t))}function E(n,_,t){e.wasm_bindgen__convert__closures__invoke1_mut__hd93805d8df4c1d5b(n,_,t)}function A(n,_,t){e.wasm_bindgen__convert__closures__invoke1_mut__h08f43863f0f6abb9(n,_,c(t))}function C(n,_,t){e.wasm_bindgen__convert__closures__invoke1_mut__h08f43863f0f6abb9(n,_,c(t))}function S(n,_,t){e.wasm_bindgen__convert__closures__invoke1_mut__h08f43863f0f6abb9(n,_,c(t))}function M(n){return function(){try{return n.apply(this,arguments)}catch(n){e.__wbindgen_exn_store(c(n))}}}n.main=function(){e.main()},n.child_entry_point=function(n){e.child_entry_point(n)};const H=new FinalizationRegistry((n=>e.__wbg_workerpool_free(n)));class L{static __wrap(n){const e=Object.create(L.prototype);return e.ptr=n,H.register(e,e.ptr,e),e}free(){const n=this.ptr;this.ptr=0,H.unregister(this),e.__wbg_workerpool_free(n)}constructor(n){var _=e.workerpool_new(n);return L.__wrap(_)}}n.WorkerPool=L,wasm_bindgen=Object.assign((async function n(t,r){if(void 0===t){let n;n="undefined"==typeof document?location.href:document.currentScript.src,t=n.replace(/\.js$/,"_bg.wasm")}const o={wbg:{}};o.wbg.__wbindgen_object_clone_ref=function(n){return c(b(n))},o.wbg.__wbg_performance_eee010e5e49f08df=function(n){var e=b(n).performance;return d(e)?0:c(e)},o.wbg.__wbg_now_5ae3d18d57dd226f=function(n){return b(n).now()},o.wbg.__wbindgen_object_drop_ref=function(n){a(n)},o.wbg.__wbindgen_cb_drop=function(n){const e=a(n).original;if(1==e.cnt--)return e.a=0,!0;return!1},o.wbg.__wbg_removeEventListener_19da1e4551104118=M((function(n,e,_,t){b(n).removeEventListener(w(e,_),b(t))})),o.wbg.__wbg_addEventListener_63378230aa6735d7=M((function(n,e,_,t){b(n).addEventListener(w(e,_),b(t))})),o.wbg.__wbg_createTextNode_bbff6f9f6e6b38bf=function(n,e,_){return c(b(n).createTextNode(w(e,_)))},o.wbg.__wbg_instanceof_Node_b8e4c20ae3455a84=function(n){return b(n)instanceof Node},o.wbg.__wbg_replaceChild_e5c6e317e9b8f117=M((function(n,e,_){return c(b(n).replaceChild(b(e),b(_)))})),o.wbg.__wbg_removeChild_51369e223cb8a779=M((function(n,e){return c(b(n).removeChild(b(e)))})),o.wbg.__wbg_insertBefore_5886cc01dc0233e3=M((function(n,e,_){return c(b(n).insertBefore(b(e),b(_)))})),o.wbg.__wbindgen_string_new=function(n,e){return c(w(n,e))},o.wbg.__wbg_error_9783be44659339ea=function(n){console.error(b(n))},o.wbg.__wbg_settextContent_1c74a88a26c29897=function(n,e,_){b(n).textContent=0===e?void 0:w(e,_)},o.wbg.__wbg_instanceof_Element_6bc6669240998e07=function(n){return b(n)instanceof Element},o.wbg.__wbg_removeAttribute_43b052e0560d223b=M((function(n,e,_){b(n).removeAttribute(w(e,_))})),o.wbg.__wbg_instanceof_Window_fbe0320f34c4cd31=function(n){return"undefined"!=typeof Window&&b(n)instanceof Window},o.wbg.__wbg_requestAnimationFrame_65ebf8f2415064e2=M((function(n,e){return b(n).requestAnimationFrame(b(e))})),o.wbg.__wbg_document_2b44f2a86e03665a=function(n){var e=b(n).document;return d(e)?0:c(e)},o.wbg.__wbg_querySelector_be2514e5de79507f=M((function(n,e,_){var t=b(n).querySelector(w(e,_));return d(t)?0:c(t)})),o.wbg.__wbg_getAttribute_5d45357036c3ad3d=function(n,_,t,r){var c=b(_).getAttribute(w(t,r)),a=d(c)?0:l(c,e.__wbindgen_malloc,e.__wbindgen_realloc),o=f;m()[n/4+1]=o,m()[n/4+0]=a},o.wbg.__wbg_newwithbase_7d05d53083c33de1=M((function(n,e,_,t){return c(new URL(w(n,e),w(_,t)))})),o.wbg.__wbg_pathname_39a814bd67adfc94=function(n,_){var t=l(b(_).pathname,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_getElementById_5bd6efc3d82494aa=function(n,e,_){var t=b(n).getElementById(w(e,_));return d(t)?0:c(t)},o.wbg.__wbg_firstChild_2e342721e842db18=function(n){var e=b(n).firstChild;return d(e)?0:c(e)},o.wbg.__wbg_navigator_74ca2cbc2348b985=function(n){return c(b(n).navigator)},o.wbg.__wbg_hardwareConcurrency_93d102d4b34be8fc=function(n){return b(n).hardwareConcurrency},o.wbg.__wbg_preventDefault_4eb36ec8e5563ad6=function(n){b(n).preventDefault()},o.wbg.__wbg_history_4a086e80d4814e51=M((function(n){return c(b(n).history)})),o.wbg.__wbg_pushState_14bda476a428fd88=M((function(n,e,_,t,r,c){b(n).pushState(b(e),w(_,t),0===r?void 0:w(r,c))})),o.wbg.__wbg_instanceof_Event_84ceaefe19aa1afc=function(n){return b(n)instanceof Event},o.wbg.__wbg_target_62e7aaed452a6541=function(n){var e=b(n).target;return d(e)?0:c(e)},o.wbg.__wbg_instanceof_HtmlInputElement_bd1ce15e756a8ae2=function(n){return b(n)instanceof HTMLInputElement},o.wbg.__wbg_instanceof_HtmlTextAreaElement_ceec64505a6bc087=function(n){return b(n)instanceof HTMLTextAreaElement},o.wbg.__wbg_instanceof_HtmlSelectElement_9e453923d9cacda8=function(n){return b(n)instanceof HTMLSelectElement},o.wbg.__wbg_instanceof_HtmlProgressElement_7512c5ab4b35ee12=function(n){return b(n)instanceof HTMLProgressElement},o.wbg.__wbg_instanceof_HtmlOptionElement_da369b5bcab0c89b=function(n){return b(n)instanceof HTMLOptionElement},o.wbg.__wbg_instanceof_HtmlButtonElement_56d6508cc80478bf=function(n){return b(n)instanceof HTMLButtonElement},o.wbg.__wbg_instanceof_HtmlDataElement_1f17691629121e81=function(n){return b(n)instanceof HTMLDataElement},o.wbg.__wbg_instanceof_HtmlMeterElement_6ae9875141622d1e=function(n){return b(n)instanceof HTMLMeterElement},o.wbg.__wbg_instanceof_HtmlLiElement_32be2d4a3143ff31=function(n){return b(n)instanceof HTMLLIElement},o.wbg.__wbg_instanceof_HtmlOutputElement_340d42a60f143539=function(n){return b(n)instanceof HTMLOutputElement},o.wbg.__wbg_instanceof_HtmlParamElement_671cb99f2302b661=function(n){return b(n)instanceof HTMLParamElement},o.wbg.__wbg_type_8589e802736fc354=function(n,_){var t=l(b(_).type,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_e27a74c3db49694f=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_41a8d072fc94a8a4=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_b431982196f2362c=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_9f3a2da51df824bd=function(n){return b(n).value},o.wbg.__wbg_value_ba9bb217261f861f=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_729c36709b55e9b8=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_0af2c4554af875e0=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_dc81ba20500457e2=function(n){return b(n).value},o.wbg.__wbg_value_6575bc19d9bf3a1b=function(n){return b(n).value},o.wbg.__wbg_value_92dc50ea2ccb6345=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_value_a3c8247f2189a61d=function(n,_){var t=l(b(_).value,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbindgen_string_get=function(n,_){const t=b(_);var r="string"==typeof t?t:void 0,c=d(r)?0:l(r,e.__wbindgen_malloc,e.__wbindgen_realloc),a=f;m()[n/4+1]=a,m()[n/4+0]=c},o.wbg.__wbg_createElement_7cbe07ad3289abea=M((function(n,e,_){return c(b(n).createElement(w(e,_)))})),o.wbg.__wbg_createElementNS_ae76308e06470c87=M((function(n,e,_,t,r){return c(b(n).createElementNS(0===e?void 0:w(e,_),w(t,r)))})),o.wbg.__wbg_setAttribute_b638fce95071fff6=M((function(n,e,_,t,r){b(n).setAttribute(w(e,_),w(t,r))})),o.wbg.__wbg_appendChild_98dedaeac24501f2=M((function(n,e){return c(b(n).appendChild(b(e)))})),o.wbg.__wbg_instanceof_HtmlElement_ca5564c35b091ac9=function(n){return b(n)instanceof HTMLElement},o.wbg.__wbg_focus_a3a607c2c1081818=M((function(n){b(n).focus()})),o.wbg.__wbg_setvalue_e3b8a9c5a4ad0114=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_closest_85d4b9d9481f9534=M((function(n,e,_){var t=b(n).closest(w(e,_));return d(t)?0:c(t)})),o.wbg.__wbg_hasAttribute_7285532dbf4a3478=function(n,e,_){return b(n).hasAttribute(w(e,_))},o.wbg.__wbg_instanceof_PopStateEvent_95152e2939b0b1f6=function(n){return b(n)instanceof PopStateEvent},o.wbg.__wbg_state_a3024a8a1af2c159=function(n){return c(b(n).state)},o.wbg.__wbg_namespaceURI_5ab9ad8edd6b745e=function(n,_){var t=b(_).namespaceURI,r=d(t)?0:l(t,e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;m()[n/4+1]=c,m()[n/4+0]=r},o.wbg.__wbg_tagName_ae2a7d92706047a7=function(n,_){var t=l(b(_).tagName,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_getAttributeNames_cc38674f20945c5a=function(n){return c(b(n).getAttributeNames())},o.wbg.__wbg_forEach_2b60cd791bc26871=function(n,_,t){try{var r={a:_,b:t};b(n).forEach(((n,_,t)=>{const b=r.a;r.a=0;try{return function(n,_,t,b,r){e.wasm_bindgen__convert__closures__invoke3_mut__h04a92558bd7b297e(n,_,c(t),b,c(r))}(b,r.b,n,_,t)}finally{r.a=b}}))}finally{r.a=r.b=0}},o.wbg.__wbg_childNodes_19bac002cf3d52ec=function(n){return c(b(n).childNodes)},o.wbg.__wbg_length_9cc9d49162055e6a=function(n){return b(n).length},o.wbg.__wbg_get_241002221ba45b17=function(n,e){var _=b(n)[e>>>0];return d(_)?0:c(_)},o.wbg.__wbg_nodeType_f23f2d530e5e3ed2=function(n){return b(n).nodeType},o.wbg.__wbg_textContent_8c1def1e3477ec2e=function(n,_){var t=b(_).textContent,r=d(t)?0:l(t,e.__wbindgen_malloc,e.__wbindgen_realloc),c=f;m()[n/4+1]=c,m()[n/4+0]=r},o.wbg.__wbindgen_memory=function(){return c(e.__wbindgen_export_0)},o.wbg.__wbg_buffer_bc64154385c04ac4=function(n){return c(b(n).buffer)},o.wbg.__wbg_new_e0158f0dbcc01d9c=function(n){return c(new Uint8ClampedArray(b(n)))},o.wbg.__wbg_slice_3236a69a24c836e1=function(n,e,_){return c(b(n).slice(e>>>0,_>>>0))},o.wbg.__wbg_new_33977f04e93e405a=M((function(n,e,_){return c(new ImageData(b(n),e,_))})),o.wbg.__wbg_putImageData_e3da2e3c722c5886=M((function(n,e,_,t){b(n).putImageData(b(e),_,t)})),o.wbg.__wbg_log_2e875b1d2f6f87ac=function(n){console.log(b(n))},o.wbg.__wbg_call_7a2b5e98ac536644=M((function(n,e,_){return c(b(n).call(b(e),b(_)))})),o.wbg.__wbg_instanceof_HtmlCanvasElement_bd2459c62d076bcd=function(n){return b(n)instanceof HTMLCanvasElement},o.wbg.__wbg_getContext_7f0328be9fe8c1ec=M((function(n,e,_){var t=b(n).getContext(w(e,_));return d(t)?0:c(t)})),o.wbg.__wbg_instanceof_CanvasRenderingContext2d_302c6fce2ddc6344=function(n){return b(n)instanceof CanvasRenderingContext2D},o.wbg.__wbindgen_number_new=function(n){return c(n)},o.wbg.__wbg_postMessage_58f317d71e0a2b3f=M((function(n,e){b(n).postMessage(b(e))})),o.wbg.__wbg_new_bae826039151b559=function(n,_){try{var t={a:n,b:_},b=new Promise(((n,_)=>{const b=t.a;t.a=0;try{return function(n,_,t,b){e.wasm_bindgen__convert__closures__invoke2_mut__h57a9371fa26939df(n,_,c(t),c(b))}(b,t.b,n,_)}finally{t.a=b}}));return c(b)}finally{t.a=t.b=0}},o.wbg.__wbg_then_3d9a54b0affdf26d=function(n,e,_){return c(b(n).then(b(e),b(_)))},o.wbg.__wbg_id_e707202c1ce9d68f=function(n,_){var t=l(b(_).id,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_getBoundingClientRect_813f74e2f4f344e2=function(n){return c(b(n).getBoundingClientRect())},o.wbg.__wbg_clientX_df24871aabb01061=function(n){return b(n).clientX},o.wbg.__wbg_left_515d30ed4e76f921=function(n){return b(n).left},o.wbg.__wbg_clientY_b91be9a030c7bd7c=function(n){return b(n).clientY},o.wbg.__wbg_top_759d072a3a15fd13=function(n){return b(n).top},o.wbg.__wbg_shiftKey_31c1bdd985f9be8e=function(n){return b(n).shiftKey},o.wbg.__wbg_width_5c7aabd2a7489c51=function(n){return b(n).width},o.wbg.__wbg_height_b3f5ccd4af20f47f=function(n){return b(n).height},o.wbg.__wbg_replaceState_452a5e828796c789=M((function(n,e,_,t,r,c){b(n).replaceState(b(e),w(_,t),0===r?void 0:w(r,c))})),o.wbg.__wbg_setonmessage_fefcbee2824ce979=function(n,e){b(n).onmessage=b(e)},o.wbg.__wbg_setonerror_8e87de7f23c137c6=function(n,e){b(n).onerror=b(e)},o.wbg.__wbindgen_jsval_eq=function(n,e){return b(n)===b(e)},o.wbg.__wbg_type_a49a5fc3f5ab818b=function(n,_){var t=l(b(_).type,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_location_0cf481bba6be4a4d=function(n){return c(b(n).location)},o.wbg.__wbg_origin_fbbb5602c8516839=function(n,_){var t=l(b(_).origin,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_pathname_28f192c711f0cc9c=function(n,_){var t=l(b(_).pathname,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_new_dc5b27cfd2149b8f=function(){return c(new Object)},o.wbg.__wbg_set_3afd31f38e771338=M((function(n,e,_){return Reflect.set(b(n),b(e),b(_))})),o.wbg.__wbindgen_json_parse=function(n,e){return c(JSON.parse(w(n,e)))},o.wbg.__wbg_newwithstrsequenceandoptions_2c10c41f3bc24d7d=M((function(n,e){return c(new Blob(b(n),b(e)))})),o.wbg.__wbg_createObjectURL_e46eb3ca9fdf20c5=M((function(n,_){var t=l(URL.createObjectURL(b(_)),e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t})),o.wbg.__wbg_new_4ae6bf757712b3fd=M((function(n,e){return c(new Worker(w(n,e)))})),o.wbg.__wbg_new_1abc33d4f9ba3e80=function(){return c(new Array)},o.wbg.__wbindgen_module=function(){return c(n.__wbindgen_wasm_module)},o.wbg.__wbg_push_44968dcdf4cfbb43=function(n,e){return b(n).push(b(e))},o.wbg.__wbg_instanceof_ErrorEvent_c25954701bebe884=function(n){return b(n)instanceof ErrorEvent},o.wbg.__wbg_instanceof_MessageEvent_e0173ecc2c3228fb=function(n){return b(n)instanceof MessageEvent},o.wbg.__wbg_message_4ca5e0c1335d1246=function(n,_){var t=l(b(_).message,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_postMessage_952d6d0f2eb1d008=M((function(n,e){b(n).postMessage(b(e))})),o.wbg.__wbg_new_59cb74e423758ede=function(){return c(new Error)},o.wbg.__wbg_stack_558ba5917b466edd=function(n,_){var t=l(b(_).stack,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_error_4bb6c2a97407129a=function(n,_){try{console.error(w(n,_))}finally{e.__wbindgen_free(n,_)}},o.wbg.__wbg_self_f865985e662246aa=M((function(){return c(self.self)})),o.wbg.__wbg_require_c59851dfa0dc7e78=M((function(n,e,_){return c(b(n).require(w(e,_)))})),o.wbg.__wbg_crypto_bfb05100db79193b=function(n){return c(b(n).crypto)},o.wbg.__wbg_msCrypto_f6dddc6ae048b7e2=function(n){return c(b(n).msCrypto)},o.wbg.__wbindgen_is_undefined=function(n){return void 0===b(n)},o.wbg.__wbg_newwithlength_48451d71403bfede=function(n){return c(new Uint8Array(n>>>0))},o.wbg.__wbg_static_accessor_MODULE_39947eb3fe77895f=function(){return c(u)},o.wbg.__wbg_length_4c7aec6f35774e3d=function(n){return b(n).length},o.wbg.__wbg_get_a8b9619536c590d4=function(n,e){return c(b(n)[e>>>0])},o.wbg.__wbg_self_77eca7b42660e1bb=M((function(){return c(self.self)})),o.wbg.__wbg_window_51dac01569f1ba70=M((function(){return c(window.window)})),o.wbg.__wbg_globalThis_34bac2d08ebb9b58=M((function(){return c(globalThis.globalThis)})),o.wbg.__wbg_global_1c436164a66c9c22=M((function(){return c(global.global)})),o.wbg.__wbg_newnoargs_ab5e899738c0eff4=function(n,e){return c(new Function(w(n,e)))},o.wbg.__wbg_call_ab183a630df3a257=M((function(n,e){return c(b(n).call(b(e)))})),o.wbg.__wbindgen_debug_string=function(n,_){var t=l(v(b(_)),e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_new_3bcf779d33c16832=function(n,e,_,t){return c(new RegExp(w(n,e),w(_,t)))},o.wbg.__wbg_match_1f59c1443e0a451a=function(n,e){var _=b(n).match(b(e));return d(_)?0:c(_)},o.wbg.__wbg_encodeURIComponent_a07c51869a3fb46b=function(n,e){return c(encodeURIComponent(w(n,e)))},o.wbg.__wbg_setsearch_977908b49232b227=function(n,e,_){b(n).search=w(e,_)},o.wbg.__wbg_sethash_e182b866cd92157f=function(n,e,_){b(n).hash=w(e,_)},o.wbg.__wbg_href_a1fd6256fbe702f7=function(n,_){var t=l(b(_).href,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_new_8de6e422995a03aa=M((function(){return c(new URLSearchParams)})),o.wbg.__wbg_append_fc7e299022bdccf9=function(n,e,_,t,r){b(n).append(w(e,_),w(t,r))},o.wbg.__wbg_toString_162eb8aa99ef09cc=function(n){return c(b(n).toString())},o.wbg.__wbg_location_df2a42f020b6b0fe=function(n){return c(b(n).location)},o.wbg.__wbg_href_6a2edee803039e44=M((function(n,_){var t=l(b(_).href,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t})),o.wbg.__wbg_hash_a98304b9b6e89817=function(n,_){var t=l(b(_).hash,e.__wbindgen_malloc,e.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},o.wbg.__wbg_decodeURIComponent_a79156f5be5849a3=M((function(n,e){return c(decodeURIComponent(w(n,e)))})),o.wbg.__wbg_searchParams_dca82447cb7421aa=function(n){return c(b(n).searchParams)},o.wbg.__wbg_from_cf86da5763e20354=function(n){return c(Array.from(b(n)))},o.wbg.__wbg_cancelAnimationFrame_594443705ec1f21d=M((function(n,e){b(n).cancelAnimationFrame(e)})),o.wbg.__wbg_setvalue_3bf5c8f910de1ebe=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_b90a627efadf1c77=function(n,e){b(n).value=e},o.wbg.__wbg_setvalue_89bc343a3a9d22c7=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_22155f5ec0698c61=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_37ef213b4ec4bb8a=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_f1ba50e7063b8f1e=function(n,e){b(n).value=e},o.wbg.__wbg_setvalue_24526afa65b94add=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_fb1afc30cdcbac47=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setvalue_0b5564c5cf1e19d3=function(n,e){b(n).value=e},o.wbg.__wbg_activeElement_7643feb97ae4b26c=function(n){var e=b(n).activeElement;return d(e)?0:c(e)},o.wbg.__wbg_is_e8ad5aa6da4b8c83=function(n,e){return Object.is(b(n),b(e))},o.wbg.__wbg_selectionStart_e307027925a4126c=M((function(n,e){var _=b(e).selectionStart;m()[n/4+1]=d(_)?0:_,m()[n/4+0]=!d(_)})),o.wbg.__wbg_selectionEnd_a60da3f21b8cd017=M((function(n,e){var _=b(e).selectionEnd;m()[n/4+1]=d(_)?0:_,m()[n/4+0]=!d(_)})),o.wbg.__wbg_setvalue_4f1af4fbd0b9942b=function(n,e,_){b(n).value=w(e,_)},o.wbg.__wbg_setselectionStart_43d78fa7e8fa80c1=M((function(n,e,_){b(n).selectionStart=0===e?void 0:_>>>0})),o.wbg.__wbg_setselectionEnd_e36952a0a88aadb0=M((function(n,e,_){b(n).selectionEnd=0===e?void 0:_>>>0})),o.wbg.__wbg_instanceof_HtmlMenuItemElement_e7e071f45a775374=function(n){return b(n)instanceof HTMLMenuItemElement},o.wbg.__wbg_setchecked_92a4301ee26a7c9f=function(n,e){b(n).checked=0!==e},o.wbg.__wbg_setchecked_1fe693236b4c24f3=function(n,e){b(n).checked=0!==e},o.wbg.__wbg_subarray_6b2dd31c84ee881f=function(n,e,_){return c(b(n).subarray(e>>>0,_>>>0))},o.wbg.__wbg_getRandomValues_57e4008f45f0e105=M((function(n,e){b(n).getRandomValues(b(e))})),o.wbg.__wbg_randomFillSync_d90848a552cbd666=M((function(n,e,_){var t,r;b(n).randomFillSync((t=e,r=_,g().subarray(t/1,t/1+r)))})),o.wbg.__wbg_length_e9f6f145de2fede5=function(n){return b(n).length},o.wbg.__wbg_new_22a33711cf65b661=function(n){return c(new Uint8Array(b(n)))},o.wbg.__wbg_set_b29de3f25280c6ec=function(n,e,_){b(n).set(b(e),_>>>0)},o.wbg.__wbindgen_throw=function(n,e){throw new Error(w(n,e))},o.wbg.__wbindgen_rethrow=function(n){throw a(n)},o.wbg.__wbg_then_b4358f6ec1ee6657=function(n,e){return c(b(n).then(b(e)))},o.wbg.__wbg_resolve_9b0f9ddf5f89cb1e=function(n){return c(Promise.resolve(b(n)))},o.wbg.__wbg_waitAsync_c504ab90b0b04d5e=function(){return c(Atomics.waitAsync)},o.wbg.__wbg_new_9aae23408b655f27=function(n){return c(new Int32Array(b(n)))},o.wbg.__wbg_waitAsync_af364d47a9d86b33=function(n,e,_){return c(Atomics.waitAsync(b(n),e,_))},o.wbg.__wbg_async_347af4fe614cf01e=function(n){return b(n).async},o.wbg.__wbg_value_610810d3f2b07a74=function(n){return c(b(n).value)},o.wbg.__wbg_of_1cea6608ba3fbce5=function(n,e,_){return c(Array.of(b(n),b(e),b(_)))},o.wbg.__wbg_data_b59dd3983cd7c614=function(n){return c(b(n).data)},o.wbg.__wbindgen_closure_wrapper293=function(n,e,_){return c(h(n,e,17,y))},o.wbg.__wbindgen_closure_wrapper295=function(n,e,_){return c(h(n,e,17,E))},o.wbg.__wbindgen_closure_wrapper297=function(n,_,t){return c(function(n,_,t,b){const r={a:n,b:_,cnt:1,dtor:t},c=(...n)=>{r.cnt++;try{return b(r.a,r.b,...n)}finally{0==--r.cnt&&(e.__wbindgen_export_3.get(r.dtor)(r.a,r.b),r.a=0,p.unregister(r))}};return c.original=r,p.register(c,r,r),c}(n,_,17,A))},o.wbg.__wbindgen_closure_wrapper571=function(n,e,_){return c(h(n,e,17,C))},o.wbg.__wbindgen_closure_wrapper1243=function(n,e,_){return c(h(n,e,17,S))},("string"==typeof t||"function"==typeof Request&&t instanceof Request||"function"==typeof URL&&t instanceof URL)&&(t=fetch(t));const{instance:i,module:u}=await async function(n,e,t){if("function"==typeof Response&&n instanceof Response){if(_=e.wbg.memory=new WebAssembly.Memory({initial:18,maximum:16384,shared:!0}),"function"==typeof WebAssembly.instantiateStreaming)try{return await WebAssembly.instantiateStreaming(n,e)}catch(e){if("application/wasm"==n.headers.get("Content-Type"))throw e;console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e)}const t=await n.arrayBuffer();return await WebAssembly.instantiate(t,e)}{_=e.wbg.memory=t;const b=await WebAssembly.instantiate(n,e);return b instanceof WebAssembly.Instance?{instance:b,module:n}:b}}(await t,o,r);return e=i.exports,n.__wbindgen_wasm_module=u,e.__wbindgen_start(),e}),n)}();