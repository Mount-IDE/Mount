import {createProjectStore} from "../../../stores/create_project.ts";
import Input from "../../common/Input.tsx";
import Check from "../../common/Check.tsx";
import List from "../../common/List.tsx";
import FSContext from "../../common/FSContext.tsx";
import Gen from "../../common/Gen.tsx";

type Props = {
    param: IPackageParameter
    set: (val: string | boolean | string[]) => void
    allParams: IPackageParameter[],
    pack: string
}


export default function PackageParameter(props: Props) {

    const param = props.param
    const dependencyValue =
        createProjectStore(state =>
            state.package_results[props.pack]?.
                [param.id] ?? undefined
        );

    const dependency = props.param.while_ ?
        props.allParams.find(el => el.id == props.param.id)
        : null

    const is_active = (() => {
        if (!param.while_) return null
        if (dependencyValue == undefined) return false
        if (!dependency) return null
        if (Array.isArray(dependencyValue)) {
            if (dependencyValue.length == 0)
                return false
        }
        if (typeof dependencyValue !== typeof dependency.def) {
            return false
        }
        return dependency.def !== dependencyValue
    })()

    const show = typeof is_active == "boolean" ? is_active : true;

    const value = createProjectStore(
        state =>
            state.package_results[props.pack]?.[param.id]
    );
    const new_def = value !== undefined ? value : props.param.def;

    return (
        <div className={"project-parameter"}>
            {
                (param.typ.typ == "input" || param.typ.typ == "area") &&
                <Input show={show && (param.typ.readonly ?? true)} required={param.typ.required} typ={param.typ.typ}
                       value={new_def?.toString() ?? ""}
                       title={param.title} write={props.set} placeholder={param.typ.placeholder ?? ""}/>
            }
            {
                param.typ.typ == "check" &&
                <Check show={show && (param.typ.readonly ?? true)} required={param.typ.required} value={!!new_def}
                       title={param.title}
                       write={props.set}/>
            }
            {
                param.typ.typ == "list" &&
                <List show={show && (param.typ.readonly ?? true)} required={param.typ.required}
                      variants={param.typ.list_type ?? []}
                      title={param.title} value={new_def?.toString() ?? ""} write={props.set}/>
            }
            {
                param.typ.typ == "file" &&
                <FSContext show={show && (param.typ.readonly ?? true)} required={param.typ.required}
                           typ={param.typ.fs_type == "file" ? "file" : "dir"} title={param.title}
                           value={new_def?.toString() ?? ""} write={props.set}
                           placeholder={param.typ.placeholder ?? ""}/>
            }
            {
                param.typ.typ == "gen" &&
                <Gen show={show && (param.typ.readonly ?? true)} required={param.typ.required} title={param.title}
                     value={Array.isArray(new_def) ? new_def : []} write={props.set}/>
            }
        </div>
    )
}