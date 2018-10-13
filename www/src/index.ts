"use strict";

import * as Event from "./event";
import * as RecvOpcode from "./recv_opcode";
import * as SendOpcode from "./send_opcode";
const webgl_test = import("./webgl_test");

//////// Globals     ////////
let gl: WebGL2RenderingContext;
//////// End globals ////////

export function log(msg: string): void {
    console.log(msg);
}

export function create_shader_sys(type: number,
    src: string): WebGLShader | undefined {
    const shader = gl.createShader(type);
    if (shader === null) {
        return undefined;
    }
    gl.shaderSource(shader, src);
    gl.compileShader(shader);

    const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (success) {
        return shader;
    }

    log(`${gl.getShaderInfoLog(shader)}`);
    gl.deleteShader(shader);

    return undefined;
}

export function create_program(vertex_shader: WebGLShader,
    fragment_shader: WebGLShader): WebGLProgram |
    undefined {
    const program = gl.createProgram();
    if (program === null) {
        return undefined;
    }
    gl.attachShader(program, vertex_shader);
    gl.attachShader(program, fragment_shader);
    gl.linkProgram(program);

    const success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) {
        return program;
    }

    log(`${gl.getProgramInfoLog(program)}`);
    gl.deleteProgram(program);

    return undefined;
}

export function get_uniform_location(prog: WebGLProgram,
    name: string): WebGLUniformLocation |
    undefined {
    const uni_loc = gl.getUniformLocation(prog, name);

    return uni_loc === null ? undefined : uni_loc;
}

export function create_buffer(): WebGLBuffer {
    const buffer = gl.createBuffer();
    if (buffer === null) {
        throw new Error("WebGLRenderingContext.createBuffer() failed");
    }

    return buffer;
}

export function get_attr_location(prog: WebGLProgram, name: string): number {
    return gl.getAttribLocation(prog, name);
}

export function bind_buffer_sys(target: number, buffer: WebGLBuffer): void {
    gl.bindBuffer(target, buffer);
}

