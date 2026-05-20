import {create} from "zustand";
import {invoke} from "@tauri-apps/api/core";

interface Type {
    tree: FsDirectory | null;
    load_tree: (cwd: string) => void;
    change_node: (cwd: cwd_, node: FsFile|FsDirectory) => void

    delete_node: (cwd: cwd_) => void;
    add_node: (cwd: cwd_, node: FsFile|FsDirectory) => void;

    unwatch: (cwd: string) => void;
}


type cwd_node = [string, "file" | "dir"] | string
type cwd_ = cwd_node[]


export const fsAsideTreeStore =
    create<Type>((set, get) => (
        {
            tree: null,
            add_node(cwd: cwd_, node: FsFile|FsDirectory): void {
            },
            change_node(cwd: cwd_, node: FsFile|FsDirectory): void {
            },
            delete_node(cwd: cwd_): void {
            },
            async load_tree(cwd: string): Promise<void> {
                try{
                    const res = await invoke<FsDirectory>("read_dir_rec", {
                        cwd: cwd
                    })
                    set({tree: res})
                }catch (e){
                    console.error(e)
                }
            },
            unwatch(cwd: string): void {
            }

        }
    ))