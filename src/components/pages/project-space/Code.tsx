import "./styles/code.css"
import {useEffect, useLayoutEffect, useRef, useState} from "react";
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
    const save = fileCacheStore(state => state.save);
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

    function _save() {
        if (file) {
            const cache = fileCacheStore.getState().get_by_id(file?.id);
            if (cache) {
                save(cache.path)
            }
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
            <CodeEditor save={_save} setText={write} text={file.content}/>
        </div>
    )
}


type CodeProps = {
    text: string
    setText: (content: string) => void
    save: () => void
}

function CodeEditor(props: CodeProps) {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    const cursor_ref = useRef<[number, number] | null>(null);

    useLayoutEffect(() => {
        if (!cursor_ref.current || !textareaRef.current) return
        const [start, end] = cursor_ref.current!;
        textareaRef.current!.setSelectionRange(start, end);
        cursor_ref.current = null
    }, [props.text]);


    function keyDown(e: React.KeyboardEvent) {
        let cur = textareaRef.current;
        console.log("down")
        if (e.ctrlKey && e.key == "d") {
            let start = cur!.selectionStart;
            let end = cur!.selectionEnd;
            let val = props.text;
            if (start != end) {
                let start_ = val.slice(0, start)
                let end_ = val.slice(end)
                let content = val.slice(start, end);
                let text = start_ + content + content + end_;
                props.setText(text);
                cursor_ref.current = [start, end]
            } else {
                let first_n = val.lastIndexOf("\n", start);
                let last_n = val.indexOf("\n", start);

                const lineStart = first_n === -1 ? 0 : first_n + 1;
                const lineEnd = last_n === -1 ? val.length : last_n;

                const column = start - lineStart;

                const line = val.slice(lineStart, lineEnd);

                const text =
                    val.slice(0, lineEnd) +
                    "\n" +
                    line +
                    val.slice(lineEnd);

                const newPos = lineEnd + 1 + column;

                cursor_ref.current = [newPos, newPos];
                props.setText(text);
            }
        }

        if (e.ctrlKey && e.key == "s") {
            props.save()
        }
    }


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
                onKeyDown={keyDown}
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