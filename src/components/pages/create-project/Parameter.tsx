import "./styles/parameter.css"

import {useEffect} from "react";
import {cacheStore} from "../../../stores/cache_store.ts";
import {createProjectStore} from "../../../stores/create_project.ts";
import Input from "../../common/Input.tsx";
import Check from "../../common/Check.tsx";
import List from "../../common/List.tsx";
import FSContext from "../../common/FSContext.tsx";

type Props = {
    param: IPackageParameter
    set: (val: string | boolean | string[]) => void,
    section: number,
    allParams: IPackageParameter[]
    is_main: boolean
}

export default function Parameter(props: Props) {
    const typ = props.param.typ.typ;

    const {param, section} = props;

    const current_template = cacheStore(state => state.currentTemplate);


    let key = "__meta__";
    if (!props.is_main) {
        key = current_template ? current_template.id : "__garbage__"
    }

    const dependencyValue =
        createProjectStore(state =>
            current_template ?
                state.results[key]?.
                    [section]?.
                    [param.while_ ?? ""]
                : undefined
    );

    const dependency = param.while_
        ? props.allParams.find(el => el.id == param.while_)
        : null;

    const is_active = (() => {
        if (!param.while_) return null;
        if (!dependency) return null;
        if (dependencyValue === undefined) {
            return false;
        }
        if (typeof dependencyValue !== typeof dependency.def) {
            return false;
        }

        return dependencyValue !== dependency.def;
    })()

    const show = typeof is_active == "boolean" ? is_active : true;


    const value = createProjectStore(
        state =>
            state.results[key]?.[section]?.[param.id]
    );
    const new_def = value !== undefined ? value : props.param.def;

    useEffect(() => {
        props.set(props.param.def as string | boolean | string[])
    }, [props.param.def]);

    return (
        <div className={"project-parameter"}>
            {(typ == "input" || typ === "area") &&
                <Input
                    show={show}
                    value={new_def?.toString() ?? ""}
                    write={props.set}
                    title={param.title}
                    typ={param.typ.typ as "input" | "area"}
                    placeholder={param.typ.placeholder ?? ""}
                    required={param.typ.required}
                />
            }
            {typ == "check" &&
                <Check
                    value={!!new_def}
                    show={show}
                    write={props.set}
                    title={param.title}
                />
            }
            {typ == "list" &&
                <List
                    value={new_def?.toString() ?? ""}
                    variants={param.typ.list_type ?? []}
                    show={show}
                    title={param.title}
                    write={props.set}
                />
            }
            {typ == "file" &&
                <FSContext
                    value={new_def?.toString() ?? ""}
                    show={show}
                    write={props.set}
                    title={param.title}
                    placeholder={param.typ.placeholder ?? ""}
                    typ={props.param.typ.fs_type as "dir" | "file"}
                />
            }
        </div>
    )
}


