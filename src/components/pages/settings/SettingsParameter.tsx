import "./styles/section-parameter.css"
import {settingsStore} from "../../../stores/settings_store.ts";
import dir from "../../../assets/dir.svg"
import * as api from "@tauri-apps/plugin-dialog"
import add from "../../../assets/plus.svg"
import close from "../../../assets/title-close.svg"
import {useEffect, useState} from "react";
import {noteStore, NotificationType} from "../../../stores/note_store.ts";

type Props = {
    obj: ISettingsParameter
    i: number,
    cat: number
}

export default function SettingsParameter(props: Props) {
    let get_res = settingsStore(state => state.get_result)
    let write_res = settingsStore(state => state.write_results)

    const [val, setVal] =
        useState<string | string[] | boolean | null>(
            () => {
                let res = get_res(props.cat, props.i, props.obj.id)
                if (res == null) {
                    return props.obj.def ?? null
                }
                return res
            }
        )

    useEffect(() => {
        if (props.obj.def != undefined) {
            write_res(props.obj.def, props.cat, props.i, props.obj.id)
            setVal(props.obj.def)
        }
    }, [props.obj.def]);
    let {obj} = props;

    function write(val: string | string[] | boolean) {
        write_res(val, props.cat, props.i, obj.id)
        setVal(val)
    }

    useEffect(() => {
        console.log(val, obj)
        if (val != null) {
            write_res(val, props.cat, props.i, props.obj.id)
        }
    }, [val]);

    return (
        <div
            style={{
                flexDirection: ["check", "list"].includes(obj.type) ? "row" : "column",
                alignItems: ["check", "list"].includes(obj.title) ? "center" : "start",
                justifyContent: ["check", "list"].includes(obj.title) ? "center" : "start",
                opacity: obj.readonly ? "0.5" : "1"
            }}
            className={"settings-parameter"}>
            {
                (obj.type == "input" || obj.type == "area")
                &&
                <ParameterInput obj={obj} write={write} val={val ?? ""}/>
            }
            {
                obj.type == "check"
                &&
                <ParameterCheck write={write} obj={obj} val={!!val}/>
            }
            {
                ["fs", "file", "dir"].includes(obj.type)
                &&
                <ParameterFs write={write} obj={obj} val={val ?? ""}/>
            }
            {
                obj.type == "list"
                &&
                <ParameterList val={val ?? ""} obj={obj} write={write}/>
            }
            {
                obj.type == "gen"
                &&
                <ParameterGen val={val ?? ""} obj={obj} write={write}/>
            }
        </div>
    )
}

type InnerProps = {
    obj: ISettingsParameter
    write: (val: string | string[] | boolean) => void
    val: string | string[] | boolean
}

function ParameterInput(props: InnerProps) {

    return (
        <>
            <p>{props.obj.title} {props.obj.readonly}</p>
            {
                props.obj.type == "input"
                && <input
                    readOnly={!!props.obj.readonly}
                    value={props.val.toString()}
                    onInput={(e) =>
                        props.write((e.target as HTMLInputElement).value)}
                />
            }
            {
                props.obj.type == "area"
                &&
                <textarea
                    readOnly={props.obj.readonly}
                    value={props.val.toString()}
                    onInput={(e) => props.write((e.target as HTMLTextAreaElement).value)}
                />
            }
        </>
    )
}


function ParameterCheck(props: InnerProps) {

    return (
        <>
            <p>{props.obj.title}</p>
            <input
                readOnly={!!props.obj.readonly}
                type={"checkbox"}
                checked={!!props.val}
                onChange={(e) => props.write((e.target as HTMLInputElement).checked)}
            />
        </>
    )
}


function ParameterFs(props: InnerProps) {

    async function dialog_() {
        const res = await api.open({
            directory: ["fs", "directory"].includes(props.obj.type)
        })
        if (res) {
            props.write(res!)
        }
    }

    return (
        <>
            <p>{props.obj.title}</p>
            <div className={"settings-parameter-fs"}>
                <input
                    readOnly={props.obj.readonly}
                    value={props.val.toString()}
                    onInput={(e) => props.write((e.target as HTMLInputElement).value)}
                />
                <button onClick={dialog_}>
                    <img src={dir}/>
                </button>
            </div>
        </>
    )
}


function ParameterList(props: InnerProps) {
    return (
        <>
            <select
                onChange={(e) =>
                    props.write((e.target as HTMLSelectElement).value)}>
                {
                    props.obj.list?.map((el, i) =>
                        <option value={el} key={i} selected={props.val == el}>
                            {el}
                        </option>
                    )
                }
            </select>
            <p>{props.obj.title}</p>
        </>
    )
}

function ParameterGen(props: InnerProps) {

    const [list, setList] = useState<string[]>([])
    useEffect(() => {
        setList(Array.isArray(props.val) ? props.val : [])
    }, [props.val])

    function change(i: number, val: string) {

        let res = [...list];
        res[i] = val;
        props.write(res)

        //props.write(list)
    }

    function add_() {
        let res = [...list]
        res.push("group")
        props.write(res)

    }

    function remove(i: number) {
        if (props.obj.required && list.length <= 1) {
            noteStore.getState().add_note({
                text: "Cannot delete last element",
                type: NotificationType.WARN
            }, 2_000)
            return;
        }
        let res = [...list]
        res.splice(i, 1)
        props.write(res);
    }

    return (
        <>
            <p>{props.obj.title}</p>
            <div className={"parameter-gen"}>
                <button className={"parameter-gen-bt"} onClick={add_}>
                    <img src={add}/>
                </button>
                <hr/>
                {
                    list.map((el, i) =>
                        <div key={i} className={"parameter-gen-el"}>
                            <input value={el}
                                   onInput={
                                       (e) =>
                                           change(
                                               i,
                                               (e.target as HTMLInputElement).value
                                           )
                                   }
                            />
                            <button className={""}
                                    onClick={() => remove(i)}
                            >
                                <img src={close}/>
                            </button>
                        </div>
                    )
                }
            </div>

        </>
    )
}