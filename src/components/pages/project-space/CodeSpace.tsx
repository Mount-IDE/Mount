import "./styles/code-space.css"
import CodeFiles from "./CodeFiles.tsx";
import Code from "./Code.tsx";
import {useState} from "react";



type Props={
    obj: ICodeSpace
}

export default function CodeSpace(props: Props) {


    const [current,setCurrent]=useState<[number, number]>([0,0])

    return (
        <div id={"project-code-space"}>
            <CodeFiles current={current} setCurrent={setCurrent} files={props.obj.opened_files} id={props.obj.id}/>
            <Code current={current}/>
        </div>
    )
}