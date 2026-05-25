import "./styles/code-space.css"
import CodeFiles from "./CodeFiles.tsx";
import Code from "./Code.tsx";
import { useEffect, useState} from "react";



type Props={
    obj: ICodeSpace
}

export default function CodeSpace(props: Props) {

    const [current, setCurrent]= useState<[number|null, number]>([props.obj.current_file,0]) // id and cache-id
    function setCurrent_(current_file: number|null, cache:number){
        setCurrent([current_file, cache])
    }

    useEffect(() => {
        setCurrent(prev=>{
            const cache = props.obj.opened_files.find(el=>el.id==props.obj.current_file);
            if (!cache){
                return prev;
            }
            return [props.obj.current_file, cache.cache_id]
        })
    }, [props.obj.current_file]);

    return (
        <div id={"project-code-space"}>
            <CodeFiles current={current} setCurrent={setCurrent_} files={props.obj.opened_files} id={props.obj.id}/>
            <Code current={current}/>
        </div>
    )
}