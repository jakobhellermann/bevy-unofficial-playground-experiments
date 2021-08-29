declare const wasm_bindgen;

const BASE_URL = "http://localhost:3000/api"


async function throwOnNon200(response) {
    if (!response.ok) {
        throw new Error(await response.text())
    }
    return response;
}

function loadScript(url: string) {
    return new Promise((resolve) => {
        let script = document.createElement("script");
        script.setAttribute("type", "application/javascript");
        script.setAttribute("src", url);
        script.setAttribute("crossorigin", "");
        script.setAttribute("data-bevy-script", "");
        script.addEventListener("load", resolve);

        document.head.appendChild(script);
    });
}

async function loadApp(id: string) {
    let projectUrl = `${BASE_URL}/project/${id}`

    let js = `${projectUrl}/playground.js`;
    let wasm = `${projectUrl}/playground.wasm`;

    await loadScript(js);
    wasm_bindgen(wasm);
}

type CompilationResult = { status: "success", id: string; }
    | { status: "error", msg: "msg"; };

export async function compile(source: string, appendLog: (string) => void) {
    appendLog("compiling...")
    const result: CompilationResult = await fetch(`${BASE_URL}/compile`, { method: "POST", body: source })
        .then(throwOnNon200)
        .then(response => response.json())
    
    if (result.status === "error") {
        result.msg.split("\n").forEach(appendLog);
        return;
    }
    

    appendLog("loading app...")
    await loadApp(result.id);

    appendLog("success")
}