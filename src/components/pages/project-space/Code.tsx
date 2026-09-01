import "./styles/code.css"
import React, {RefObject, useEffect, useLayoutEffect, useMemo, useRef, useState} from "react";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

import {themeStore} from "../../../stores/theme_store.ts";
import {treeStore} from "../../../stores/tree_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {get_last_entity_of_path} from "../../../utils/utils.ts";
import {LanguageInner, languageStore} from "../../../stores/language_store.ts";
import {QueryCapture} from "web-tree-sitter";

type Props = {
    current: [number | null, number]
}


const order_types = [
    // ─────────────────────────────
    // Base
    // ─────────────────────────────
    "unknown",
    "expression",
    "identifier",
    "property",
    "operator",
    "punctuation",

    // ─────────────────────────────
    // Literals
    // ─────────────────────────────

    "string",
    "char",
    "number",
    "bool",
    "null",
    "regex",
    "template",

    // ─────────────────────────────
    // Declarations / types
    // ─────────────────────────────

    "type",
    "type_builtin",
    "generic",
    "param",
    "argument",

    "const",

    "fn_decl",
    "fn_call",
    "fn_b",

    "a_fn_decl",
    "a_fn_call",
    "a_fn_b",

    "spec_fn",

    "entity_stat",

    // ─────────────────────────────
    // Modifiers / keywords
    // ─────────────────────────────

    "modifier",
    "visibility",
    "storage",

    "other_kw_decl",
    "other_kw_stat",

    // ─────────────────────────────
    // Control flow
    // ─────────────────────────────

    "if_stat",

    "while_stat",
    "for_stat",
    "loop_stat",

    "match_in",
    "match_stat",

    "util_stat",
    "util_decl",

    // ─────────────────────────────
    // Exceptions
    // ─────────────────────────────

    "exception",
    "exception_decl",

    // ─────────────────────────────
    // Modules
    // ─────────────────────────────

    "scope",
    "import",
    "export",

    // ─────────────────────────────
    // Special
    // ─────────────────────────────

    "decorator",
    "annotation",
    "attribute",
    "macro",

    // ─────────────────────────────
    // Comments / documentation
    // ─────────────────────────────

    "comment",
    "doc_comment",

    // ─────────────────────────────
    // Misc
    // ─────────────────────────────

    "label",
    "escape",
    "error",
    "other",
] as const


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
            <div className={"project-code"}>File not found</div>
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

    const ref = useRef<HTMLDivElement>(null)

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
            <CodeEditor ref={ref} cache={file!} save={_save} setText={write} text={file.content}/>
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

async function tokenize(text: string, path: string, last: string, pack: IPackage, highlight: IPackageHighlight): Promise<QueryCapture[] | null> {
    // console.log(last)
    // console.log(languageStore.getState().languages)
    const inner = languageStore.getState().languages[pack.id]?.[highlight.id] as LanguageInner | undefined;
    if (!inner) {
        return null
    }
    let tree = treeStore.getState().set_tree(path, pack.id, highlight.id, text);
    if (!tree) {
        //    console.log("not tree")
        return null
    }
    let captures = inner.query.captures(tree[0].rootNode);

    // console.log("complete")
    for (let i of captures) {
        console.log("CAP", `[${i.node.startIndex}:${i.node.endIndex}]`, i.node.text, i.name)
    }

    return captures
}

function get_type_from_arr(typ: string, arr: Record<string, string>): typeof order_types[number] {
    let type_ = arr[typ] as string | undefined;
    console.log("\t\t\t got", typ, type_)
    if (typeof type_ == "string" && order_types.includes(type_ as typeof order_types[number])) {
        return type_ as typeof order_types[number]
    }
    return "unknown"
}


function get_order(typ: typeof order_types[number]): number {
    let index = order_types.indexOf(typ)
    return index ?? 0
}


export interface Token {
    typ: typeof order_types[number],
    text: string,
    start: number,
    end: number
}

export interface ExtToken extends Token {
    color: string
}

const LINE_HEIGHT = 21;
const OVERSCAN = 5;

