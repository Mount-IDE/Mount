import {LanguageInner, languageStore} from "../stores/language_store.ts";
import {treeStore} from "../stores/tree_store.ts";
import {projectStore} from "../stores/project_store.ts";
import {QueryCapture} from "web-tree-sitter";
import {ExtToken, Order, Token} from "../components/pages/project-space/Code.tsx";

declare const self: DedicatedWorkerGlobalScope;

interface Props {
    langStore: typeof languageStore,
    treeStore: typeof treeStore,
    projStore: typeof projectStore,
    text: string,
    highlight: IPackageHighlight,
    pack: IPackage,
    filepath: string
    last: string
    order_types: string[]
    syntax: any
}


async function tokenize(props: Props): Promise<QueryCapture[] | null> {
    const inner = props.langStore.getState().languages[props.pack.id]?.[props.highlight.id] as LanguageInner | undefined;
    if (!inner) {
        return null
    }
    let tree = props.treeStore.getState().set_tree(props.filepath, props.pack.id, props.highlight.id, props.text);
    if (!tree) {
        return null
    }
    let captures = inner.query.captures(tree[0].rootNode);
    return captures
}

function get_type_from_arr(typ: string, arr: Record<string, string>, props: Props): Order {
    let type_ = arr[typ] as string | undefined;
    console.log("\t\t\t got", typ, type_)
    if (typeof type_ == "string" && props.order_types.includes(type_)) {
        return type_ as Order
    }
    return "unknown"
}


function get_order(typ: string, props: Props): number {
    let index = props.order_types.indexOf(typ)
    return index ?? 0
}


interface OutProps {
    typ: "complete" | "null",
    tokens?: ExtToken[]
}


self.addEventListener("message", async (e: MessageEvent<Props>) => {
    let props = e.data;

    let res = await tokenize(props);
    if (!res) {
        self.postMessage({
            typ: "null"
        } satisfies OutProps)
    }
    let dict = props.highlight.nodes;
    let map = new Map<string, Token>();

    //console.log("ordering", dict)
    for (let i of res!) {
        let start = i.node.startIndex;
        let end = i.node.endIndex
        let key = `${start}:${end}`
        let typ = get_type_from_arr(i.name, dict, props);
        let got = map.get(key);
        if (!got) {
            map.set(key, {
                end, start, text: i.node.text, typ: typ
            }satisfies Token)
            continue;
        }

        let typ2 = got.typ;
        let order1 = get_order(typ, props)
        let order2 = get_order(typ2, props)
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
        self.postMessage({typ: "null"} satisfies OutProps)
        return
    }
    let res_tokens: ExtToken[] = [];
    let colors = props.highlight.syntax ?? props.syntax ?? {} satisfies IThemeSyntax

    //colors
    for (let i of result) {
        let typ = i.typ;
        let color = colors.tokens?.[typ as string];
        //   console.log("col", i.typ, color)
        if (!color) {
            color = colors.base_color;
            // console.log("\t\t sec col", color)
            if (!color) {
                color = props.syntax?.tokens?.[typ as string];
                //  console.log("\t\t\t 3 col", color)
                if (!color) {
                    color = props.syntax?.base_color
                    //  console.log("\t\t\t\t 4 col", color)
                    if (!color) {
                        color = "inherit"
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
            } else if (props.syntax?.colors) {
                // console.log("\tcolors 2")
                if (color in props.syntax.colors) {
                    color = props.syntax.colors[color]
                    // console.log("\t\t col2", color)
                }
            }
        } else if (props.syntax?.colors) {
            //console.log("\tsyntax")
            if (color in props.syntax.colors) {
                color = props.syntax.colors[color]
                //console.log("\t\t col3", color)

            }
        }
        if (!color) {
            color = "inherit"
            // console.log("\tinherit")
        }
        //console.log("\t\tcolor", color)
        res_tokens.push({...i, color})
    }


    self.postMessage({
        tokens: res_tokens,
        typ: "complete"
    } satisfies OutProps)
    // console.log("complete 22", res_tokens)


})









