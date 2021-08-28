declare const wasm_bindgen;

// import * as monaco from 'monaco-editor/esm/vs/editor/editor.main.js';
import * as monaco from "monaco-editor";


const BASE_URL = "http://localhost:3000/api"
const DEFAULT_MAIN_RS = `use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            canvas: Some("#bevy_canvas".to_string()),
            ..Default::default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(bevy_webgl2::DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // cube
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}`;


function loadScript(url) {
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

async function loadApp(id) {
    let projectUrl = `${BASE_URL}/project/${id}`

    let js = `${projectUrl}/playground.js`;
    let wasm = `${projectUrl}/playground.wasm`;

    await loadScript(js);
    await wasm_bindgen(wasm);
}

async function compile() {
    const source = editor.getModel().getValue();
    const id = await fetch(`${BASE_URL}/compile`, { method: "POST", body: source })
        .then(throwOnNon200);

    await loadApp(id);

    console.log("Successfully reloaded app");
}

async function throwOnNon200(response) {
    if (!response.ok) {
        throw new Error(await response.text())
    }
    return await response.text();
}

const editorElement = document.getElementById('editor');
let editor = monaco.editor.create(editorElement, {
	value: DEFAULT_MAIN_RS,
    language: 'rust',
    theme: "vs-dark",
});


window.compile = compile;
