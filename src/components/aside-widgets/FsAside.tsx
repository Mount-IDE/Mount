import "./styles/fs-aside.css"
import {fsAsideTreeStore} from "../../stores/fs_aside_tree_store.ts";
import React, {useEffect, useRef, useState} from "react";
import {projectStore} from "../../stores/project_store.ts";
import DirectoryX from "./DirectoryX.tsx";
import ContextMenu, {IContextMenuButton} from "../common/ContextMenu.tsx";
import {menuStore} from "../../stores/menu_store.ts";
import {fileCacheStore} from "../../stores/file_cache_store.ts";
import {contextStore} from "../../stores/context_store.ts";


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

    const open_window = menuStore(state=>state.open_file_create_menu)

    const buttons: IContextMenuButton[] = [
        {
            cb: () => open_window(),
            hotkeys: "",
            icon: "copy.svg",
            title: "New"
        }, {
            cb: () => {
            },
            hotkeys: "Ctrl+X",
            icon: "copy.svg",
            title: "Cut"
        }, {
            cb: () => {
            },
            hotkeys: "Ctrl+C",
            icon: "copy.svg",
            title: "Copy"
        },
    ]
    const set_path = contextStore(state=>state.set_path)
    function open_context_menu(e: React.MouseEvent, obj: FsFile | FsDirectory, path?:string) {
        e.preventDefault()
        const x = e.clientX;
        const y = e.clientY;
        setCursor([x, y]);
        setShowContext(true)
        setContextTree(obj);
        set_path(path?? obj.path)

    }

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
        return ()=>{
            window.removeEventListener("click", handler)
        }
    }, [cursor]);

    const ref =
        useRef<HTMLDivElement>(null)

    return (
        <>
            <ContextMenu ref={ref} show={showContext} buttons={buttons} x={cursor[0]} y={cursor[1]}/>
            <div className={"aside-in"}
                onContextMenu={(e)=>open_context_menu(e, tree!)}
            >
                {tree != null && <DirectoryX
                    onContext={open_context_menu} obj={tree}/>}
            </div>
        </>
    )
}



