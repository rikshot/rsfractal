(function() {
    const __exports = {};
    let wasm;

    let memory;

    const heap = new Array(32);

    heap.fill(undefined);

    heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.__wbindgen_export_0.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.__wbindgen_export_0.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().slice(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
};

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.__wbindgen_export_0.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.__wbindgen_export_0.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}
function __wbg_adapter_32(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9c1dc7f8562e1ab7(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_35(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h9c1dc7f8562e1ab7(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_38(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0731274cf485881c(arg0, arg1, arg2);
}

function __wbg_adapter_41(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h637a17bbb60e7ff4(arg0, arg1, addHeapObject(arg2));
}

/**
*/
__exports.main = function() {
    wasm.main();
};

/**
* Entry point invoked by `worker.js`, a bit of a hack but see the \"TODO\" above
* about `worker.js` in general.
* @param {number} ptr
*/
__exports.child_entry_point = function(ptr) {
    wasm.child_entry_point(ptr);
};

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}
function __wbg_adapter_283(arg0, arg1, arg2, arg3, arg4) {
    wasm.wasm_bindgen__convert__closures__invoke3_mut__h8df0e5d3bc580a90(arg0, arg1, addHeapObject(arg2), arg3, addHeapObject(arg4));
}

function __wbg_adapter_300(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures__invoke2_mut__h610af1614dd5de5e(arg0, arg1, addHeapObject(arg2), addHeapObject(arg3));
}

/**
*/
class WorkerPool {

    static __wrap(ptr) {
        const obj = Object.create(WorkerPool.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_workerpool_free(ptr);
    }
    /**
    * Creates a new `WorkerPool` which immediately creates `initial` workers.
    *
    * The pool created here can be used over a long period of time, and it
    * will be initially primed with `initial` workers. Currently workers are
    * never released or gc\'d until the whole pool is destroyed.
    *
    * # Errors
    *
    * Returns any error that may happen while a JS web worker is created and a
    * message is sent to it.
    * @param {number} initial
    */
    constructor(initial) {
        var ret = wasm.workerpool_new(initial);
        return WorkerPool.__wrap(ret);
    }
}
__exports.WorkerPool = WorkerPool;

function init(module, maybe_memory) {
    if (typeof module === 'undefined') {
        let src;
        if (self.document === undefined) {
            src = self.location.href;
        } else {
            src = self.document.currentScript.src;
        }
        module = src.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_forget = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        var ret = JSON.parse(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
        var ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
        var ret = getObject(arg0) === getObject(arg1);
        return ret;
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        try {
            console.error(getStringFromWasm0(arg0, arg1));
        } finally {
            wasm.__wbindgen_free(arg0, arg1);
        }
    };
    imports.wbg.__wbg_waitAsync_02f87cb33f7ea4f9 = function() {
        var ret = Atomics.waitAsync;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbg_waitAsync_6976aa8142b0ec39 = function(arg0, arg1, arg2) {
        var ret = Atomics.waitAsync(getObject(arg0), arg1, arg2);
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_error_1_ = function(arg0) {
        console.error(getObject(arg0));
    };
    imports.wbg.__widl_f_log_1_ = function(arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__widl_instanceof_Window = function(arg0) {
        var ret = typeof Window !== "undefined" && getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__widl_f_new_with_str_sequence_and_options_Blob = function(arg0, arg1) {
        try {
            var ret = new Blob(getObject(arg0), getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_CanvasRenderingContext2D = function(arg0) {
        var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
        return ret;
    };
    imports.wbg.__widl_f_set_fill_style_CanvasRenderingContext2D = function(arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__widl_f_fill_rect_CanvasRenderingContext2D = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__widl_f_width_DOMRect = function(arg0) {
        var ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__widl_f_height_DOMRect = function(arg0) {
        var ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__widl_f_top_DOMRectReadOnly = function(arg0) {
        var ret = getObject(arg0).top;
        return ret;
    };
    imports.wbg.__widl_f_left_DOMRectReadOnly = function(arg0) {
        var ret = getObject(arg0).left;
        return ret;
    };
    imports.wbg.__widl_f_post_message_DedicatedWorkerGlobalScope = function(arg0, arg1) {
        try {
            getObject(arg0).postMessage(getObject(arg1));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_element_Document = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).createElement(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_element_ns_Document = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            var ret = getObject(arg0).createElementNS(arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_text_node_Document = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).createTextNode(getStringFromWasm0(arg1, arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_query_selector_Document = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).querySelector(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_active_element_Document = function(arg0) {
        var ret = getObject(arg0).activeElement;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_instanceof_Element = function(arg0) {
        var ret = getObject(arg0) instanceof Element;
        return ret;
    };
    imports.wbg.__widl_f_closest_Element = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).closest(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_get_attribute_Element = function(arg0, arg1, arg2, arg3) {
        var ret = getObject(arg1).getAttribute(getStringFromWasm0(arg2, arg3));
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_get_attribute_names_Element = function(arg0) {
        var ret = getObject(arg0).getAttributeNames();
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_get_bounding_client_rect_Element = function(arg0) {
        var ret = getObject(arg0).getBoundingClientRect();
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_remove_attribute_Element = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).removeAttribute(getStringFromWasm0(arg1, arg2));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4) {
        try {
            getObject(arg0).setAttribute(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_namespace_uri_Element = function(arg0, arg1) {
        var ret = getObject(arg1).namespaceURI;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_tag_name_Element = function(arg0, arg1) {
        var ret = getObject(arg1).tagName;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_id_Element = function(arg0, arg1) {
        var ret = getObject(arg1).id;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_instanceof_ErrorEvent = function(arg0) {
        var ret = getObject(arg0) instanceof ErrorEvent;
        return ret;
    };
    imports.wbg.__widl_f_message_ErrorEvent = function(arg0, arg1) {
        var ret = getObject(arg1).message;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_prevent_default_Event = function(arg0) {
        getObject(arg0).preventDefault();
    };
    imports.wbg.__widl_f_type_Event = function(arg0, arg1) {
        var ret = getObject(arg1).type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_target_Event = function(arg0) {
        var ret = getObject(arg0).target;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3) {
        try {
            getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_remove_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3) {
        try {
            getObject(arg0).removeEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLButtonElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLButtonElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLButtonElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLCanvasElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        return ret;
    };
    imports.wbg.__widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLDataElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLDataElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLDataElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLElement;
        return ret;
    };
    imports.wbg.__widl_f_focus_HTMLElement = function(arg0) {
        try {
            getObject(arg0).focus();
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLInputElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLInputElement;
        return ret;
    };
    imports.wbg.__widl_f_set_checked_HTMLInputElement = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__widl_f_type_HTMLInputElement = function(arg0, arg1) {
        var ret = getObject(arg1).type;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_set_value_HTMLInputElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_f_selection_start_HTMLInputElement = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).selectionStart;
            getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_set_selection_start_HTMLInputElement = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).selectionStart = arg1 === 0 ? undefined : arg2 >>> 0;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_selection_end_HTMLInputElement = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).selectionEnd;
            getInt32Memory0()[arg0 / 4 + 1] = isLikeNone(ret) ? 0 : ret;
            getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_set_selection_end_HTMLInputElement = function(arg0, arg1, arg2) {
        try {
            getObject(arg0).selectionEnd = arg1 === 0 ? undefined : arg2 >>> 0;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_HTMLLIElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLLIElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLLIElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_instanceof_HTMLMenuItemElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLMenuItemElement;
        return ret;
    };
    imports.wbg.__widl_f_set_checked_HTMLMenuItemElement = function(arg0, arg1) {
        getObject(arg0).checked = arg1 !== 0;
    };
    imports.wbg.__widl_instanceof_HTMLMeterElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLMeterElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLMeterElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_instanceof_HTMLOptionElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLOptionElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLOptionElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLOutputElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLOutputElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLOutputElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLParamElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLParamElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLParamElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLProgressElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLProgressElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLProgressElement = function(arg0, arg1) {
        getObject(arg0).value = arg1;
    };
    imports.wbg.__widl_instanceof_HTMLSelectElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLSelectElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLSelectElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HTMLTextAreaElement = function(arg0) {
        var ret = getObject(arg0) instanceof HTMLTextAreaElement;
        return ret;
    };
    imports.wbg.__widl_f_set_value_HTMLTextAreaElement = function(arg0, arg1, arg2) {
        getObject(arg0).value = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_instanceof_HashChangeEvent = function(arg0) {
        var ret = getObject(arg0) instanceof HashChangeEvent;
        return ret;
    };
    imports.wbg.__widl_f_new_url_HashChangeEvent = function(arg0, arg1) {
        var ret = getObject(arg1).newURL;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_push_state_with_url_History = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        try {
            getObject(arg0).pushState(getObject(arg1), getStringFromWasm0(arg2, arg3), arg4 === 0 ? undefined : getStringFromWasm0(arg4, arg5));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_href_Location = function(arg0, arg1) {
        try {
            var ret = getObject(arg1).href;
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_instanceof_MessageEvent = function(arg0) {
        var ret = getObject(arg0) instanceof MessageEvent;
        return ret;
    };
    imports.wbg.__widl_f_data_MessageEvent = function(arg0) {
        var ret = getObject(arg0).data;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_instanceof_MouseEvent = function(arg0) {
        var ret = getObject(arg0) instanceof MouseEvent;
        return ret;
    };
    imports.wbg.__widl_f_client_x_MouseEvent = function(arg0) {
        var ret = getObject(arg0).clientX;
        return ret;
    };
    imports.wbg.__widl_f_client_y_MouseEvent = function(arg0) {
        var ret = getObject(arg0).clientY;
        return ret;
    };
    imports.wbg.__widl_f_hardware_concurrency_Navigator = function(arg0) {
        var ret = getObject(arg0).hardwareConcurrency;
        return ret;
    };
    imports.wbg.__widl_instanceof_Node = function(arg0) {
        var ret = getObject(arg0) instanceof Node;
        return ret;
    };
    imports.wbg.__widl_f_append_child_Node = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).appendChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_insert_before_Node = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_remove_child_Node = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).removeChild(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_replace_child_Node = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_node_type_Node = function(arg0) {
        var ret = getObject(arg0).nodeType;
        return ret;
    };
    imports.wbg.__widl_f_child_nodes_Node = function(arg0) {
        var ret = getObject(arg0).childNodes;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_first_child_Node = function(arg0) {
        var ret = getObject(arg0).firstChild;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_next_sibling_Node = function(arg0) {
        var ret = getObject(arg0).nextSibling;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_text_content_Node = function(arg0, arg1) {
        var ret = getObject(arg1).textContent;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_set_text_content_Node = function(arg0, arg1, arg2) {
        getObject(arg0).textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__widl_f_get_NodeList = function(arg0, arg1) {
        var ret = getObject(arg0)[arg1 >>> 0];
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_length_NodeList = function(arg0) {
        var ret = getObject(arg0).length;
        return ret;
    };
    imports.wbg.__widl_f_now_Performance = function(arg0) {
        var ret = getObject(arg0).now();
        return ret;
    };
    imports.wbg.__widl_instanceof_PopStateEvent = function(arg0) {
        var ret = getObject(arg0) instanceof PopStateEvent;
        return ret;
    };
    imports.wbg.__widl_f_state_PopStateEvent = function(arg0) {
        var ret = getObject(arg0).state;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_new_URL = function(arg0, arg1) {
        try {
            var ret = new URL(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_new_with_base_URL = function(arg0, arg1, arg2, arg3) {
        try {
            var ret = new URL(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_create_object_url_with_blob_URL = function(arg0, arg1) {
        try {
            var ret = URL.createObjectURL(getObject(arg1));
            var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len0 = WASM_VECTOR_LEN;
            getInt32Memory0()[arg0 / 4 + 1] = len0;
            getInt32Memory0()[arg0 / 4 + 0] = ptr0;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_pathname_URL = function(arg0, arg1) {
        var ret = getObject(arg1).pathname;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_search_URL = function(arg0, arg1) {
        var ret = getObject(arg1).search;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_hash_URL = function(arg0, arg1) {
        var ret = getObject(arg1).hash;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__widl_f_cancel_animation_frame_Window = function(arg0, arg1) {
        try {
            getObject(arg0).cancelAnimationFrame(arg1);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_request_animation_frame_Window = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
            return ret;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_document_Window = function(arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_location_Window = function(arg0) {
        var ret = getObject(arg0).location;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_history_Window = function(arg0) {
        try {
            var ret = getObject(arg0).history;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_navigator_Window = function(arg0) {
        var ret = getObject(arg0).navigator;
        return addHeapObject(ret);
    };
    imports.wbg.__widl_f_performance_Window = function(arg0) {
        var ret = getObject(arg0).performance;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__widl_f_new_Worker = function(arg0, arg1) {
        try {
            var ret = new Worker(getStringFromWasm0(arg0, arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_post_message_Worker = function(arg0, arg1) {
        try {
            getObject(arg0).postMessage(getObject(arg1));
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__widl_f_set_onmessage_Worker = function(arg0, arg1) {
        getObject(arg0).onmessage = getObject(arg1);
    };
    imports.wbg.__widl_f_set_onerror_Worker = function(arg0, arg1) {
        getObject(arg0).onerror = getObject(arg1);
    };
    imports.wbg.__wbg_call_12b949cfc461d154 = function(arg0, arg1) {
        try {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_globalThis_22e06d4bea0084e3 = function() {
        try {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_self_00b0599bca667294 = function() {
        try {
            var ret = self.self;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_window_aa795c5aad79b8ac = function() {
        try {
            var ret = window.window;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_global_cc239dc2303f417c = function() {
        try {
            var ret = global.global;
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_newnoargs_c4b2cbbd30e2d057 = function(arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_encodeURIComponent_59a1fe6e25aed151 = function(arg0, arg1) {
        var ret = encodeURIComponent(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_3c32f9cd3d7f4595 = function() {
        var ret = new Array();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_forEach_bd7728906f61e603 = function(arg0, arg1, arg2) {
        try {
            var state0 = {a: arg1, b: arg2};
            var cb0 = (arg0, arg1, arg2) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_283(a, state0.b, arg0, arg1, arg2);
                } finally {
                    state0.a = a;
                }
            };
            getObject(arg0).forEach(cb0);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_of_564adb751c7ef21f = function(arg0, arg1, arg2) {
        var ret = Array.of(getObject(arg0), getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_push_446cc0334a2426e8 = function(arg0, arg1) {
        var ret = getObject(arg0).push(getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_call_ce7cf17fc6380443 = function(arg0, arg1, arg2) {
        try {
            var ret = getObject(arg0).call(getObject(arg1), getObject(arg2));
            return addHeapObject(ret);
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbg_is_4ef97be877c257d0 = function(arg0, arg1) {
        var ret = Object.is(getObject(arg0), getObject(arg1));
        return ret;
    };
    imports.wbg.__wbg_new_7dd9b384a913884d = function() {
        var ret = new Object();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_new_d3eff62d5c013634 = function(arg0, arg1) {
        try {
            var state0 = {a: arg0, b: arg1};
            var cb0 = (arg0, arg1) => {
                const a = state0.a;
                state0.a = 0;
                try {
                    return __wbg_adapter_300(a, state0.b, arg0, arg1);
                } finally {
                    state0.a = a;
                }
            };
            var ret = new Promise(cb0);
            return addHeapObject(ret);
        } finally {
            state0.a = state0.b = 0;
        }
    };
    imports.wbg.__wbg_resolve_6885947099a907d3 = function(arg0) {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_b6fef331fde5cf0a = function(arg0, arg1) {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_7d828a330efec051 = function(arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_buffer_1bb127df6348017b = function(arg0) {
        var ret = getObject(arg0).buffer;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_set_8d5fd23e838df6b0 = function(arg0, arg1, arg2) {
        try {
            var ret = Reflect.set(getObject(arg0), getObject(arg1), getObject(arg2));
            return ret;
        } catch (e) {
            handleError(e)
        }
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
        const obj = getObject(arg1);
        var ret = typeof(obj) === 'string' ? obj : undefined;
        var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_module = function() {
        var ret = init.__wbindgen_wasm_module;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_memory = function() {
        var ret = wasm.__wbindgen_export_0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper555 = function(arg0, arg1, arg2) {

        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_adapter_32(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbindgen_export_3.get(170)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        var ret = real;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper355 = function(arg0, arg1, arg2) {

        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_adapter_38(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbindgen_export_3.get(81)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        var ret = real;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper557 = function(arg0, arg1, arg2) {

        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_adapter_35(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbindgen_export_3.get(170)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        var ret = real;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper356 = function(arg0, arg1, arg2) {

        const state = { a: arg0, b: arg1, cnt: 1 };
        const real = (arg0) => {
            state.cnt++;
            const a = state.a;
            state.a = 0;
            try {
                return __wbg_adapter_41(a, state.b, arg0);
            } finally {
                if (--state.cnt === 0) wasm.__wbindgen_export_3.get(81)(a, state.b);
                else state.a = a;
            }
        }
        ;
        real.original = state;
        var ret = real;
        return addHeapObject(ret);
    };

    if ((typeof URL === 'function' && module instanceof URL) || typeof module === 'string' || (typeof Request === 'function' && module instanceof Request)) {
        memory = imports.wbg.memory = new WebAssembly.Memory({initial:18,maximum:16384,shared:true});
        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                return response
                .then(r => {
                    if (r.headers.get('Content-Type') != 'application/wasm') {
                        console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                        return r.arrayBuffer();
                    } else {
                        throw e;
                    }
                })
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {
        memory = imports.wbg.memory = maybe_memory;
        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;
        wasm.__wbindgen_start();
        return wasm;
    });
}

self.wasm_bindgen = Object.assign(init, __exports);

})();
