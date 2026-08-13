import "./styles/main-project-settings.css"
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import {useEffect, useRef} from "react";
import ParameterGen from "../../common/ParameterGen.tsx";


interface Option {
    label: string,
    def: IVal
    type: "input" | "area" | "select" | "list" | "gen"
    disabled?: boolean
    placeholder?: string
}

const licenses = [
    "MIT",
    "Apache 2.0",
    "BSD",
    "LGPL-3.0",
    "GPL-3.0",
    "AGPL-3.0",
    "CC0",
    "Nonlicense",
]

export default function MainProjectSettings() {
    const project = projectSettingsStore(state => state.new_project_data)

    const options: Option[] = [
        {
            def: `${project?.name}`,
            label: "Project Name",
            type: "input",
            disabled: true,
            placeholder: "Project Name"
        },
        {

            def: `${project?.path}`,
            label: "Project Path",
            type: "input",
            disabled: true,
            placeholder: "Project Path"
        },
        {
            def: `${project?.meta.description}`,
            label: "Description",
            type: "area",
        },
        {
            def: project?.meta.authors ?? [],
            label: "Authors",
            type: "gen",
            placeholder: "author1:author2"
        },
        {
            def: `${project?.meta.license}`,
            label: "License",
            type: "select",
        },
        {
            def: project?.meta.tags ?? [],
            label: "Tags",
            type: "gen",
            placeholder: "tag1:tag2"
        },


    ]

    return (
        <div id={"main-project-settings"}>
            {options.map((el, i) =>
                <ProjectOption obj={el} opt={i} key={i}/>
            )}
        </div>
    )
}

type OptionProps = {
    obj: Option
    opt: number
}

function ProjectOption(props: OptionProps) {

    const ref = useRef<HTMLTextAreaElement>(null)

    const val = projectSettingsStore(state => state.find_main(props.opt) ?? props.obj.def)
    useEffect(() => {
        const cur = ref.current
        if (!cur) return


        function resize() {
            if (cur!.scrollHeight > cur!.clientHeight) {
                cur!.style.height = `${cur!.scrollHeight}px`
            }
        }

        cur.addEventListener("input", resize)
        return () => {
            cur.removeEventListener("input", resize)
        }
    }, []);

    function write(val_: IVal) {
        projectSettingsStore.getState().write_main(props.opt, val_);
    }


    return (
        <div className={"project-option"}>
            {props.obj.type != "gen" && <p>{props.obj.label}</p>}
            {props.obj.type == "input" &&
                <input
                    placeholder={props.obj.placeholder}
                    disabled={props.obj.disabled}
                    value={val.toString()}
                    onInput={
                        (e) => write(e.currentTarget.value)}
                />
            }
            {
                props.obj.type == "area" &&
                <textarea ref={ref}
                          placeholder={props.obj.placeholder}

                          disabled={props.obj.disabled}
                          value={val.toString()}
                          onInput={
                              (e) => write(e.currentTarget.value)}

                />
            }
            {
                props.obj.type == "select" &&
                <select value={val.toString()} onChange={(e) => write(e.currentTarget.value)}>
                    {
                        licenses.map((el, key) =>
                            <option key={key} value={el}>{el}</option>
                        )
                    }
                </select>
            }
            {
                props.obj.type == "gen" &&
                <ParameterGen title={props.obj.label} val={val as string[]} write={write}/>
            }
        </div>
    )
}