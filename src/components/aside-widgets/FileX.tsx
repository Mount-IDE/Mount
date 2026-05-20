import "./styles/file.css"
import {fsExtStore} from "../../stores/fs_ext_store.tsx";
import {fileCacheStore} from "../../stores/file_cache_store.ts";
import {codeSpaceStore} from "../../stores/code_space_store.ts";
import {invoke} from "@tauri-apps/api/core";

type Props={
    obj: FsFile
}


export default function FileX(props: Props){
    const name = props.obj.name;
    const last_point = name.lastIndexOf(".");
    const ext = name.slice(last_point);
    const ico = fsExtStore.getState().get_file_by_ext(ext);
    const path = `/builtin/fs-icons/${ico[1]}`

    const add_to_cache = fileCacheStore(state=>state.add_to_cache);
    const add_file = codeSpaceStore(state=>state.add_file_to_code_space)
    const current = codeSpaceStore(state=>state.current)
    const check = fileCacheStore(state=>state.check)
    async function click() {
        const path = props.obj.path;
        const file : OpenedFile= {
            path: path, cursor: [0,0], name: props.obj.name
        }
        if (check(path)){
            add_file(current, file);
            return;
        }
        try{
            const content = await invoke<string>("read_file", {path: path})
            const for_cache: FileCache= {
                content: content,
                path: path
            }
            add_to_cache(for_cache);
        }catch(e){
            console.error(e)
        }
    }

    return (
        <div className={"fs-filex"} onClick={click}>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}>
                    {ico[0] &&
                        <img src={path}/>
                    }
                </div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
        </div>
    )
}