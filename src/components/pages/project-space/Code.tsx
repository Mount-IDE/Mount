import "./styles/code.css"
import React, {RefObject, useEffect, useLayoutEffect, useMemo, useRef, useState} from "react";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

import {themeStore} from "../../../stores/theme_store.ts";
//import {treeStore} from "../../../stores/tree_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {get_last_entity_of_path} from "../../../utils/utils.ts";
//import {LanguageInner, languageStore} from "../../../stores/language_store.ts";
//import {QueryCapture} from "web-tree-sitter";
import {highlightClient} from "../../../workers/highlight_client.ts";
import {ExtToken, Token} from "../../../workers/highlight_shared.ts";

type Props = {
    current: [number | null, number]
}


export default function Code(props: Props) {


    const file =
        fileCacheStore(state => state.files.find(el => el.id == props.current[1]))
    const write_file = fileCacheStore(state => state.write_file_by_id)
    const save = fileCacheStore(state => state.save);

    const [rows, setRows] = useState(file?.content.split("\n") ?? [])
    useEffect(() => {
        setRows(file?.content.split("\n") ?? [])
    }, [file?.content]);

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

    const ref = useRef<HTMLDivElement>(null)

    if (props.current[0] == null) {
        return <div className={"project-code"}>Not Opened</div>
    }

    if (file === undefined) {
        return (
            <div className={"project-code"}>File not found</div>
        )
    }


    return (
        <div className={"project-code"} ref={ref}>
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
                        lineHeight: "21px",
                        fontFamily: "monospace",
                        letterSpacing: "normal",
                        wordSpacing: "normal",
                        // padding:"10px"
                    }}>{i + 1}</p>
                )}
            </div>
            {/*<div className={"code-editor-scroll"}>*/}
            <CodeEditor ref={ref} cache={file!} save={_save} setText={write} text={file.content}/>
            {/*</div>*/}
        </div>
    )
}


type CodeProps = {
    text: string
    setText: (content: string) => void
    save: () => void
    cache: FileCache
    ref: RefObject<HTMLDivElement | null>

}

function contains(regex: string[] | undefined, text: string) {
    if (!regex) {
        return true
    }
    for (let i of regex) {
        let reg = new RegExp(i);
        if (!reg.test(text)) {
            return false
        }
    }
    return true
}


function get_needed_package(packs: [string, PackageInner][], filename: string): [string, PackageInner] | undefined {
    return packs.find(el => {

        if (contains(el[1].main.files.ignore_files, filename!) && el[1].main.files.ignore_files != null) {
            console.log("ignore")
            return false
        }
        if (!contains(el[1].main.files.files, filename!)) {
            console.log("files")
            return false
        }
        for (let i of el[1].main.files.extentions) {
            if (filename?.trim().endsWith(i.trim())) {
                console.log("ext", filename, i)
                return true
            }
        }
        return false
    })
}

function get_needed_highlight(pack: IPackage, filename: string): IPackageHighlight | undefined {
    return pack.highlight.find(el => {
        console.log(el)
        console.log(contains(el.ignore_files, filename))
        console.log(contains(el.files, filename))
        if (contains(el.ignore_files, filename) && el.ignore_files != null) {
            return false
        }
        if (!contains(el.files, filename)) {
            return false
        }
        for (let i of el.extentions) {
            if (filename?.trim().endsWith(i.trim())) {
                return true
            }
        }
        return false
    })
}


const LINE_HEIGHT = 21;
const OVERSCAN = 5;

