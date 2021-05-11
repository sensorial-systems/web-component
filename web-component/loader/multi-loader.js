import JavaScriptLoader from "./javascript.js"
import WASMLoader       from "./wasm.js"
import Loader           from "./loader.js"

export default class MultiLoader {
    constructor() {
        this.javascript = new JavaScriptLoader();
        this.wasm       = new WASMLoader();
        this.default    = new Loader();
    }

    static _getType(module) {
        try {
            if (module.instance["web_component_target_wasm"] || module.instance.default.name == "init")
                return "WASM";
            else
                return "JAVASCRIPT";
        } catch(e) {
            return "DEFAULT";
        }
    }

    async load(module, data, template) {
        let type = MultiLoader._getType(module);
             if (type == "WASM")       this.wasm.load(module, data, template);
        else if (type == "JAVASCRIPT") this.javascript.load(module, data, template);
        else                           this.default.load(module, data, template);
    }
}
