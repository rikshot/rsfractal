!function(){const n={};let _,e;const t=new Array(32);function i(n){return t[n]}t.fill(void 0),t.push(void 0,null,!0,!1);let r=t.length;function o(n){const _=i(n);return function(n){n<36||(t[n]=r,r=n)}(n),_}function c(n){r===t.length&&t.push(t.length+1);const _=r;return r=t[_],t[_]=n,_}let a=new TextDecoder("utf-8",{ignoreBOM:!0,fatal:!0});a.decode();let l=null;function u(){return null!==l&&l.buffer===_.__wbindgen_export_0.buffer||(l=new Uint8Array(_.__wbindgen_export_0.buffer)),l}function w(n,_){return a.decode(u().slice(n,n+_))}let f=0,b=new TextEncoder("utf-8");function g(n,_,e){if(void 0===e){const e=b.encode(n),t=_(e.length);return u().subarray(t,t+e.length).set(e),f=e.length,t}let t=n.length,i=_(t);const r=u();let o=0;for(;o<t;o++){const _=n.charCodeAt(o);if(_>127)break;r[i+o]=_}if(o!==t){0!==o&&(n=n.slice(o)),i=e(i,t,t=o+3*n.length),o+=function(n,_){const e=b.encode(n);return _.set(e),{read:n.length,written:e.length}}(n,u().subarray(i+o,i+t)).written}return f=o,i}function d(n){return null==n}let s=null;function m(){return null!==s&&s.buffer===_.__wbindgen_export_0.buffer||(s=new Int32Array(_.__wbindgen_export_0.buffer)),s}function v(n){_.__wbindgen_exn_store(c(n))}n.child_entry_point=function(n){_.child_entry_point(n)},n.main=function(){_.main()};class h{static __wrap(n){const _=Object.create(h.prototype);return _.ptr=n,_}free(){const n=this.ptr;this.ptr=0,_.__wbg_workerpool_free(n)}constructor(n){var e=_.workerpool_new(n);return h.__wrap(e)}}n.WorkerPool=h,self.wasm_bindgen=Object.assign((function n(t,r){if(void 0===t){let n;n=void 0===self.document?self.location.href:self.document.currentScript.src,t=n.replace(/\.js$/,"_bg.wasm")}let a;const l={wbg:{}};if(l.wbg.__wbindgen_object_drop_ref=function(n){o(n)},l.wbg.__wbindgen_cb_drop=function(n){const _=o(n).original;if(1==_.cnt--)return _.a=0,!0;return!1},l.wbg.__wbindgen_number_new=function(n){return c(n)},l.wbg.__wbindgen_jsval_eq=function(n,_){return i(n)===i(_)},l.wbg.__wbindgen_string_new=function(n,_){return c(w(n,_))},l.wbg.__wbindgen_object_clone_ref=function(n){return c(i(n))},l.wbg.__wbindgen_cb_forget=function(n){o(n)},l.wbg.__wbindgen_json_parse=function(n,_){return c(JSON.parse(w(n,_)))},l.wbg.__wbg_new_6379253ce630863f=function(n,_,e){try{return c(new ImageData(i(n),_,e))}catch(n){v(n)}},l.wbg.__wbg_new_59cb74e423758ede=function(){return c(new Error)},l.wbg.__wbg_stack_558ba5917b466edd=function(n,e){var t=g(i(e).stack,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__wbg_error_4bb6c2a97407129a=function(n,e){try{console.error(w(n,e))}finally{_.__wbindgen_free(n,e)}},l.wbg.__wbg_waitAsync_02f87cb33f7ea4f9=function(){return c(Atomics.waitAsync)},l.wbg.__wbindgen_is_undefined=function(n){return void 0===i(n)},l.wbg.__wbg_waitAsync_6976aa8142b0ec39=function(n,_,e){return c(Atomics.waitAsync(i(n),_,e))},l.wbg.__widl_f_error_1_=function(n){console.error(i(n))},l.wbg.__widl_f_log_1_=function(n){console.log(i(n))},l.wbg.__widl_instanceof_Window=function(n){return"undefined"!=typeof Window&&i(n)instanceof Window},l.wbg.__widl_f_new_with_str_sequence_and_options_Blob=function(n,_){try{return c(new Blob(i(n),i(_)))}catch(n){v(n)}},l.wbg.__widl_instanceof_CanvasRenderingContext2D=function(n){return i(n)instanceof CanvasRenderingContext2D},l.wbg.__widl_f_put_image_data_with_dirty_x_and_dirty_y_and_dirty_width_and_dirty_height_CanvasRenderingContext2D=function(n,_,e,t,r,o,c,a){try{i(n).putImageData(i(_),e,t,r,o,c,a)}catch(n){v(n)}},l.wbg.__widl_f_width_DOMRect=function(n){return i(n).width},l.wbg.__widl_f_height_DOMRect=function(n){return i(n).height},l.wbg.__widl_f_top_DOMRectReadOnly=function(n){return i(n).top},l.wbg.__widl_f_left_DOMRectReadOnly=function(n){return i(n).left},l.wbg.__widl_f_post_message_DedicatedWorkerGlobalScope=function(n,_){try{i(n).postMessage(i(_))}catch(n){v(n)}},l.wbg.__widl_f_create_element_Document=function(n,_,e){try{return c(i(n).createElement(w(_,e)))}catch(n){v(n)}},l.wbg.__widl_f_create_element_ns_Document=function(n,_,e,t,r){try{return c(i(n).createElementNS(0===_?void 0:w(_,e),w(t,r)))}catch(n){v(n)}},l.wbg.__widl_f_create_text_node_Document=function(n,_,e){return c(i(n).createTextNode(w(_,e)))},l.wbg.__widl_f_get_element_by_id_Document=function(n,_,e){var t=i(n).getElementById(w(_,e));return d(t)?0:c(t)},l.wbg.__widl_f_query_selector_Document=function(n,_,e){try{var t=i(n).querySelector(w(_,e));return d(t)?0:c(t)}catch(n){v(n)}},l.wbg.__widl_f_active_element_Document=function(n){var _=i(n).activeElement;return d(_)?0:c(_)},l.wbg.__widl_instanceof_Element=function(n){return i(n)instanceof Element},l.wbg.__widl_f_closest_Element=function(n,_,e){try{var t=i(n).closest(w(_,e));return d(t)?0:c(t)}catch(n){v(n)}},l.wbg.__widl_f_get_attribute_Element=function(n,e,t,r){var o=i(e).getAttribute(w(t,r)),c=d(o)?0:g(o,_.__wbindgen_malloc,_.__wbindgen_realloc),a=f;m()[n/4+1]=a,m()[n/4+0]=c},l.wbg.__widl_f_get_attribute_names_Element=function(n){return c(i(n).getAttributeNames())},l.wbg.__widl_f_get_bounding_client_rect_Element=function(n){return c(i(n).getBoundingClientRect())},l.wbg.__widl_f_remove_attribute_Element=function(n,_,e){try{i(n).removeAttribute(w(_,e))}catch(n){v(n)}},l.wbg.__widl_f_set_attribute_Element=function(n,_,e,t,r){try{i(n).setAttribute(w(_,e),w(t,r))}catch(n){v(n)}},l.wbg.__widl_f_namespace_uri_Element=function(n,e){var t=i(e).namespaceURI,r=d(t)?0:g(t,_.__wbindgen_malloc,_.__wbindgen_realloc),o=f;m()[n/4+1]=o,m()[n/4+0]=r},l.wbg.__widl_f_tag_name_Element=function(n,e){var t=g(i(e).tagName,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_id_Element=function(n,e){var t=g(i(e).id,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_instanceof_ErrorEvent=function(n){return i(n)instanceof ErrorEvent},l.wbg.__widl_f_message_ErrorEvent=function(n,e){var t=g(i(e).message,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_prevent_default_Event=function(n){i(n).preventDefault()},l.wbg.__widl_f_type_Event=function(n,e){var t=g(i(e).type,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_target_Event=function(n){var _=i(n).target;return d(_)?0:c(_)},l.wbg.__widl_f_add_event_listener_with_callback_EventTarget=function(n,_,e,t){try{i(n).addEventListener(w(_,e),i(t))}catch(n){v(n)}},l.wbg.__widl_f_remove_event_listener_with_callback_EventTarget=function(n,_,e,t){try{i(n).removeEventListener(w(_,e),i(t))}catch(n){v(n)}},l.wbg.__widl_instanceof_HTMLButtonElement=function(n){return i(n)instanceof HTMLButtonElement},l.wbg.__widl_f_value_HTMLButtonElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLButtonElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLCanvasElement=function(n){return i(n)instanceof HTMLCanvasElement},l.wbg.__widl_f_get_context_HTMLCanvasElement=function(n,_,e){try{var t=i(n).getContext(w(_,e));return d(t)?0:c(t)}catch(n){v(n)}},l.wbg.__widl_instanceof_HTMLDataElement=function(n){return i(n)instanceof HTMLDataElement},l.wbg.__widl_f_value_HTMLDataElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLDataElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLElement=function(n){return i(n)instanceof HTMLElement},l.wbg.__widl_f_focus_HTMLElement=function(n){try{i(n).focus()}catch(n){v(n)}},l.wbg.__widl_instanceof_HTMLInputElement=function(n){return i(n)instanceof HTMLInputElement},l.wbg.__widl_f_set_checked_HTMLInputElement=function(n,_){i(n).checked=0!==_},l.wbg.__widl_f_type_HTMLInputElement=function(n,e){var t=g(i(e).type,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_value_HTMLInputElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLInputElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_f_selection_start_HTMLInputElement=function(n,_){try{var e=i(_).selectionStart;m()[n/4+1]=d(e)?0:e,m()[n/4+0]=!d(e)}catch(n){v(n)}},l.wbg.__widl_f_set_selection_start_HTMLInputElement=function(n,_,e){try{i(n).selectionStart=0===_?void 0:e>>>0}catch(n){v(n)}},l.wbg.__widl_f_selection_end_HTMLInputElement=function(n,_){try{var e=i(_).selectionEnd;m()[n/4+1]=d(e)?0:e,m()[n/4+0]=!d(e)}catch(n){v(n)}},l.wbg.__widl_f_set_selection_end_HTMLInputElement=function(n,_,e){try{i(n).selectionEnd=0===_?void 0:e>>>0}catch(n){v(n)}},l.wbg.__widl_instanceof_HTMLLIElement=function(n){return i(n)instanceof HTMLLIElement},l.wbg.__widl_f_value_HTMLLIElement=function(n){return i(n).value},l.wbg.__widl_f_set_value_HTMLLIElement=function(n,_){i(n).value=_},l.wbg.__widl_instanceof_HTMLMenuItemElement=function(n){return i(n)instanceof HTMLMenuItemElement},l.wbg.__widl_f_set_checked_HTMLMenuItemElement=function(n,_){i(n).checked=0!==_},l.wbg.__widl_instanceof_HTMLMeterElement=function(n){return i(n)instanceof HTMLMeterElement},l.wbg.__widl_f_value_HTMLMeterElement=function(n){return i(n).value},l.wbg.__widl_f_set_value_HTMLMeterElement=function(n,_){i(n).value=_},l.wbg.__widl_instanceof_HTMLOptionElement=function(n){return i(n)instanceof HTMLOptionElement},l.wbg.__widl_f_value_HTMLOptionElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLOptionElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLOutputElement=function(n){return i(n)instanceof HTMLOutputElement},l.wbg.__widl_f_value_HTMLOutputElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLOutputElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLParamElement=function(n){return i(n)instanceof HTMLParamElement},l.wbg.__widl_f_value_HTMLParamElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLParamElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLProgressElement=function(n){return i(n)instanceof HTMLProgressElement},l.wbg.__widl_f_value_HTMLProgressElement=function(n){return i(n).value},l.wbg.__widl_f_set_value_HTMLProgressElement=function(n,_){i(n).value=_},l.wbg.__widl_instanceof_HTMLSelectElement=function(n){return i(n)instanceof HTMLSelectElement},l.wbg.__widl_f_value_HTMLSelectElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLSelectElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HTMLTextAreaElement=function(n){return i(n)instanceof HTMLTextAreaElement},l.wbg.__widl_f_value_HTMLTextAreaElement=function(n,e){var t=g(i(e).value,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_set_value_HTMLTextAreaElement=function(n,_,e){i(n).value=w(_,e)},l.wbg.__widl_instanceof_HashChangeEvent=function(n){return i(n)instanceof HashChangeEvent},l.wbg.__widl_f_new_url_HashChangeEvent=function(n,e){var t=g(i(e).newURL,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_push_state_with_url_History=function(n,_,e,t,r,o){try{i(n).pushState(i(_),w(e,t),0===r?void 0:w(r,o))}catch(n){v(n)}},l.wbg.__widl_f_href_Location=function(n,e){try{var t=g(i(e).href,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t}catch(n){v(n)}},l.wbg.__widl_instanceof_MessageEvent=function(n){return i(n)instanceof MessageEvent},l.wbg.__widl_f_data_MessageEvent=function(n){return c(i(n).data)},l.wbg.__widl_instanceof_MouseEvent=function(n){return i(n)instanceof MouseEvent},l.wbg.__widl_f_client_x_MouseEvent=function(n){return i(n).clientX},l.wbg.__widl_f_client_y_MouseEvent=function(n){return i(n).clientY},l.wbg.__widl_f_hardware_concurrency_Navigator=function(n){return i(n).hardwareConcurrency},l.wbg.__widl_instanceof_Node=function(n){return i(n)instanceof Node},l.wbg.__widl_f_append_child_Node=function(n,_){try{return c(i(n).appendChild(i(_)))}catch(n){v(n)}},l.wbg.__widl_f_insert_before_Node=function(n,_,e){try{return c(i(n).insertBefore(i(_),i(e)))}catch(n){v(n)}},l.wbg.__widl_f_remove_child_Node=function(n,_){try{return c(i(n).removeChild(i(_)))}catch(n){v(n)}},l.wbg.__widl_f_replace_child_Node=function(n,_,e){try{return c(i(n).replaceChild(i(_),i(e)))}catch(n){v(n)}},l.wbg.__widl_f_node_type_Node=function(n){return i(n).nodeType},l.wbg.__widl_f_child_nodes_Node=function(n){return c(i(n).childNodes)},l.wbg.__widl_f_first_child_Node=function(n){var _=i(n).firstChild;return d(_)?0:c(_)},l.wbg.__widl_f_next_sibling_Node=function(n){var _=i(n).nextSibling;return d(_)?0:c(_)},l.wbg.__widl_f_text_content_Node=function(n,e){var t=i(e).textContent,r=d(t)?0:g(t,_.__wbindgen_malloc,_.__wbindgen_realloc),o=f;m()[n/4+1]=o,m()[n/4+0]=r},l.wbg.__widl_f_set_text_content_Node=function(n,_,e){i(n).textContent=0===_?void 0:w(_,e)},l.wbg.__widl_f_get_NodeList=function(n,_){var e=i(n)[_>>>0];return d(e)?0:c(e)},l.wbg.__widl_f_length_NodeList=function(n){return i(n).length},l.wbg.__widl_f_now_Performance=function(n){return i(n).now()},l.wbg.__widl_instanceof_PopStateEvent=function(n){return i(n)instanceof PopStateEvent},l.wbg.__widl_f_state_PopStateEvent=function(n){return c(i(n).state)},l.wbg.__widl_f_new_URL=function(n,_){try{return c(new URL(w(n,_)))}catch(n){v(n)}},l.wbg.__widl_f_new_with_base_URL=function(n,_,e,t){try{return c(new URL(w(n,_),w(e,t)))}catch(n){v(n)}},l.wbg.__widl_f_create_object_url_with_blob_URL=function(n,e){try{var t=g(URL.createObjectURL(i(e)),_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t}catch(n){v(n)}},l.wbg.__widl_f_pathname_URL=function(n,e){var t=g(i(e).pathname,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_search_URL=function(n,e){var t=g(i(e).search,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_hash_URL=function(n,e){var t=g(i(e).hash,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_cancel_animation_frame_Window=function(n,_){try{i(n).cancelAnimationFrame(_)}catch(n){v(n)}},l.wbg.__widl_f_request_animation_frame_Window=function(n,_){try{return i(n).requestAnimationFrame(i(_))}catch(n){v(n)}},l.wbg.__widl_f_document_Window=function(n){var _=i(n).document;return d(_)?0:c(_)},l.wbg.__widl_f_location_Window=function(n){return c(i(n).location)},l.wbg.__widl_f_history_Window=function(n){try{return c(i(n).history)}catch(n){v(n)}},l.wbg.__widl_f_navigator_Window=function(n){return c(i(n).navigator)},l.wbg.__widl_f_performance_Window=function(n){var _=i(n).performance;return d(_)?0:c(_)},l.wbg.__widl_f_new_Worker=function(n,_){try{return c(new Worker(w(n,_)))}catch(n){v(n)}},l.wbg.__widl_f_post_message_Worker=function(n,_){try{i(n).postMessage(i(_))}catch(n){v(n)}},l.wbg.__widl_f_set_onmessage_Worker=function(n,_){i(n).onmessage=i(_)},l.wbg.__widl_f_set_onerror_Worker=function(n,_){i(n).onerror=i(_)},l.wbg.__widl_f_location_WorkerGlobalScope=function(n){return c(i(n).location)},l.wbg.__widl_f_origin_WorkerLocation=function(n,e){var t=g(i(e).origin,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__widl_f_pathname_WorkerLocation=function(n,e){var t=g(i(e).pathname,_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__wbg_call_12b949cfc461d154=function(n,_){try{return c(i(n).call(i(_)))}catch(n){v(n)}},l.wbg.__wbg_globalThis_22e06d4bea0084e3=function(){try{return c(globalThis.globalThis)}catch(n){v(n)}},l.wbg.__wbg_self_00b0599bca667294=function(){try{return c(self.self)}catch(n){v(n)}},l.wbg.__wbg_window_aa795c5aad79b8ac=function(){try{return c(window.window)}catch(n){v(n)}},l.wbg.__wbg_global_cc239dc2303f417c=function(){try{return c(global.global)}catch(n){v(n)}},l.wbg.__wbg_newnoargs_c4b2cbbd30e2d057=function(n,_){return c(new Function(w(n,_)))},l.wbg.__wbg_encodeURIComponent_59a1fe6e25aed151=function(n,_){return c(encodeURIComponent(w(n,_)))},l.wbg.__wbg_new_3c32f9cd3d7f4595=function(){return c(new Array)},l.wbg.__wbg_forEach_bd7728906f61e603=function(n,e,t){try{var r={a:e,b:t};i(n).forEach((n,e,t)=>{const i=r.a;r.a=0;try{return function(n,e,t,i,r){_.wasm_bindgen__convert__closures__invoke3_mut__h8df0e5d3bc580a90(n,e,c(t),i,c(r))}(i,r.b,n,e,t)}finally{r.a=i}})}finally{r.a=r.b=0}},l.wbg.__wbg_of_564adb751c7ef21f=function(n,_,e){return c(Array.of(i(n),i(_),i(e)))},l.wbg.__wbg_push_446cc0334a2426e8=function(n,_){return i(n).push(i(_))},l.wbg.__wbg_call_ce7cf17fc6380443=function(n,_,e){try{return c(i(n).call(i(_),i(e)))}catch(n){v(n)}},l.wbg.__wbg_is_4ef97be877c257d0=function(n,_){return Object.is(i(n),i(_))},l.wbg.__wbg_new_7dd9b384a913884d=function(){return c(new Object)},l.wbg.__wbg_new_d3eff62d5c013634=function(n,e){try{var t={a:n,b:e},i=new Promise((n,e)=>{const i=t.a;t.a=0;try{return function(n,e,t,i){_.wasm_bindgen__convert__closures__invoke2_mut__h610af1614dd5de5e(n,e,c(t),c(i))}(i,t.b,n,e)}finally{t.a=i}});return c(i)}finally{t.a=t.b=0}},l.wbg.__wbg_resolve_6885947099a907d3=function(n){return c(Promise.resolve(i(n)))},l.wbg.__wbg_then_b6fef331fde5cf0a=function(n,_){return c(i(n).then(i(_)))},l.wbg.__wbg_then_7d828a330efec051=function(n,_,e){return c(i(n).then(i(_),i(e)))},l.wbg.__wbg_buffer_1bb127df6348017b=function(n){return c(i(n).buffer)},l.wbg.__wbg_new_5b9ab22ded991deb=function(n){return c(new Uint8ClampedArray(i(n)))},l.wbg.__wbg_slice_1c8e641aca089885=function(n,_,e){return c(i(n).slice(_>>>0,e>>>0))},l.wbg.__wbg_set_8d5fd23e838df6b0=function(n,_,e){try{return Reflect.set(i(n),i(_),i(e))}catch(n){v(n)}},l.wbg.__wbindgen_string_get=function(n,e){const t=i(e);var r="string"==typeof t?t:void 0,o=d(r)?0:g(r,_.__wbindgen_malloc,_.__wbindgen_realloc),c=f;m()[n/4+1]=c,m()[n/4+0]=o},l.wbg.__wbindgen_debug_string=function(n,e){var t=g(function n(_){const e=typeof _;if("number"==e||"boolean"==e||null==_)return`${_}`;if("string"==e)return`"${_}"`;if("symbol"==e){const n=_.description;return null==n?"Symbol":`Symbol(${n})`}if("function"==e){const n=_.name;return"string"==typeof n&&n.length>0?`Function(${n})`:"Function"}if(Array.isArray(_)){const e=_.length;let t="[";e>0&&(t+=n(_[0]));for(let i=1;i<e;i++)t+=", "+n(_[i]);return t+="]",t}const t=/\[object ([^\]]+)\]/.exec(toString.call(_));let i;if(!(t.length>1))return toString.call(_);if(i=t[1],"Object"==i)try{return"Object("+JSON.stringify(_)+")"}catch(n){return"Object"}return _ instanceof Error?`${_.name}: ${_.message}\n${_.stack}`:i}(i(e)),_.__wbindgen_malloc,_.__wbindgen_realloc),r=f;m()[n/4+1]=r,m()[n/4+0]=t},l.wbg.__wbindgen_throw=function(n,_){throw new Error(w(n,_))},l.wbg.__wbindgen_rethrow=function(n){throw o(n)},l.wbg.__wbindgen_module=function(){return c(n.__wbindgen_wasm_module)},l.wbg.__wbindgen_memory=function(){return c(_.__wbindgen_export_0)},l.wbg.__wbindgen_closure_wrapper263=function(n,e,t){const i={a:n,b:e,cnt:1},r=n=>{i.cnt++;const e=i.a;i.a=0;try{return function(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hadc78083feccfa9e(n,e,c(t))}(e,i.b,n)}finally{0==--i.cnt?_.__wbindgen_export_3.get(42)(e,i.b):i.a=e}};return r.original=i,c(r)},l.wbg.__wbindgen_closure_wrapper264=function(n,e,t){const i={a:n,b:e,cnt:1},r=n=>{i.cnt++;const e=i.a;i.a=0;try{return function(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0731274cf485881c(n,e,t)}(e,i.b,n)}finally{0==--i.cnt?_.__wbindgen_export_3.get(42)(e,i.b):i.a=e}};return r.original=i,c(r)},l.wbg.__wbindgen_closure_wrapper631=function(n,e,t){const i={a:n,b:e,cnt:1},r=n=>{i.cnt++;const e=i.a;i.a=0;try{return function(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4db7b90d45fe4677(n,e,c(t))}(e,i.b,n)}finally{0==--i.cnt?_.__wbindgen_export_3.get(193)(e,i.b):i.a=e}};return r.original=i,c(r)},l.wbg.__wbindgen_closure_wrapper629=function(n,e,t){const i={a:n,b:e,cnt:1},r=n=>{i.cnt++;const e=i.a;i.a=0;try{return function(n,e,t){_._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h4db7b90d45fe4677(n,e,c(t))}(e,i.b,n)}finally{0==--i.cnt?_.__wbindgen_export_3.get(193)(e,i.b):i.a=e}};return r.original=i,c(r)},"function"==typeof URL&&t instanceof URL||"string"==typeof t||"function"==typeof Request&&t instanceof Request){e=l.wbg.memory=new WebAssembly.Memory({initial:23,maximum:16384,shared:!0});const n=fetch(t);a="function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(n,l).catch(_=>n.then(n=>{if("application/wasm"!=n.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",_),n.arrayBuffer();throw _}).then(n=>WebAssembly.instantiate(n,l))):n.then(n=>n.arrayBuffer()).then(n=>WebAssembly.instantiate(n,l))}else e=l.wbg.memory=r,a=WebAssembly.instantiate(t,l).then(n=>n instanceof WebAssembly.Instance?{instance:n,module:t}:n);return a.then(({instance:e,module:t})=>(_=e.exports,n.__wbindgen_wasm_module=t,_.__wbindgen_start(),_))}),n)}();