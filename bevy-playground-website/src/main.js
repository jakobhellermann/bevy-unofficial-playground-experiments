function loadScript(url) {
    return new Promise((resolve) => {
        let script = document.createElement("script");
        script.setAttribute("type", "application/javascript");
        script.setAttribute("src", url);
        script.setAttribute("crossorigin", "true");
        script.addEventListener("load", resolve);

        document.head.appendChild(script);
    });
}

function loadApp() {
    let projectId = "0";
    let baseUrl = "http://localhost:3000/api"
    let projectUrl = `${baseUrl}/project/${projectId}`

    let js = `${projectUrl}/playground.js`;
    let wasm = `${projectUrl}/playground.wasm`;

    loadScript(js)
        .then(() => {
            wasm_bindgen(wasm);
        });
}

window.loadApp = loadApp;
