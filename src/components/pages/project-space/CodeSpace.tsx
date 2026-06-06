import "./styles/code-space.css"
import CodeFiles from "./CodeFiles.tsx";
import Code from "./Code.tsx";
import {codeSpaceStore} from "../../../stores/code_space_store.ts";


type Props={
    obj: ICodeSpace
    amount: number
}

export default function CodeSpace(props: Props) {

    // const [current, setCurrent]=
    //     useState<[number|null, number]>([props.obj.current_file,0]) // id and cache-id

    const select_current = codeSpaceStore(state => state.select_current_file)

    const current_file = props.obj.opened_files.find(el => el.id == props.obj.current_file);


    function setCurrent_(current_file: number|null, cache:number){
        // setCurrent([current_file, cache])
        select_current(props.obj.id, current_file)
    }

    // useEffect(() => {
    //     setCurrent(prev=>{
    //         const cache =
    //             props.obj.opened_files.find(el=>el.id==props.obj.current_file);
    //         if (!cache){
    //             return [null, 0];
    //         }
    //         return [props.obj.current_file, cache.cache_id]
    //     })
    // }, [props.obj.current_file, props.obj.opened_files]);

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