export function buffer_data_sys(target: number,
    src_data: Uint8Array,
    usage: number,
    src_offset: number,
    length: number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function buffer_data_u16_sys(target: number,
    src_data: Uint16Array,
    usage: number,
    src_offset: number,
    length: number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function buffer_data_f32_sys(target: number,
    src_data: Float32Array,
    usage: number,
    src_offset: number,
    length: number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function create_vertex_array(): WebGLVertexArrayObject {
    const vao = gl.createVertexArray();
    if (vao === null) {
        throw new Error("Failed to create a VAO");
    }

    return vao;
}

export function bind_vertex_array(vao: WebGLVertexArrayObject): void {
    gl.bindVertexArray(vao);
}

export function enable_vertex_attr_array(index: number): void {
    gl.enableVertexAttribArray(index);
}

export function vertex_attr_ptr_sys(index: number,
    size: number,
    data_type: number,
    normalized: boolean,
    stride: number,
    offset: number): void {
    gl.vertexAttribPointer(index, size, data_type, normalized, stride, offset);
}

export function get_canvas_width(): number {
    return gl.canvas.width;
}

export function get_canvas_height(): number {
    return gl.canvas.height;
}

export function resize_canvas_to_display(): void {
    if (
        gl.canvas.width !== gl.canvas.clientWidth ||
        gl.canvas.height !== gl.canvas.clientHeight
    ) {
        gl.canvas.width = gl.canvas.clientWidth;
        gl.canvas.height = gl.canvas.clientHeight;
    }
}

export function reset_viewport(): void {
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
}

export function clear_color(r: number, g: number, b: number, a: number): void {
    gl.clearColor(r, g, b, a);
}

export function clear(mask: number): void {
    gl.clear(mask);
}

export function use_program(prog: WebGLProgram): void {
    gl.useProgram(prog);
}

export function draw_arrays_sys(mode: number,
    first: number,
    count: number): void {
    gl.drawArrays(mode, first, count);
}

export function draw_elements_sys(mode: number,
    count: number,
    data_type: number,
    offset: number): void {
    gl.drawElements(mode, count, data_type, offset);
}

export function uniform2ui(loc: WebGLUniformLocation,
    x: number,
    y: number): void {
    gl.uniform2ui(loc, x, y);
}

export function uniform2f(loc: WebGLUniformLocation,
    x: number,
    y: number): void {
    gl.uniform2f(loc, x, y);
}

export function uniform3f(loc: WebGLUniformLocation,
    x: number,
    y: number,
    z: number): void {
    gl.uniform3f(loc, x, y, z);
}

export function uniform_matrix3fv(loc: WebGLUniformLocation,
    data: Float32Array): void {
    gl.uniformMatrix3fv(loc, false, data);
}

export function uniform_matrix4fv(loc: WebGLUniformLocation,
    data: Float32Array): void {
    gl.uniformMatrix4fv(loc, false, data);
}

export function uniform1i(loc: WebGLUniformLocation, x: number): void {
    gl.uniform1i(loc, x);
}

export function enable_sys(cap: number): void {
    gl.enable(cap);
}

export function create_texture(): WebGLTexture {
    const ret = gl.createTexture();
    if (ret === null) {
        throw new Error("Failed to create texture");
    }

    return ret;
}

export function active_texture_sys(texture_ix: number): void {
    gl.activeTexture(texture_ix);
}

export function bind_texture_sys(target: number, texture: WebGLTexture): void {
    gl.bindTexture(target, texture);
}

export function get_seed(): Uint32Array {
    const seed = new Uint32Array(2);
    window.crypto.getRandomValues(seed);

    return seed;
}

export function now(): DOMHighResTimeStamp {
    return performance.now();
}

export function tex_image_2d_u8_sys(target: number,
    level: number,
    internal_format: number,
    width: number,
    height: number,
    format: number,
    src_data: Uint8Array): void {
    gl.texImage2D(
        target,
        level,
        internal_format,
        width,
        height,
        0,
        format,
        gl.UNSIGNED_BYTE,
        src_data,
    );
}

export function tex_image_2d_u16_sys(target: number,
    level: number,
    internal_format: number,
    width: number,
    height: number,
    format: number,
    src_data: Uint16Array): void {
    gl.texImage2D(
        target,
        level,
        internal_format,
        width,
        height,
        0,
        format,
        gl.UNSIGNED_SHORT,
        src_data,
    );
}

export function pixel_storei_sys(pname: number, param: number): void {
    gl.pixelStorei(pname, param);
}

export function tex_parameteri_sys(target: number,
    pname: number,
    param: number): void {
    //console.log("gl.texParameteri(", target, ",", pname, ",", param, ");");
    gl.texParameteri(target, pname, param);
}

export function depth_mask(flag: boolean): void {
    gl.depthMask(flag);
}

export function depth_func_sys(func: number): void {
    gl.depthFunc(func);
}

// fetch
// https://github.com/rustwasm/wasm-bindgen/tree/master/examples/fetch

/// download data using Object Url
export function download_blob(data: Uint8Array): void {
    const url = URL.createObjectURL(new Blob([data], { type: "image/png" }));
    window.open(url);
}

/// notice, TextEncoder/TextDecoder can be undefined in some platforms(Edge)
/// see polyfill:
/// https://github.com/rustwasm/wasm-bindgen/commit/717cfa303d2340bdb865cc2d51670395959b837b
export function uint8array_to_string(uint8array: Uint8Array): string {
    return new TextDecoder("utf-8").decode(uint8array);
}

export function string_to_uint8array(str: string): Uint8Array {
    return new TextEncoder().encode(str);
}

export class AccountInfo {
    id: string;
    avatar: string;
    identity_name: string;
    display_name: string;

    public constructor(id: string, avatar: string, identity_name: string, display_name: string) {
        this.id = id;
        this.avatar = avatar;
        this.identity_name = identity_name;
        this.display_name = display_name;
    }

}

export class IpInfo {
    ip: string;
    country: string;
    region: string;
    city: string;

    public constructor(ip: string, country: string, region: string, city: string) {
        this.ip = ip;
        this.country = country;
        this.region = region;
        this.city = city;
    }

}

webgl_test.then(bg => {

    //const bitmap = bg.test_mesh();
    //download_blob(bitmap);

    const prom = bg.get_account_info();
    prom.then((data: AccountInfo) => {
        console.log(data);
        console.log("account id", data.id);
    });

    //const ip = bg.get_ip_info();
    //prom.then((data: IpInfo) => {
    //    console.log(data);
    //    console.log("city id", data.city);
    //});

    bg.test_delaunator();

    bg.run_gltf();

    const result = bg.test_compress();
    console.log(uint8array_to_string(result));

    // Establish WebSocket correspondence
    const ws = new WebSocket(`ws://${location.host}/ws/`);
    ws.binaryType = "arraybuffer";
    // Request map data
    ws.addEventListener("open", () => {
        ws.send(new Uint8Array([SendOpcode.MAP_REQUEST]));
    });
    // Handle received messages
    ws.addEventListener("message", e => {
        if (!(e.data instanceof ArrayBuffer)) {
            throw new Error("Expected to receive `ArrayBuffer`");
        }

        const data = new Uint8Array(e.data);
        switch (data[0]) {
            case RecvOpcode.MAP_DATA:
                // Feed the map data into the wasm code
                if (bg.load_map_bg(new Uint8Array(data.buffer, 1)) !== 0) {
                    throw new Error("Could not load map");
                }

                // Kick off the main loop
                window.requestAnimationFrame(main_loop);
                break;
            default:
                log(`Unexpected opcode received: ${data[0]}`);
        }
    });

    // Get WebGL2 rendering context
    const canvas = document.getElementById("c");
    if (!(canvas instanceof HTMLCanvasElement)) {
        throw new Error("No HTMLCanvasElement with the ID \"c\"");
    }

    const gl_ctx = canvas.getContext("webgl2");
    if (!(gl_ctx instanceof WebGL2RenderingContext)) {
        throw new Error("No WebGL2 support detected");
    }

    gl = gl_ctx;

    // Initialize state within Rust (wasm) code
    if (bg.init_bg() !== 0) {
        throw new Error("`init_bg` failed");
    }

    // Set up DOM event handling apparatus
    const event_queue = new Event.EventQueue();

    // Pointer locking
    let pointer_locked = false;
    document.addEventListener("pointerlockchange", () => {
        pointer_locked = !pointer_locked;
    });
    document.addEventListener("pointerlockerror", () => {
        log("pointerlockerror");
    });
    canvas.addEventListener("click", e => {
        if (e.button === 0 && !pointer_locked) {
            canvas.requestPointerLock();
        }
    });

    document.addEventListener("keydown", e => {
        const key_code = Event.get_key_code(e.code);
        if (key_code !== undefined) {
            event_queue.push(
                new Event.Event(Event.KEY_DOWN, new Uint8Array([key_code]))
            );
        } else if (pointer_locked && e.code === "Escape") {
            // Always allow user to unlock pointer using ESC key
            document.exitPointerLock();
        }
    });
    document.addEventListener("keyup", e => {
        const key_code = Event.get_key_code(e.code);
        if (key_code !== undefined) {
            event_queue.push(
                new Event.Event(Event.KEY_UP, new Uint8Array([key_code]))
            );
        }
    });
    canvas.addEventListener("mousemove", e => {
        if (pointer_locked) {
            const movement_data = new Float32Array([e.movementX, e.movementY]);
            event_queue.push(new Event.Event(
                Event.MOUSE_MOVE, new Uint8Array(movement_data.buffer)
            ));
        }
    });

    // Main loop
    function main_loop(t: DOMHighResTimeStamp): void {
        window.requestAnimationFrame(main_loop);
        if (bg.main_loop_bg(t, event_queue) !== 0) {
            throw new Error("`main_loop_bg` failed");
        }
        event_queue.clear();
    }
})
    .catch(e => {
        log(`Error resolving promise \`webgl_test\`: ${e}`);
        console.log(e);
    });
