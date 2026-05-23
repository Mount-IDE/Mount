import "./styles/code.css"
import {Dispatch, SetStateAction, useEffect, useRef, useState} from "react";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

type Props = {
    current: [number, number]
}


export default function Code(props: Props) {

    const file =
        fileCacheStore(state => state.files.find(el => el.id == props.current[1]))
    if (file === undefined) {
        return (
            <div className={"project-code"}>Fallback</div>
        )
    }
    const [text, setText]=useState(file.content);
    const rows = text.split("\n");

    return (
        <div className={"project-code"}>
            <div className={"project-code-rows"}
            style={{
                paddingRight: "5px"
            }}
            >
                {rows.map((_,i)=>
                    <p key={i} style={{
                        textAlign: "end",
                        fontSize: "14px",
                        color: "var(--border2)",
                        lineHeight: 1.5,

                    }}>{i+1}</p>
                )}
            </div>
            <CodeEditor setText={setText} text={text}/>
        </div>
    )
}



type CodeProps = {
    text: string
    setText: Dispatch<SetStateAction<string>>
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
            }}
            onClick={()=>textareaRef.current!.focus()}
        >

            <textarea
                ref={textareaRef}
                spellCheck={false}
                autoCorrect="off"
                autoCapitalize="off"
                autoComplete="off"
                style={{
                    zIndex:11,
                    position: "absolute",
                    left: 0,
                    top: 0,
                    minWidth: "100%",
                    height: "100%",
                    opacity: 1,
                    border: "none",
                    outline: "none",
                    resize: "none",
                    background: "transparent",
                    color: "var(--subtitle)",
                    caretColor:"var(--subtitle)",
                    lineHeight: 1.5,
                    fontSize: "14px",
                    fontStyle: "monospace",
                    overflow: "auto",
                    whiteSpace:"nowrap"
                }}
                defaultValue={props.text}
                onInput={(e) => props.setText((e.target as HTMLTextAreaElement).value)}
            />
            {/*<canvas*/}
            {/*    style={{*/}
            {/*        zIndex: 10,*/}
            {/*        minWidth: "100%",*/}
            {/*        height: "100%",*/}
            {/*        position:"absolute",*/}
            {/*        top: "0",*/}
            {/*        left: "0",*/}
            {/*        pointerEvents: "none",*/}
            {/*        // background: "transparent"*/}
            {/*    }}*/}
            {/*    ref={canvasRef} />*/}
        </div>
    );
}