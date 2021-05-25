import * as __SNOWPACK_ENV__ from '../../../_snowpack/env.js';
import.meta.env = __SNOWPACK_ENV__;


let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm0(arg, malloc) {
    const ptr = malloc(arg.length * 1);
    getUint8Memory0().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function notDefined(what) { return () => { throw new Error(`${what} is not defined`); }; }
/**
*/
export class Chip8 {

    static __wrap(ptr) {
        const obj = Object.create(Chip8.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_chip8_free(ptr);
    }
    /**
    * @returns {Display}
    */
    get display() {
        var ret = wasm.__wbg_get_chip8_display(this.ptr);
        return Display.__wrap(ret);
    }
    /**
    * @param {Display} arg0
    */
    set display(arg0) {
        _assertClass(arg0, Display);
        var ptr0 = arg0.ptr;
        arg0.ptr = 0;
        wasm.__wbg_set_chip8_display(this.ptr, ptr0);
    }
    /**
    * @param {Uint8Array} data
    * @returns {Chip8}
    */
    static new(data) {
        var ptr0 = passArray8ToWasm0(data, wasm.__wbindgen_malloc);
        var len0 = WASM_VECTOR_LEN;
        var ret = wasm.chip8_new(ptr0, len0);
        return Chip8.__wrap(ret);
    }
    /**
    * @param {number} key
    */
    handle_key_down(key) {
        wasm.chip8_handle_key_down(this.ptr, key);
    }
    /**
    * @param {number} key
    */
    handle_key_up(key) {
        wasm.chip8_handle_key_up(this.ptr, key);
    }
    /**
    */
    decrement_timers() {
        wasm.chip8_decrement_timers(this.ptr);
    }
    /**
    * @param {number} delta
    */
    cycle(delta) {
        wasm.chip8_cycle(this.ptr, delta);
    }
}
/**
*/
export class Display {

    static __wrap(ptr) {
        const obj = Object.create(Display.prototype);
        obj.ptr = ptr;

        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.ptr;
        this.ptr = 0;

        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_display_free(ptr);
    }
    /**
    * @returns {number}
    */
    get_ptr() {
        var ret = wasm.display_get_ptr(this.ptr);
        return ret;
    }
    /**
    * @returns {boolean}
    */
    should_update() {
        var ret = wasm.display_should_update(this.ptr);
        return ret !== 0;
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('index_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_random_29218b0f217b2697 = typeof Math.random == 'function' ? Math.random : notDefined('Math.random');
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };

    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }



    const { instance, module } = await load(await input, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;

    return wasm;
}

export default init;

