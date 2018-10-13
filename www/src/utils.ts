

/// check if webassembly is supported
/// https://stackoverflow.com/questions/47879864/how-can-i-check-if-a-browser-supports-webassembly/47880734#47880734
export function is_wasm_supported(): boolean {
    if (typeof WebAssembly === "object" && typeof WebAssembly.instantiate === "function") {
        return true;
    } else {
        return false;
    }
}


