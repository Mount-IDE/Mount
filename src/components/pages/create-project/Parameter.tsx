import "./styles/parameter.css"

import {useEffect} from "react";
import {cacheStore} from "../../../stores/cache_store.ts";
import {createProjectStore} from "../../../stores/create_project.ts";
import Input from "../../common/Input.tsx";
import Check from "../../common/Check.tsx";
import List from "../../common/List.tsx";
import FSContext from "../../common/FSContext.tsx";

type Props = {
    param: IParameter
    set: (val: string | boolean) => void,
    section: number,
    allParams: IParameter[]
    is_main: boolean
}

export default function Parameter(props: Props) {
    const typ = props.param.typ.length > 0 ? props.param.typ[0] : null;

    const {param, section} = props;

    const current_template = cacheStore(state => state.currentTemplate);


    let key = "__meta__";
    if (!props.is_main) {
        key = current_template ? current_template.id : "__garbage__"
    }

    const dependencyValue = createProjectStore(state =>
        current_template ? state.results[key]?.[section]?.[param.while_ ?? ""] : undefined
    );

    const dependency = param.while_
        ? props.allParams.find(el => el.out == param.while_)
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
            state.results[key]?.[section]?.[param.out]
    );
    const new_def = value !== undefined ? value : props.param.def;

    useEffect(() => {
        props.set(props.param.def as string | boolean)
    }, [props.param.def]);

    return (
        <div className={"project-parameter"}>
            {typ == "input" &&
                <Input
                    show={show}
                    value={new_def.toString()}
                    write={props.set}
                    title={Array.isArray(props.param.label) ?
                        props.param.label[0]
                        : props.param.label
                    }
                    typ={props.param.typ[1] == "base" ? "input" : "area"}
                    placeholder={
                        Array.isArray(props.param.label) ?
                            props.param.label[1]
                            : ""
                    }
                    required={props.param.req}
                />
            }
            {typ == "check" &&
                <Check
                    value={!!new_def}
                    show={show}
                    write={props.set}
                    title={Array.isArray(props.param.label) ?
                        props.param.label[0]
                        : props.param.label
                    }
                />
            }
            {typ == "list" &&
                <List
                    value={new_def.toString()}
                    variants={props.param.typ.slice(1)}
                    show={show}
                    title={
                        Array.isArray(props.param.label) ?
                            props.param.label[0]
                            : props.param.label
                    }
                    write={props.set}/>
            }
            {typ == "file" &&
                <FSContext
                    value={new_def.toString()}
                    show={show}
                    write={props.set}
                    title={Array.isArray(props.param.label) ?
                        props.param.label[0]
                        : props.param.label
                    }
                    placeholder={
                        Array.isArray(props.param.label) ?
                            props.param.label[1]
                            : ""
                    }

                    typ={props.param.typ[1] == "dir" ? "dir" : "file"}
                />
            }
            {
                typ == null &&
                <p>{JSON.stringify(props.param.def)}</p>
            }
        </div>
    )
}