function CodeEditor(props: CodeProps) {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    const cursor_ref = useRef<[number, number] | null>(null);
    const [tokens, setTokens] = useState<Map<number, ExtToken[]> | null>(null)

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


    function buildLineIndex(text: string, tokens: ExtToken[]) {
        const lineStarts: number[] = [0];
        for (let i = 0; i < text.length; i++) {
            if (text[i] === "\n") lineStarts.push(i + 1);
        }

        function lineOf(offset: number): number {
            let lo = 0, hi = lineStarts.length - 1;
            while (lo < hi) {
                const mid = (lo + hi + 1) >> 1;
                if (lineStarts[mid] <= offset) lo = mid; else hi = mid - 1;
            }
            return lo;
        }

        const byLine = new Map<number, ExtToken[]>();

        for (const t of tokens) {
            const startLine = lineOf(t.start);
            const endLine = lineOf(Math.max(t.start, t.end - 1));
            if (startLine === endLine) {
                byLine.set(startLine, [...(byLine.get(startLine) ?? []), t]);
                continue;
            }
            for (let l = startLine; l <= endLine; l++) {
                const lineStart = lineStarts[l];
                const lineEnd = lineStarts[l + 1] !== undefined ? lineStarts[l + 1] - 1 : text.length;
                const s = Math.max(t.start, lineStart);
                const e = Math.min(t.end, lineEnd);
                byLine.set(l, [...(byLine.get(l) ?? []), {...t, start: s, end: e, text: text.slice(s, e)}]);
            }
        }
        return {lineStarts, byLine};
    }

    /**
     * highlight logic
     */
    useEffect(() => {


        async function a() {
            let file = get_last_entity_of_path(props.cache.path);
            if (!file) {
                console.log("\t not file")
                setTokens(null)
                return
            }
            let packs = [...projectStore.getState().selected_packages.entries()];
            let pack = get_needed_package(packs, file!);
            if (!pack) {
                console.log("not pack")
                setTokens(null)
                return
            }
            let highlight = get_needed_highlight(pack[1].main, file);
            if (!highlight) {
                console.log("not highlight")
                setTokens(null)
                return
            }


            let res = await tokenize(props.text, props.cache.path, file, pack[1].main, highlight);
            if (!res) {
                console.log("not tokenize")
                setTokens(null)
                return
            }

            let dict = highlight.nodes;
            let map = new Map<string, Token>();

            for (let i of res) {
                let start = i.node.startIndex;
                let end = i.node.endIndex
                let key = `${start}:${end}`
                let typ = get_type_from_arr(i.name, dict);
                let got = map.get(key);
                if (!got) {
                    map.set(key, {
                        end, start, text: i.node.text, typ: typ
                    }satisfies Token)
                    continue;
                }

                let typ2 = got.typ;
                let order1 = get_order(typ)
                let order2 = get_order(typ2)
                if (order1 > order2) {
                    map.set(key, {
                        end, start, text: i.node.text, typ: typ
                    } satisfies Token)
                }
            }

            console.log("step 1")
            const tokens: Token[] = [...map.values()].sort((a, b) => a.start - b.start);
            const result: Token[] = [];

            let position = 0;

            for (const token of tokens) {
                if (position < token.start) {
                    result.push({
                        start: position,
                        end: token.start,
                        text: props.text.slice(position, token.start),
                        typ: "unknown"
                    });
                }

                result.push(token);

                position = Math.max(position, token.end);
            }

            if (position < props.text.length) {
                result.push({
                    start: position,
                    end: props.text.length,
                    text: props.text.slice(position),
                    typ: "unknown"
                });
            }


            if (result.length == 0) {
                setTokens(null)
                return
            }
            let res_tokens: ExtToken[] = [];
            let colors = highlight.syntax ?? syntaxTheme ?? {} satisfies IThemeSyntax

            console.log("colors")
            //colors
            for (let i of result) {
                let typ = i.typ;
                let color = colors.tokens?.[typ as string];
                if (!color) {
                    color = colors.base_color;
                    if (!color) {
                        color = syntaxTheme?.tokens?.[typ as string];
                        if (!color) {
                            color = syntaxTheme?.base_color
                            if (!color) {
                                color = "var(--subtitle)"
                            }
                        }
                    }
                }
                if (colors.colors) {
                    if (color in colors.colors) {
                        color = colors.colors[color]
                    } else if (syntaxTheme?.colors) {
                        if (color in syntaxTheme.colors) {
                            color = syntaxTheme.colors[color]
                        }
                    }
                } else if (syntaxTheme?.colors) {
                    if (color in syntaxTheme.colors) {
                        color = syntaxTheme.colors[color]

                    }
                }
                if (!color) {
                    color = "var(--subtitle)"
                }
                res_tokens.push({...i, color})
            }

            console.log("end")
            let result_ = buildLineIndex(props.text, res_tokens)
            setTokens(result_.byLine)

        }


        a().then()
    }, [props.text]);

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


    return (
        <div
            ref={container_ref}
            style={{
                position: "relative",
                width: "100%",
                height: height + "px",
                minHeight: " 100%",
                // overflow: "hidden",
                padding: "10x"
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
                    zIndex: 10,
                    opacity: "1",
                    border: "none",
                    outline: "none",
                    height: height + "px",
                    overflow: "hidden",
                    resize: "none",
                    background: "transparent",
                    color: "var(--subtitle)",
                    caretColor: "var(--subtitle)",
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
                width: "calc(100% - 40px)",
                left: "40px",
                height: "21px",
                background: "var(--border3)",
                border: "none",
                display: "block"
            }}></hr>
        </div>
    );
}
