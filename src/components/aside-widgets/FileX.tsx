import "./styles/file.css"
import {fsExtStore} from "../../stores/fs_ext_store.ts";
import {fileCacheStore} from "../../stores/file_cache_store.ts";
import {codeSpaceStore} from "../../stores/code_space_store.ts";
import {invoke} from "@tauri-apps/api/core";
import React, {useEffect} from "react";
import {asideStore} from "../../stores/aside_store.ts";
import {fsAsideTreeStore} from "../../stores/fs_aside_tree_store.ts";

type Props = {
    obj: FsFile
    onContext: (e: React.MouseEvent, obj: FsDirectory | FsFile, is_file: boolean, path?: string, path_file?: string) => void
    parent_path: string
    root: FsDirectory
}

/**
 * file component that is used in project hierarchy tree
 * @param props
 * @constructor
 */
export default function FileX(props: Props) {
    const name = props.obj.name;
    // const last_point = name.lastIndexOf(".");
    // const ext = name.slice(last_point);
    const ico = fsExtStore.getState().get_file_by_name(name);
    const path = `/builtin/fs-icons/${ico[1]}`

    const setCurrentFile = codeSpaceStore(state => state.select_current_file)


    const from_cache = fileCacheStore(state => state.get_by_path(props.obj.path));

    const add_to_cache = fileCacheStore(state => state.add_to_cache);
    const add_file_to_tab = codeSpaceStore(state => state.add_file_to_code_space)
    const current = codeSpaceStore(state => state.current)
    const check_in_cache = fileCacheStore(state => state.check)

    const results = asideStore(state => state.widgets_results["fs"]) as Record<string, IVal> | undefined;

    const meta = results?.["ap:det"]

    useEffect(() => {
        let file = {...props.obj}
        file.meta = {
            memory: file.meta?.memory ?? 0,
            readonly: file.meta?.readonly ?? false,
            modified: new Date().getTime()
        };

        fsAsideTreeStore.getState().change_node(file.path, file);
    }, [from_cache?.content]);


    /**
     * Function that added file to cache and on tab of current space
     * @async
     */
    async function click() {
        const path = props.obj.path;
        const file: OpenedFile = {
            path: path, cursor: [0, 0], name: props.obj.name
        }

        const res = check_in_cache(path)
        if (res[0]) {
            add_file_to_tab(current, res[1], file);
            return;
        }
        try {
            const content = await invoke<string>("read_file", {path: path})
            const for_cache: FileCacheLight = {
                content: content,
                path: path,
                is_dirty: false
            }
            add_to_cache(for_cache);
            const cache = fileCacheStore.getState().check(path);
            if (cache[0]) {
                const id = add_file_to_tab(current, cache[1], file);
                if (id >= 0) {
                    setCurrentFile(current, id);
                }
            }
        } catch (e) {
            console.error(e)
        }
    }


    const date = new Date(props.obj.meta?.modified ?? 0)
    return (
        <div
            onContextMenu={(e) => {
                e.stopPropagation()
                props.onContext(e, props.obj, true, props.parent_path, props.obj.path)
            }}

            className={"fs-filex"} onClick={click}>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon2"}>
                    {ico[0] &&
                        <img src={path}/>
                    }
                </div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
                {from_cache !== null && from_cache.is_dirty && <div className={"fs-aside-dirty"}>*</div>}
                {
                    meta &&
                    <div className={"fs-aside-meta"}>
                        {`${date.getFullYear()}.${date.getMonth()}.${date.getDate()}, ${props.obj.meta?.memory} B ${", " + props.obj.meta?.readonly ? "Readonly" : ""}`}

                    </div>
                }
            </div>
        </div>
    )
}