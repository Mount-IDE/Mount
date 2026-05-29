import "./styles/directory.css"
import FileX from "./FileX.tsx";
import arrow from "../../assets/arrow.svg"
import React, {useRef, useState} from "react";
import {fsExtStore} from "../../stores/fs_ext_store.ts";

/**
 * @type Props
 * @param obj directory object
 * @param onContext a function that calls when OnContext event was emitted
 */
type Props = {
   obj: FsDirectory
    onContext: (e:React.MouseEvent, obj: FsDirectory|FsFile,is_file:boolean, path?:string, path_file?:string)=>void

}

/**
 * A directory component that is used in project hierarchy tree
 * @param props Props
 * @constructor
 */
export default function DirectoryX(props: Props){

    const directories = props.obj.directories
    const files = props.obj.files

    const [opened, setOpened]=useState(false);

    const ref=useRef<HTMLDivElement>(null);
    const ico = fsExtStore.getState().get_dir_by_type()

    const path = `/builtin/fs-icons/${ico[1]}`


    return (
        <div
            className={"fs-dirx"}>
            <div
                onContextMenu={
                    (e)=> {
                        e.stopPropagation()
                        props.onContext(e, props.obj, false)
                    }}
                className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}
                     style={{
                     transform: opened? "rotate(0deg)":"rotate(-90deg)"
                     }}
                     onClick={()=>setOpened(prev=>!prev)}
                >
                    <img src={arrow}/>
                </div>
                <div className={"fs-aside-icon2"}>
                    {ico[0] &&
                        <img src={path}/>
                    }
                </div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
            {opened && <div ref={ref} className={"dirx-body"}>
                {directories.map(el=>
                    <DirectoryX onContext={props.onContext} obj={el} key={`${el.path}`}/>
                )}
                {files.map(el=>
                    <FileX parent_path={props.obj.path} onContext={props.onContext} obj={el} key={`${el.path}`}/>
                )}
            </div>}
        </div>
    )
}