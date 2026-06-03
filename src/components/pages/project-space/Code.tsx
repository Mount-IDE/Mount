import "./styles/code.css"
import {useEffect, useRef, useState} from "react";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

type Props = {
    current: [number | null, number]
}


export default function Code(props: Props) {

    if (props.current[0] == null) {
        return <div className={"project-code"}>Not Opened</div>
    }
    const file =
        fileCacheStore(state => state.files.find(el => el.id == props.current[1]))
    const write_file = fileCacheStore(state => state.write_file_by_id)
    if (file === undefined) {
        return (
            <div className={"project-code"}>Fallback</div>
        )
    }
    const [rows, setRows] = useState(file.content.split("\n"))

    useEffect(() => {
        setRows(file.content.split("\n"))
    }, [file.content]);

    function write(content: string) {
        if (file) {
            write_file(file.id, content);
        }
    }


    return (
        <div className={"project-code"}>
            <div className={"project-code-rows"}
                 style={{
                     paddingRight: "5px"
                 }}
            >
                {rows.map((_, i) =>
                    <p key={i} style={{
                        textAlign: "end",
                        fontSize: "14px",
                        color: "var(--border2)",
                        lineHeight: 1.5,

                    }}>{i + 1}</p>
                )}
            </div>
            <CodeEditor setText={write} text={file.content}/>
        </div>
    )
}


type CodeProps = {
    text: string
    setText: (content: string) => void
}

function CodeEditor(props: CodeProps) {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    return (
        <div
            style={{
                position: "relative",
                width: "100%",
                height: "100%",
                overflow: "hidden",
                padding: "10x"
            }}
            onClick={() => textareaRef.current!.focus()}
        >

            <textarea
                ref={textareaRef}
                spellCheck={false}
                autoCorrect="off"
                autoCapitalize="off"
                autoComplete="off"
                style={{
                    zIndex: 11,
                    position: "absolute",
                    left: 0,
                    top: 0,
                    padding: "10px",
                    minWidth: "100%",
                    height: "100%",
                    opacity: 1,
                    border: "none",
                    outline: "none",
                    resize: "none",
                    background: "transparent",
                    color: "var(--subtitle)",
                    caretColor: "var(--subtitle)",
                    lineHeight: 1.5,
                    fontSize: "14px",
                    fontStyle: "monospace",
                    overflow: "auto",
                    whiteSpace: "nowrap"
                }}
                value={props.text}
                onInput={(e) =>
                    props.setText(e.currentTarget.value)
                }
            />

        </div>
    );
}