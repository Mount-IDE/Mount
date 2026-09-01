import {Edit, Point, Range, Tree} from "web-tree-sitter";
import {create} from "zustand";
import {LanguageInner, languageStore} from "./language_store.ts";
import {invoke} from "@tauri-apps/api/core";
//import {fileCacheStore} from "./file_cache_store.ts";

type filename = string // full path to file


/*interface TreeVal {
    tree: Tree,
    version: number // document version
}*/


interface TreeEntry {
    tree: Tree,
    content: string
    pack: string, // id
    highlight: string // id
}

interface Type {
    trees: Record<filename, TreeEntry>,
    set_tree: (filename: string, pack: string, highlight: string, text: string) => [Tree, Range[] | null] | null,
    load_csm: (pack: string, highlight: string) => Promise<string | null> // text of file
}


function indexToPoint(text: string, index: number): Point {
    let row = 0;
    let lastNewline = -1;
    for (let i = 0; i < index; i++) {
        if (text.charCodeAt(i) === 10 /* \n */) {
            row++;
            lastNewline = i;
        }
    }
    return {row, column: index - lastNewline - 1};
}

function computeEdit(oldText: string, newText: string) {
    const oldLen = oldText.length;
    const newLen = newText.length;
    const maxCommon = Math.min(oldLen, newLen);

    let start = 0;
    while (start < maxCommon && oldText.charCodeAt(start) === newText.charCodeAt(start)) {
        start++;
    }

    let oldEnd = oldLen;
    let newEnd = newLen;
    while (
        oldEnd > start &&
        newEnd > start &&
        oldText.charCodeAt(oldEnd - 1) === newText.charCodeAt(newEnd - 1)
        ) {
        oldEnd--;
        newEnd--;
    }

    return {
        startIndex: start,
        oldEndIndex: oldEnd,
        newEndIndex: newEnd,
        startPosition: indexToPoint(oldText, start),
        oldEndPosition: indexToPoint(oldText, oldEnd),
        newEndPosition: indexToPoint(newText, newEnd),
    };
}


export const treeStore = create<Type>((set, get) => ({
    async load_csm(pack: string, highlight: string): Promise<string | null> {
        try {
            let text = await invoke<string>("read_scm", {pack, highlight})
            return text
        } catch {
        }
        return null
    },
    set_tree(filename: string, pack: string, highlight: string, text: string): [Tree, Range[] | null] | null {
        const inner = languageStore.getState().languages[pack]?.[highlight] as LanguageInner | undefined;
        if (!inner) {
            return null
        }

        const trees = get().trees;
        const prev = trees[filename];

        const parser = inner.parser;
        let parsed: Tree | null;
        let changedRanges: Range[] | null = null
        // используем старое дерево только если это тот же файл + тот же язык/пресет подсветки
        if (prev && prev.pack === pack && prev.highlight === highlight) {
            if (prev.content === text) {
                // текст не изменился — возвращаем закешированное дерево
                return [prev.tree, null]
            }
            const edit = computeEdit(prev.content, text);
            prev.tree.edit(edit as Edit);
            parsed = parser.parse(text, prev.tree);
            changedRanges = prev.tree.getChangedRanges(parsed!)
        } else {
            parsed = parser.parse(text);
        }

        if (!parsed) {
            return null
        }

        set({
            trees: {
                ...trees,
                [filename]: {
                    tree: parsed,
                    pack,
                    highlight,
                    content: text
                }
            }
        })

        return [parsed, changedRanges]
    }, trees: {}

}))