import "./styles/code.css"
import React, {useEffect, useLayoutEffect, useRef, useState} from "react";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

import {themeStore} from "../../../stores/theme_store.ts";
import {treeStore} from "../../../stores/tree_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {get_last_entity_of_path} from "../../../utils/utils.ts";
import {LanguageInner, languageStore} from "../../../stores/language_store.ts";
import {QueryCapture} from "web-tree-sitter";
import {highlightWorkerStore} from "../../../stores/highlight_worker_store.ts";

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
                        lineHeight: "21px",
                        fontFamily: "monospace",
                        letterSpacing: "normal",
                        wordSpacing: "normal",
                        // padding:"10px"
                    }}>{i + 1}</p>
                )}
            </div>
            <CodeEditor cache={file!} save={_save} setText={write} text={file.content}/>
        </div>
    )
}


type CodeProps = {
    text: string
    setText: (content: string) => void
    save: () => void
    cache: FileCache
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

async function tokenize(text: string, path: string, last: string, pack: IPackage, highlight: IPackageHighlight, prev_text: string): Promise<QueryCapture[] | null> {
    console.log(last)
    console.log(languageStore.getState().languages)
    const inner = languageStore.getState().languages[pack.id]?.[highlight.id] as LanguageInner | undefined;
    if (!inner) {
        return null
    }
    let tree = treeStore.getState().set_tree(path, pack.id, highlight.id, text);
    if (!tree) {
        console.log("not tree")
        return null
    }
    let captures = inner.query.captures(tree[0].rootNode);

    console.log("complete")
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

function CodeEditor(props: CodeProps) {
    const textareaRef = useRef<HTMLTextAreaElement>(null);

    const cursor_ref = useRef<[number, number] | null>(null);
    //const [text, setText]=useState("")
    const [tokens, setTokens] = useState<ExtToken[] | null>(null)

    const syntaxTheme = themeStore(state => state.current_theme?.elements?.common?.syntax)

    const worker = highlightWorkerStore(state => state.worker);

    useLayoutEffect(() => {
        if (!cursor_ref.current || !textareaRef.current) return
        const [start, end] = cursor_ref.current!;
        textareaRef.current!.setSelectionRange(start, end);
        cursor_ref.current = null
    }, [props.text]);

    useEffect(() => {


        async function a() {
            let file = get_last_entity_of_path(props.cache.path);
            if (!file) {
                //   console.log("\t not file")
                setTokens(null)
                return
            }
            let packs = [...projectStore.getState().selected_packages.entries()];
            // console.log(packs)
            let pack = get_needed_package(packs, file!);
            if (!pack) {
                //    console.log("\t not pack")
                setTokens(null)
                return
            }
            let highlight = get_needed_highlight(pack[1].main, file);
            if (!highlight) {
                //   console.log("\t not highlight")
                setTokens(null)
                return
            }


            let res = await tokenize(props.text, props.cache.path, file, pack[1].main, highlight, props.text);
            //console.log("______________\n||||||||\n", typeof res)

            // res.then(res=>{
            if (!res) {
                setTokens(null)
                return
            }

            let dict = highlight.nodes;
            let map = new Map<string, Token>();

            //console.log("ordering", dict)
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

            const tokens: Token[] = [...map.values()].sort((a, b) => a.start - b.start);
            const result: Token[] = [];

            let position = 0;

            for (const token of tokens) {
                // Если между предыдущим токеном и текущим есть текст
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
            //console.log("res tokens")
            // result.sort((a, b) => a.start - b.start);

            let res_tokens: ExtToken[] = [];
            let colors = highlight.syntax ?? syntaxTheme ?? {} satisfies IThemeSyntax

            //colors
            for (let i of result) {
                let typ = i.typ;
                let color = colors.tokens?.[typ as string];
                //   console.log("col", i.typ, color)
                if (!color) {
                    color = colors.base_color;
                    // console.log("\t\t sec col", color)
                    if (!color) {
                        color = syntaxTheme?.tokens?.[typ as string];
                        //  console.log("\t\t\t 3 col", color)
                        if (!color) {
                            color = syntaxTheme?.base_color
                            //  console.log("\t\t\t\t 4 col", color)
                            if (!color) {
                                color = "var(--subtitle)"
                                //    console.log("\t\t\t\t\t 5 col", color)
                            }
                        }
                    }
                }
                if (colors.colors) {
                    // console.log("\tcolors")
                    if (color in colors.colors) {
                        color = colors.colors[color]
                        // console.log("\t\t col2", color)
                    } else if (syntaxTheme?.colors) {
                        // console.log("\tcolors 2")
                        if (color in syntaxTheme.colors) {
                            color = syntaxTheme.colors[color]
                            // console.log("\t\t col2", color)
                        }
                    }
                } else if (syntaxTheme?.colors) {
                    //console.log("\tsyntax")
                    if (color in syntaxTheme.colors) {
                        color = syntaxTheme.colors[color]
                        //console.log("\t\t col3", color)

                    }
                }
                if (!color) {
                    color = "var(--subtitle)"
                    // console.log("\tinherit")
                }
                //console.log("\t\tcolor", color)
                res_tokens.push({...i, color})
            }

            // console.log("complete 22", res_tokens)

            setTokens(res_tokens)
            //setText(props.text)
            //  })


        }

        a()
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
            <div ref={h_ref} className={"code-editor-text"} style={{
                zIndex: 11,
                pointerEvents: "none",

            }}
                 onClick={() => textareaRef.current?.focus()}
            >
                {
                    tokens != null &&

                    tokens.map(el =>
                        <span key={`${el.start}:${el.end}`}
                              style={{color: el.color}}
                        >
                            {el.text}
                        </span>
                    )
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
                onScroll={(e) => e.preventDefault()}
                style={{
                    zIndex: 10,
                    opacity: "1",
                    border: "none",
                    outline: "none",
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
                zIndex: 10,
                width: "calc(100% - 40px)",
                left: "40px",
                height: "21px",
                background: "var(--border3)",
                border: "none"
            }}></hr>
        </div>
    );
}
