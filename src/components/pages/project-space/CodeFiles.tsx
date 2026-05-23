import "./styles/code-files.css"
import {fsExtStore} from "../../../stores/fs_ext_store.ts";
import cross from "../../../assets/title-close.svg"
import {Dispatch, SetStateAction, useState} from "react";
import {codeSpaceStore} from "../../../stores/code_space_store.ts";

type Props = {
    files: Opened[]
    id: number;
    current: [number, number],
    setCurrent: Dispatch<SetStateAction<[number, number]>>
}


export default function CodeFiles(props: Props) {

    const remove_file = codeSpaceStore(state=>state.remove_file_from_code_space)
    function cb(obj_: Opened){
        remove_file(props.id, obj_);
        console.log("deleted")
    }
    function onSelect(obj_: Opened){
        props.setCurrent([obj_.id,obj_.cache_id]);
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
    return (
        <div onClick={()=>props.onSelect(props.obj)}
            style={{
                borderBottom: props.selected? "1px solid var(--border)": "1px solid transparent"
            }}
            className={"code-space-file"}>
            <div className={"code-file-img"}>
                <img src={path_to}/>
            </div>
            <p>{name}</p>
            <button onClick={()=>props.onRemove(props.obj)}>
                <img src={cross}/>
            </button>
        </div>
    )
}