function CodeEditor(props: CodeProps) {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    const cursor_ref = useRef<[number, number] | null>(null);

    const syntaxTheme = themeStore(state => state.current_theme?.elements?.common?.syntax)

    const totalLines = useMemo(() => {
        return props.text.split("\n").length
    }, [props.text])

    const height = 20 + totalLines * LINE_HEIGHT


    useLayoutEffect(() => {
        if (!cursor_ref.current || !textareaRef.current) return
        const [start, end] = cursor_ref.current!;
        textareaRef.current!.setSelectionRange(start, end);
        cursor_ref.current = null
    }, [props.text]);

    const reqRef = useRef(0);
    const [rawTokens, setRawTokens] = useState<Map<number, Token[]> | null>(null);
    const [hl, setHl] = useState<IPackageHighlight | null>(null);

    useEffect(() => {
        let a = highlightClient
            .subscribe(
                (fileId, reqId, lines) => {
                    if (fileId !== String(props.cache.id)) return;
                    if (reqId !== reqRef.current) return;// устаревший ответ
                    console.log("LINES", lines)
                    setRawTokens(lines);
                });

        return () => {
            a()
        }
    }, [props.cache.id]);

    useEffect(() => () => highlightClient.close(String(props.cache.id)), [props.cache.id]);

    useEffect(() => {
        const timer = setTimeout(() => {
            const file = get_last_entity_of_path(props.cache.path);
            if (!file) return setRawTokens(null);

            const packs = [...projectStore.getState().selected_packages.entries()];
            const pack = get_needed_package(packs, file);
            if (!pack) return setRawTokens(null);

            const highlight = get_needed_highlight(pack[1].main, file);
            if (!highlight) return setRawTokens(null);

            const key = `${pack[1].main.id}:${highlight.id}`;
            setHl(highlight);
            const id = ++reqRef.current;
            highlightClient.highlight(id, String(props.cache.id), key, props.text, highlight.nodes);
        }, 30);
        return () => clearTimeout(timer);
    }, [props.text, props.cache.path]);


    function resolveColor(typ: string, colors: any, theme: any): string {
        let color = colors.tokens?.[typ] ?? colors.base_color
            ?? theme?.tokens?.[typ] ?? theme?.base_color ?? "var(--subtitle)";
        if (colors.colors && color in colors.colors) color = colors.colors[color];
        else if (theme?.colors && color in theme.colors) color = theme.colors[color];
        return color || "var(--subtitle)";
    }

    const tokens = useMemo(() => {
        if (!rawTokens || !hl) return null;
        const colors = hl.syntax ?? syntaxTheme ?? {};
        const out = new Map<number, ExtToken[]>();
        for (const [line, arr] of rawTokens) {
            out.set(line, arr.map(t => ({...t, color: resolveColor(t.typ, colors, syntaxTheme)})));
        }
        return out;
    }, [rawTokens, hl, syntaxTheme]);

    function keyDown(e: React.KeyboardEvent) {
        let cur = textareaRef.current;
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
        if (e.ctrlKey && e.key === "x") {
            e.preventDefault();

            let start = cur!.selectionStart;
            let end = cur!.selectionEnd;
            let val = props.text;

            if (start != end) {
                let before = val.slice(0, start);
                let after = val.slice(end);
                let text = before + after;
                let cutted = val.slice(start, end);

                props.setText(text);
                navigator.clipboard.writeText(cutted).then();
                cursor_ref.current = [start, start];
            } else {
                let first_n = val.lastIndexOf("\n", start - 1);
                let last_n = val.indexOf("\n", end);

                const lineStart = first_n === -1 ? 0 : first_n + 1;
                const lineEndExclusive = last_n === -1 ? val.length : last_n;
                const lineEndInclusive = last_n === -1 ? val.length : last_n + 1;

                const column = start - lineStart;
                const line = val.slice(lineStart, lineEndExclusive);

                let before = val.slice(0, lineStart);
                let after = val.slice(lineEndInclusive);
                let text = before + after;

                navigator.clipboard.writeText(line + "\n").then();

                const newPos = Math.min(lineStart + column, text.length);

                props.setText(text);
                cursor_ref.current = [newPos, newPos];
            }
        }

        if (e.key === "Tab") {
            e.preventDefault()
            e.stopPropagation()
            let start = cur!.selectionStart;
            let end = cur!.selectionEnd;
            let val = props.text;
            if (start == end) {
                let before = val.slice(0, start)
                let after = val.slice(end)
                props.setText(before + "    " + after)
                cursor_ref.current = [end + 4, end + 4]
            } else {

            }
        }
    }


    const h_ref = useRef<HTMLDivElement>(null)


    const row_ref = useRef<HTMLHRElement>(null)


    /**
     * highlighted row
     */
    useEffect(() => {

        let cur = textareaRef.current;
        let cur2 = row_ref.current;
        if (!cur || !cur2) return


        function handler(e: MouseEvent) {
            let tg = (e.currentTarget as HTMLTextAreaElement)
            let pos = tg.selectionEnd;
            let text = tg.value;
            let slice = text.slice(0, pos);
            let count = slice.split("\n").length;
            let height = count * 21;
            console.log("rows", count, pos, height)
            cur2!.style.display = "block";
            cur2!.style.top = (height - 21 / 2) + "px"
        }

        function blur() {
            cur2!.style.display = "none"
        }

        cur.addEventListener("click", handler)
        cur.addEventListener("blur", blur)

        return () => {
            cur.removeEventListener("click", handler)
            cur.removeEventListener("blur", blur)
        }
    }, [])

    let raf_ref = useRef<number>(null)

    useEffect(() => {
        let cur = props.ref.current
        if (!cur) return
        let obs = new ResizeObserver(entries => {
            let entry = entries[0]
            let top = entry.target.clientHeight;
            setViewportH(top)
        })

        obs.observe(cur)

        return () => {
            obs.disconnect()
        }

    }, []);


    const [scrollTop, setScrollTop] = useState(0);
    const [viewportH, setViewportH] = useState(0);

    const firstLine = Math.max(0, Math.floor(scrollTop / LINE_HEIGHT) - OVERSCAN);
    const lastLine = Math.min(
        props.text.split("\n").length - 1,
        Math.ceil((scrollTop + viewportH) / LINE_HEIGHT) + OVERSCAN
    );

    useEffect(() => {
        let cur = props.ref.current;
        let cur2 = h_ref.current;
        if (!cur || !cur2) return


        function handleScroll(e: Event) {
            const top = (e.currentTarget as HTMLTextAreaElement).scrollTop;
            if (raf_ref.current) return;
            raf_ref.current = requestAnimationFrame(() => {
                setScrollTop(top);
                raf_ref.current = null;
            });
        }

        cur.addEventListener("scroll", handleScroll)
        return () => {
            cur.removeEventListener("scroll", handleScroll)
        }

    }, [])

    let container_ref = useRef<HTMLDivElement>(null)

    const CHAR_WIDTH = 8.43;

    const maxLen = useMemo(
        () => props.text.split("\n").reduce((m, l) => Math.max(m, l.length), 0),
        [props.text]
    );
    const contentWidth = maxLen * CHAR_WIDTH;


    return (
        <div
            ref={container_ref}
            style={{
                position: "relative",
                width: "100%",
                minWidth: `${contentWidth}px`,
                height: height + "px",
                minHeight: "100%",
                // padding: "10x",
                zIndex: 19
            }}
            onClick={() => textareaRef.current!.focus()}
        >

            <div ref={h_ref} className={"code-editor-text"} style={{
                zIndex: 11,
                pointerEvents: "none",
                overflow: "hidden",
                height: "100%"
            }}>
                {tokens != null &&
                    Array.from({length: lastLine - firstLine + 1}, (_, i) => firstLine + i)
                        .map(lineIndex => {
                            const line_tokens = tokens?.get(lineIndex) ?? []
                            return (
                                <div key={lineIndex}
                                     style={{
                                         position: "absolute",
                                         top: 10 + lineIndex * LINE_HEIGHT,
                                         height: LINE_HEIGHT + "px",
                                         lineHeight: LINE_HEIGHT + "px",
                                         whiteSpace: "pre",
                                         left: 10,
                                         right: 10,
                                         fontFamily: "monospace",
                                         fontSize: "14px",
                                         letterSpacing: "normal",
                                         wordSpacing: "normal",

                                     }}
                                >
                                    {line_tokens.map(el =>
                                        <span key={`${el.start}:${el.end}`} style={{color: el.color}}>{el.text}</span>
                                    )}
                                </div>
                            )
                        })
                }
            </div>
            <textarea
                ref={textareaRef}
                onKeyDown={keyDown}
                spellCheck={false}
                autoCorrect="off"
                autoCapitalize="off"
                autoComplete="off"
                className={"code-editor-text"}

                style={{
                    height: "100%",
                    whiteSpace: "nowrap"
                }}
                value={props.text}
                onInput={(e) =>
                    props.setText(e.currentTarget.value)
                }
            />

            <hr ref={row_ref} style={{
                position: "absolute",
                zIndex: 9,
                // left: "40px",
                width: "calc(100%)",
                height: "21px",
                background: "var(--border3)",
                border: "none",
                display: "none"
            }}></hr>
        </div>
    );
}
