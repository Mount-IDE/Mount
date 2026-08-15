import "./styles/section-parameter.css"
import {settingsStore} from "../../../stores/settings_store.ts";
import {useEffect, useState} from "react";
import Input from "../../common/Input.tsx";
import Check from "../../common/Check.tsx";
import FSContext from "../../common/FSContext.tsx";
import List from "../../common/List.tsx";
import Gen from "../../common/Gen.tsx";

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
        if (val != null) {
            write_res(val, props.cat, props.i, props.obj.id)
        }
    }, [val]);

    return (
        <>
            {
                (obj.type == "input" || obj.type == "area")
                &&
                <Input
                    placeholder={""}
                    title={obj.title}
                    write={write}
                    value={val?.toString() ?? ""}
                    typ={obj.type}
                    show={obj.readonly}
                />
            }
            {
                obj.type == "check"
                &&
                <Check
                    write={write}
                    title={obj.title}
                    value={!!val}
                    show={obj.readonly}
                />
            }
            {
                ["fs", "file", "dir"].includes(obj.type)
                &&
                <FSContext
                    write={write}
                    title={obj.title}
                    show={obj.readonly}
                    value={val?.toString() ?? ""}
                    typ={obj.type == "file" ? "file" : "dir"}
                    placeholder={""}
                />
            }
            {
                obj.type == "list"
                &&
                <List
                    title={obj.title}
                    value={val?.toString() ?? ""}
                    show={obj.readonly}
                    write={write}
                    variants={obj.list ?? []}
                />
            }
            {
                obj.type == "gen"
                &&
                <Gen
                    def={obj.gen_def ?? ""}
                    value={Array.isArray(val) ? (val ?? []) : []}
                    write={write}
                    title={obj.title}
                />
            }
        </>
    )
}

