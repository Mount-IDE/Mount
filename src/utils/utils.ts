import {packageStore} from "../stores/package_store.ts";
import {invoke} from "@tauri-apps/api/core";
import {cacheStore} from "../stores/cache_store.ts";
import {highlightClient} from "../workers/highlight_client.ts";

// noinspection JSUnusedGlobalSymbols
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
        let meta_packs3: Record<string, PackageInner> =
            await invoke("get_meta_of_selected_packages", {packs: packs2})

        let meta_packs = new Map(Object.entries(meta_packs3));

        for (let i of Object.entries(meta_packs3)) {
            let {main: pack} = i[1];
            let id = pack.id;
            try {
                let scms = await invoke<string[]>("read_scms", {pack})
                for (let i = 0; i < pack.highlight.length; i++) {
                    let h = pack.highlight[i]
                    let path = packageStore.getState().get_wasm(id, h.id)
                    let bin = await invoke<number[]>("read_binary", {path})
                    let arr = new Uint8Array(bin)
                    highlightClient.register(`${id}:${h.id}`, arr, scms[i])
                }
            } catch (e) {
                console.error(e)
            }
        }
        return meta_packs
    } catch (e) {
        console.error(e)
    }
    return null
}


export function get_last_entity_of_path(path: string): string | null {
    let os = cacheStore.getState().os;
    let splited: string[]
    if (os == "windows") {
        splited = path.split("\\");
    } else {
        splited = path.split("/")
    }
    if (splited.length == 0) return null
    return splited[splited.length - 1]
}

