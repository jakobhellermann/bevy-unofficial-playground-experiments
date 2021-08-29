import * as classes from "./main.module.css";
import defaultSource from "./defaultSource";
import { compile } from "./compile";

import React, { useState, useRef } from "react";
import ReactDOM from "react-dom";

import Editor, { Monaco } from "@monaco-editor/react";

import type * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
type CodeEditor = monaco.editor.IStandaloneCodeEditor;

interface SourceEditorProps {
    source: string,
    setSource: (string) => void,

}
const SourceEditor = ({ source, setSource }: SourceEditorProps) => {
    const editorRef = useRef(null);

    const handleEditorMount = (editor: CodeEditor, monaco: Monaco) => {
        editorRef.current = editor;
    };

    return <Editor
        theme="vs-dark"
        language="rust"
        onMount={handleEditorMount}
        value={source}
        onChange={setSource}
    />;
};

const App = () => {
    const [source, setSource] = useState(defaultSource);
    const [compiling, setCompiling] = useState(false);

    const [logs, setLogs] = useState([]);
    const resetLogs = () => setLogs([]);
    const appendLog = (line: string) => {
        setLogs(logs => [...logs, line]);
    };

    const startCompilation = () => {
        resetLogs();
        setCompiling(true);
        compile(source, appendLog).finally(() => setCompiling(false));
    };

    return <div>
        <div className={classes.controls}>
            <div className={classes.editor}>
                <SourceEditor source={source} setSource={setSource} />
            </div>
            <div className={classes.messages}>
                {logs.map((line, i) => <code key={i}>{line}</code>)}
            </div>
        </div>
        <button onClick={startCompilation} disabled={compiling}>Click me</button>
        <div id="container">
            <canvas id="bevy_canvas" width="400" height="200"></canvas>
        </div>
    </div>;
};

ReactDOM.render(<App />, document.getElementById("app"));