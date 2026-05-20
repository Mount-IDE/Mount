import "./styles/central.css"
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import CodeSpace from "./CodeSpace.tsx";




export default function Central() {

    const code_spaces = codeSpaceStore(state=>state.spaces)

    return (
        <div id={"project-central"}>
            {code_spaces.map(el=>
                <CodeSpace key={el.id} obj={el}/>
            )}
        </div>
    )
}