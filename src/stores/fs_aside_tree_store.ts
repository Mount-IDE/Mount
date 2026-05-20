import {create} from "zustand";

interface Type {
    tree: FsType | null;
    load_tree: (cwd: string) => void;
    change_node: (cwd: cwd_, node: FsType) => void

    delete_node: (cwd: cwd_) => void;
    add_node: (cwd: cwd_, node: FsType) => void;

    unwatch: (cwd: string) => void;
}


type cwd_node = [string, "file" | "dir"] | string
type cwd_ = cwd_node[]


export const fsAsideTreeStore =
    create<Type>((set, get) => (
        {
            tree: null,
            add_node(cwd: cwd_, node: FsType): void {
            },
            change_node(cwd: cwd_, node: FsType): void {
            },
            delete_node(cwd: cwd_): void {
            },
            load_tree(cwd: string): void {

            },
            unwatch(cwd: string): void {
            }

        }
    ))