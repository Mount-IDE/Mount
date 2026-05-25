import "./styles/code-files.css"
import {fsExtStore} from "../../../stores/fs_ext_store.ts";
import cross from "../../../assets/title-close.svg"
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";

type Props = {
    files: Opened[]
    id: number;
    current: [number|null, number],
    setCurrent: (id: number, id2:number)=>void
}


export default function CodeFiles(props: Props) {

    const remove_file = codeSpaceStore(state=>state.remove_file_from_code_space)
    function cb(obj_: Opened){
        remove_file(props.id, obj_);
        console.log("deleted")
    }
    function onSelect(obj_: Opened){
        props.setCurrent(obj_.id, obj_.cache_id);
    }


    return (
        <div className={"code-space-files"}>
            {props.files.map(el =>
                <CodeFile onSelect={onSelect} onRemove={cb} obj={el} selected={props.current[0]==el.id} key={el.id}/>
            )}
        </div>
    )
}


type FileProps = {
    obj: Opened
    selected: boolean
    onRemove: (obj: Opened)=>void
    onSelect: (obj: Opened)=>void
}

function CodeFile(props: FileProps) {

    const name = props.obj.name;
    const get = fsExtStore.getState().get_file_by_name(name);
    const path_to = `/builtin/fs-icons/${get[1]}`
    const from_cache = fileCacheStore(state=>state.get_by_id(props.obj.cache_id));

    return (
        <div onClick={()=>props.onSelect(props.obj)}
            style={{
                borderBottom: props.selected? "1px solid var(--border)": "1px solid transparent"
            }}
            className={"code-space-file"}>
            <div className={"code-file-img"}>
                <img src={path_to}/>
            </div>
            {from_cache!=null && from_cache.is_dirty && <p>*</p>}
            <p>{name}</p>
            <button onClick={()=>props.onRemove(props.obj)}>
                <img src={cross}/>
            </button>
        </div>
    )
}