import "./styles/code-space.css"
import CodeFiles from "./CodeFiles.tsx";
import Code from "./Code.tsx";
import {codeSpaceStore} from "../../../stores/code_space_store.ts";


type Props={
    obj: ICodeSpace
    amount: number
}

/**
 *
 * @param props
 * @constructor
 */
export default function CodeSpace(props: Props) {
    const select_current = codeSpaceStore(state => state.select_current_file)
    const current_file = props.obj.opened_files.find(el => el.id == props.obj.current_file);

    function setCurrent_(current_file: number | null, _: number) {
        select_current(props.obj.id, current_file)
    }

    return (
        <div
            style={{
                borderLeft: props.amount > 0 ? "1px solid var(--border3)" : "1px solid transparent"
            }}
            id={"project-code-space"}>
            <CodeFiles current={current_file ? [current_file.id, current_file.cache_id] : [null, 0]}
                       setCurrent={setCurrent_} files={props.obj.opened_files} id={props.obj.id}/>
            <Code current={current_file ? [current_file.id, current_file.cache_id] : [null, 0]}/>
        </div>
    )
}