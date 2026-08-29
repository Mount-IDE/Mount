import {packageStore} from "../stores/package_store.ts";
import {invoke} from "@tauri-apps/api/core";
import {cacheStore} from "../stores/cache_store.ts";
import {languageStore} from "../stores/language_store.ts";

export function LOG(...messages: any[]) {
    console.log(...messages);
}

export function WARN(...messages: any[]) {
    console.warn(...messages);
}

export function ERROR(...messages: any[]) {
    console.error(...messages);
}


export async function loadPackages(pack: string[]): Promise<Map<string, PackageInner> | null> {
    try {
        let packs = packageStore.getState().packages;
        let packs2 = packs.filter(el => pack.includes(el.id))
        console.log(packs2, pack, packs)
        let meta_packs3: Record<string, PackageInner> =
            await invoke("get_meta_of_selected_packages", {packs: packs2})
        console.log(meta_packs3)

        let meta_packs = new Map(Object.entries(meta_packs3));
        for (let i of Object.entries(meta_packs3)) {
            await languageStore.getState().add_languages(i[1].main)
        }
        return meta_packs
    } catch (e) {
        console.error(e)
    }
    return null
}


export function get_last_entity_of_path(path: string): string | null {
    let os = cacheStore.getState().os;
    let splited: string[] = []
    if (os == "windows") {
        splited = path.split("\\");
    } else {
        splited = path.split("/")
    }
    if (splited.length == 0) return null
    return splited[splited.length - 1]
}

