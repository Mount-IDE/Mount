import "./styles/launch-option.css"
import {launchStore} from "../../../stores/launch_store.ts";
import dir from "../../../assets/dir.svg"
import {useEffect} from "react";
import {open} from "@tauri-apps/plugin-dialog";
import {invoke} from "@tauri-apps/api/core";
import {LOG} from "../../../utils/utils.ts";

type Props = {
    obj: LaunchOption,
    section: number,
    functions: LaunchFunction[]
    cur_ref: number
    project: IProject | null

}


export default function LaunchOption(props: Props) {
    const {obj} = props;
    const find = launchStore(state => state.find_temp);
    const write = launchStore(state => state.write_temp);
    const typ = obj.typ.typ;

    const val_ = find(props.section, obj.id, props.cur_ref, props.project);
    LOG(`VAL LAUNCH ${val_} :: ${props.section} ${props.cur_ref} ${JSON.stringify(props.project?.workspace.launch_references.find(el => el.id == props.cur_ref)!.results)}`)
    LOG(`TEMPLATE ${JSON.stringify(
        props.project?.workspace.launch_templates[0].sections
    )}`)
    const val = val_ ? val_ : ""
    useEffect(() => {
        async function handler() {

            if (typeof obj.def == "number") {
                try {
                    let fn = props.functions.find(el => el.id == obj.def)
                    if (!fn) {
                        return
                    }
                    const res = await invoke<string | string[]>("call_function", {
                        func: fn!,
                        temp: launchStore.getState().current_template ?? {},
                        results: launchStore.getState().temp_results ?? {}
                    })
                    if (Array.isArray(res)) {
                        write(props.section, obj.id, res[0], props.cur_ref, props.project);
                    } else {
                        write(props.section, obj.id, res, props.cur_ref, props.project);
                    }
                } catch (e) {
                    console.error(e)
                }
            } else {
                write(props.section, obj.id, obj.def, props.cur_ref, props.project);
            }
        }

        handler().then()

    }, [obj.def]);


    function write_(val_: string) {
        write(props.section, obj.id, val_, props.cur_ref, props.project)
    }

    return (
        <div className={"launch-option"}>
            {
                typ == "check" &&
                <LaunchOptionCheck obj={obj} write={write_} val={val}/>
            }
            {
                typ == "input" &&
                <LaunchOptionInput obj={obj} write={write_} val={val}/>
            }
            {
                typ == "list" &&
                <LaunchOptionList obj={obj} write={write_} val={val}/>
            }
            {
                typ == "path" &&
                <LaunchOptionPath obj={obj} write={write_} val={val}/>
            }
        </div>
    )
}

type ChildProps = {
    write: (value: string) => void,
    val: string,
    obj: LaunchOption,
}

function LaunchOptionCheck(props: ChildProps) {
    return (
        <div className={"launch-option-child check"}>
            <input type={"checkbox"} checked={props.val == "1"}
                   onChange={(e) =>
                       props.write(e.currentTarget.checked ? "1" : "0")
                   }
            />
            <p>{props.obj.title}</p>
        </div>
    )
}

function LaunchOptionInput(props: ChildProps) {
    return (
        <div className={"launch-option-child input"}>
            <p>{props.obj.title}</p>
            <input value={props.val}
                   onInput={(e) =>
                       props.write(e.currentTarget.value)
                   }
            />
        </div>
    )
}

function LaunchOptionList(props: ChildProps) {
    return (
        <div className={"launch-option-child list"}>
            {
                props.obj.typ.list_types &&
                <select>
                    {
                        props.obj.typ.list_types!.map((el, i) =>
                            <option
                                key={i}
                                value={el}
                                selected={el == props.obj.def}>
                                {el}
                            </option>
                        )
                    }
                </select>
            }
            <p>{props.obj.title}</p>
        </div>
    )
}

function LaunchOptionPath(props: ChildProps) {

    const typ = props.obj.typ.path_type ?? "file";

    async function read_file() {
        if (typ == "file") {
            const res = await open({
                directory: false,
                title: "Choose the file"
            })
            if (res !== null) {
                props.write(res)
            }
        } else {
            const res = await open({
                directory: true,
                title: "Choose the directory"
            })
            if (res !== null) {
                props.write(res)
            }
        }
    }

    return (
        <div className={"launch-option-child path"}>
            <p>{props.obj.title}</p>
            <div>
                <input value={props.val}
                       onInput={(e) => {
                           props.write(e.currentTarget.value)
                       }
                       }
                />
                <button
                    onClick={read_file}
                ><img src={dir}/></button>
            </div>
        </div>
    )
}