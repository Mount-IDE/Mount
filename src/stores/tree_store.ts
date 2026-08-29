import {Parser, Tree} from "web-tree-sitter";
import {create} from "zustand";
import {LanguageInner, languageStore} from "./language_store.ts";
import {invoke} from "@tauri-apps/api/core";

type filename = string // full path to file


/*interface TreeVal {
    tree: Tree,
    version: number // document version
}*/

interface Type {
    trees: Record<filename, Tree>,
    set_tree: (filename: string, pack: string, highlight: string, text: string) => Tree | null,
    load_csm: (pack: string, highlight: string) => Promise<string | null> // text of file
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
    set_tree(filename: string, pack: string, highlight: string, text: string): Tree | null {
        const inner = languageStore.getState().languages[pack]?.[highlight] as LanguageInner | undefined;
        if (!inner) {
            return null
        }
        let parser = new Parser()
        parser.setLanguage(inner.lang)
        let parsed = parser.parse(text);
        if (!parsed) {
            return null
        }
        let trees = get().trees
        trees[filename] = parsed!;

        set({
            trees: {...trees}
        })
        return parsed
    },
    trees: {}

}))