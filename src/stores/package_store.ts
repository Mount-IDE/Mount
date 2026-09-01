import {create} from "zustand";
import {cacheStore} from "./cache_store.ts";


interface Type {
    packages: IPackage[] // all packages

    set_package: (pack: IPackage[]) => void
    get_wasm: (pack: string, highlight: string) => string; // return only path to
}


export const packageStore = create<Type>((set, _) => ({
    set_package(pack: IPackage[]): void {
        set({packages: pack})
    },
    packages: [],
    get_wasm(pack: string, highlight: string): string {
        let path = cacheStore.getState().data_dir;
        let constuct = cacheStore.getState().make_path(
            [path, "packages", pack, "highlights", highlight, "parser.wasm"]
        )
        return constuct
    }


}))