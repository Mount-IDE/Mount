import "./styles/fs-aside.css"
import {fsAsideTreeStore} from "../../stores/fs_aside_tree_store.ts";
import React, {useEffect, useRef, useState} from "react";
import {projectStore} from "../../stores/project_store.ts";
import DirectoryX from "./DirectoryX.tsx";
import ContextMenu, {IContextMenuButton} from "../common/ContextMenu.tsx";
import {menuStore} from "../../stores/menu_store.ts";
import {fileCacheStore} from "../../stores/file_cache_store.ts";
import {contextStore} from "../../stores/context_store.ts";
import {ModalButton} from "../common/Modal.tsx";
import {invoke} from "@tauri-apps/api/core"
import {cacheStore} from "../../stores/cache_store.ts";

export default function FsAside() {

    const tree = fsAsideTreeStore(state => state.tree);
    const load_tree = fsAsideTreeStore(state => state.load_tree);
    const watch = fsAsideTreeStore(state => state.watch);
    const unwatch = fsAsideTreeStore(state => state.unwatch);
    const cwd = projectStore(state => state.path_to_current_project);

    useEffect(() => {
        if (cwd.length === 0) {
            return;
        }

        load_tree(cwd).then();
        watch(cwd).then();

        return () => {
            void unwatch();
        }
    }, [cwd, load_tree, watch, unwatch]);

    const [cursor, setCursor] = useState([0, 0])
    const [showContext, setShowContext] = useState(false)
    const [contextTree, setContextTree] = useState<FsFile | FsDirectory | null>(null)

    const open_window = menuStore(state => state.open_file_create_menu)

    const [buttons, setButtons] = useState<IContextMenuButton[]>([])

    const set_path = contextStore(state => state.set_path)


    useEffect(() => {
        function handler(e: MouseEvent) {
            const tg = ref.current;
            if (!tg) return;
            const elem = e.target as Element
            if (!(tg.contains(elem) || tg == elem)) {
                setShowContext(false)

            }

        }

        window.addEventListener("click", handler)
        return () => {
            window.removeEventListener("click", handler)
        }
    }, [cursor]);

    const ref =
        useRef<HTMLDivElement>(null)

    const save_file = fileCacheStore(state => state.save)


    const open_modal = menuStore(state => state.open_modal)

    const close_modal = menuStore(state => state.close_modal)
    // const remove_from_cache = fileCacheStore(state => state.remove);

    function open_context_menu(e: React.MouseEvent, obj: FsFile | FsDirectory, is_file: boolean, path?: string, path_file?: string) {
        e.preventDefault()
        const x = e.clientX;
        const y = e.clientY;
        setCursor([x, y]);
        setShowContext(true)
        setContextTree(obj);
        set_path(path ?? obj.path)

        if (is_file) {
            setButtons(
                [
                    {
                        cb: () => open_window(),
                        hotkeys: "",
                        icon: "copy.svg",
                        title: "New"
                    },
                    {
                        cb: () => {
                        },
                        hotkeys: "Ctrl+X",
                        icon: "cut.svg",
                        title: "Cut"
                    },
                    {
                        cb: () => {
                        },
                        hotkeys: "Ctrl+C",
                        icon: "copy.svg",
                        title: "Copy"
                    }, {
                    cb: () => {
                    },
                    hotkeys: "Ctrl+C",
                    icon: "copy.svg",
                    title: "Copy Path"
                }, {
                    cb: () => {
                    },
                    hotkeys: "Ctrl+C",
                    icon: "copy.svg",
                    title: "Paste"
                }, {
                    cb(): void {
                        console.log("saved", path_file)
                        if (path_file) {
                            save_file(path_file!)
                        }
                    },
                    hotkeys: "",
                    icon: "save.svg",
                    title: "Save"

                }, {
                    cb(): void {
                        const buttons: ModalButton[] = [
                            {
                                cb: (_) => {
                                    try {
                                        invoke("remove_file", {path: obj.path}).then();
                                        // remove_from_cache(obj.path);
                                        close_modal()
                                    } catch (e) {
                                        console.error(e)
                                    }
                                },
                                title: "Confirm",
                                typ: "cancel"
                            }
                        ]
                        open_modal({
                            buttons: buttons,
                            title: `Delete ${obj.name}?`,
                            typ: "confirm",
                        })

                    },
                    hotkeys: "",
                    icon: "",
                    title: "Delete"

                }, {
                    cb: () => {
                        const buttons: ModalButton[] = [
                            {
                                cb: (val) => {
                                    console.log("rename ready", val)
                                    if (val !== undefined) {
                                        const os = cacheStore.getState().os;
                                        const sep = os!=="windows"?"/":"\\"
                                        const path_ = `${path}${sep}${val}`
                                        console.log("rename: ", obj.path, path_);
                                        try {
                                            invoke("rename_file", {from: obj.path, to: path_}).then();
                                            close_modal();
                                        } catch (e) {
                                            console.error(e)
                                        }
                                    }
                                },
                                title: "Confirm",
                                typ: "input"
                            }
                        ]
                        open_modal({
                            buttons: buttons,
                            title: `Rename ${obj.name}?`,
                            typ: "prompt",
                            val: obj.name
                        })
                    },
                    hotkeys: "",
                    icon: "",
                    title: "Rename"

                }
                ]
            )
        }

    }

    return (
        <>
            <ContextMenu ref={ref} show={showContext} buttons={buttons} x={cursor[0]} y={cursor[1]}/>
            <div className={"aside-in"}
                 onContextMenu={(e) =>
                     open_context_menu(e, tree!, false, undefined)
                 }
            >
                {tree != null && <DirectoryX
                    onContext={(e,
                                obj,
                                is_file,
                                path, path2) =>
                        open_context_menu(e, obj, is_file, path, path2)

                    } obj={tree}/>}
            </div>
        </>
    )
}





