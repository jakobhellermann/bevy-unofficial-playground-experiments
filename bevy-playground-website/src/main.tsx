import * as classes from "./main.module.css";
import "./splitpane.css";

import defaultSource from "./defaultSource";
import { compile } from "./compile";

import React, { useState, useRef, useEffect, useCallback } from "react";
import ReactDOM from "react-dom";

import SplitPane from "react-split-pane";

import Editor, { Monaco } from "@monaco-editor/react";

import type * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
type CodeEditor = monaco.editor.IStandaloneCodeEditor;

interface SourceEditorProps {
    source: string,
    setSource: (string) => void,
    recompile: (string) => void,

}
const SourceEditor = ({ source, setSource, recompile }: SourceEditorProps) => {
    const editorRef = useRef<CodeEditor>(null);

    const recompileAction = () => {
        recompile(editorRef.current.getValue());
    };

    const handleEditorMount = (editor: CodeEditor, monaco: Monaco) => {
        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.Enter, recompileAction);
        editorRef.current = editor;
    };

    return <Editor
        loading={<p style={{ color: "white" }}>Loading</p>}
        theme="vs-dark"
        language="rust"
        onMount={handleEditorMount}
        value={source}
        onChange={setSource}
        className={classes.editor}
        options={{ minimap: { enabled: false } }}
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

    const startCompilation = (source) => {
        resetLogs();
        setCompiling(true);
        compile(source, appendLog)
            .catch(e => setLogs([`failed to compile: ${e.message}`]))
            .finally(() => setCompiling(false));
    };

    return <div>
        <SplitPane split="horizontal" defaultSize={"60%"} style={{ overflowY: "scroll", overflowX: "hidden" }}>
            <SplitPane split="vertical" defaultSize={"60%"} >
                <SourceEditor source={source} setSource={setSource} recompile={startCompilation} />
                <div className={classes.messages}>
                    {logs.map((line, i) => <code key={i}>{line}</code>)}
                </div>
            </SplitPane>
            <div>
                <canvas id="bevy_canvas" width={512} height={268} style={{ backgroundColor: "#393939" }}></canvas>
            </div>
        </SplitPane>
    </div>;
};

ReactDOM.render(<App />, document.getElementById("app"));