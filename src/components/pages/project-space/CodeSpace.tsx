import "./styles/code-space.css"
import CodeFiles from "./CodeFiles.tsx";
import Code from "./Code.tsx";



type Props={
    obj: ICodeSpace
}

export default function CodeSpace(props: Props) {
    return (
        <div id={"project-code-space"}>
            <CodeFiles files={props.obj.opened_files}/>
            <Code/>
        </div>
    )
}