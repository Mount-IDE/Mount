import {Language} from "web-tree-sitter"
import {create} from "zustand";
import {projectStore} from "./project_store.ts";
import {packageStore} from "./package_store.ts";
import {invoke} from "@tauri-apps/api/core";

type packageId = string
type highlightId = string

export interface LanguageInner {
    lang: Language,
    scm: string
}


interface Type {
    languages: Record<packageId, Record<highlightId, LanguageInner>>


    add_language: (pack: packageId, highiligh: highlightId) => Promise<LanguageInner | null>;

    add_languages: (pack: IPackage) => Promise<LanguageInner[] | null>
    remove_languages: (pack: packageId) => void;
    remove_language: (pack: packageId, highlight: highlightId) => void
}

export const languageStore = create<Type>((set, get) => ({
    async add_language(pack: packageId, highlight: highlightId): Promise<LanguageInner | null> {
//        await Parser.init()
        let packages = projectStore.getState().selected_packages;
        let found = packages.get(pack);
        if (!found) return null
        let needed = found.main
            .highlight
            .find(el => el.id == highlight)
        if (!needed) return null
        let path = packageStore.getState().get_wasm(pack, highlight);
        let bin = await invoke<number[]>("read_binary", {path: path})
        let arr = new Uint8Array(bin);

        let lang = await Language.load(arr)
        let langs = get().languages;
        try {

            let scm = await invoke<string>("read_scm", {
                pack, highlight
            })
            langs[pack] = {
                ...langs[pack] ?? {},
                [highlight]: {
                    lang: lang,
                    scm: scm
                }
            }
            set({
                languages: {...langs}
            })

            return {
                lang: lang,
                scm: scm
            } satisfies LanguageInner
        } catch (e) {
            console.error(e)
        }

        return null


    },
    languages: {},
    remove_language(pack: packageId, highlight: highlightId): void {
        let langs = get().languages;
        delete langs[pack]?.[highlight]
        set({
            languages: {...langs}
        })
    },
    remove_languages(pack: packageId): void {
        let langs = get().languages;
        delete langs[pack];
        set({
            languages: {...langs}
        })
    },
    async add_languages(pack: IPackage): Promise<LanguageInner[] | null> {
        let pack_id = pack.id;

        let langs = get().languages;
        let res = []
        try {
            let scms = await invoke<string>("read_scms", {pack: pack})
            console.log("scms")
            for (let i = 0; i < pack.highlight.length; i++) {
                let h = pack.highlight[i]
                let path = packageStore.getState().get_wasm(pack_id, h.id);
                console.log(path)
                let bin = await invoke<number[]>("read_binary", {path: path})
                let arr = new Uint8Array(bin);
                console.log("bin")
                let lang = await Language.load(arr)
                res.push({
                    lang, scm: scms[i] ?? ""
                })
                console.log("setting", pack_id, h.id)
                langs[pack_id] = {
                    ...langs[pack_id] ?? {},
                    [h.id]: {lang, scm: scms[i] ?? ""}
                }
            }
            set({
                languages: {...langs}
            })
            return res
        } catch (e) {
            console.error(e)
        }

        return null
    }

}))