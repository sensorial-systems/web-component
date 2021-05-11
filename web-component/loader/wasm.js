import Loader from "./loader.js"
import WASMWebComponent from "../wasm.js"

export default class WASMLoader extends Loader {
    async load(module, data, template) {
        if (module.instance.default) await module.instance.default();
        for (let method in module.instance) {
            var result = method.match(/components_web_(.*)_create/);
            if (result && result.length == 2) {
                let componentName = "web-" + result[1];
                this._loadComponent(module, componentName, data, template);
            }
        }
    }

    _loadComponent(module, name, data, template) {
        class LoadedWebComponent extends WASMWebComponent {}
        let path = ("components_" + name).replace(/-|\//g,"_");
        LoadedWebComponent.prototype.module = module;
        LoadedWebComponent.prototype.path   = path;
        this._defineWebComponent(LoadedWebComponent, name, path, data, template)
    }
}