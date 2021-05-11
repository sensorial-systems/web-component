import WebComponent from "./web-component.js"

export default class WASMWebComponent extends WebComponent {
    constructor() {
        super();
        this.ffi_create       = this._getMethod("create");
        this.ffi_template     = this._getMethod("template");
        this.ffi_get_data     = this._getMethod("get_data");
        this.ffi_update_field = this._getMethod("update_field");
        this.ffi_on_loaded    = this._getMethod("on_loaded");
    }

    _getMethod(name) {
        let module = this.module.instance;
        let method = `${this.path}_${name}`;
            method = module[method];
        return method;
    }

    async getTemplate() {
        return this.ffi_template();
    }

    createObject() {
        return this.ffi_create(this.shadowRoot.host.attributes);
    }

    getData() {
        if (this.ffi_get_data) {
            this.data = JSON.parse(this.ffi_get_data(this.object));
        }
        return this.data
    }

    onload() {
        return this.ffi_on_loaded(this.object, this.shadowRoot);
    }

    updateField(name, value) {
        this.ffi_update_field(this.object, name, value);
    }

    async connectedCallback() {
        this.object = this.createObject(this.shadowRoot.host.attributes);

//        for (var name in data) {
//            watch[name] = (function(name) {
//                return function(new_val,_) {
//                    let data = JSON.stringify(new_val);
//                    this.web_component.updateField(name, data);
//                }
//            })(name)
//        }

        super.connectedCallback();
    }
}