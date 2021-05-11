import Path   from "./loader/path.js"
import Loader from "./loader/multi-loader.js"

class Importer {
    constructor() {
        this.loader = new Loader();
    }

    async _importModule(manifest, data, template) {
        let modulePath = `${manifest.path.getResolvedPath()}/${manifest.module}`;
        let module = { manifest, data, template };
        try {
            module.instance = await import(modulePath);
        } catch(e) {
            console.error(`[web-component] ${e}`);
        }
        await this.loader.load(module, data, template);
    }

    async _importManifest(path) {
        let name = path.getName();
        let manifestPath = `${path}/manifest.json`;
        let manifest = {
            name,
            module: `${name}.js`,
            template: `${name}.html`,
            data: `${name}.json`
        };
        let loadedManifest = await fetch(manifestPath);

        // Overwrite the manifest's properties with the loaded ones.
        if (loadedManifest.ok) {
            loadedManifest = await loadedManifest.json();
            for (let property in loadedManifest) {
                manifest[property] = loadedManifest[property];
            }
        }

        // These properties won't be overwritten by the loadedManifest.
        manifest.path = path;
        return manifest;
    }

    async _importData(manifest) {
        let dataPath = `${manifest.path}/${manifest.data}`;
        let data = await fetch(dataPath);
        if (data.ok) {
            return await data.json();
        } else return {};
    }

    async _importTemplate(manifest) {
        let templatePath = `${manifest.path}/${manifest.template}`;
        let template = await fetch(templatePath);
        if (template.ok)
            return await template.text();
        else
            return "<template></template>";
    }

    async import(path) {
        path = new Path(path);
        let manifest = await this._importManifest(path);
        let data = await this._importData(manifest);
        let template = await this._importTemplate(manifest);
        let module = await this._importModule(manifest, data, template);
    }
}

let importer = new Importer();

export default async function ImportWebComponent(path) {
    await importer.import(path);
}

class ImporterWebComponent extends HTMLElement {
    async connectedCallback() {
        await ImportWebComponent(this.getAttribute("path"));
    }
}

customElements.define('web-component', ImporterWebComponent);