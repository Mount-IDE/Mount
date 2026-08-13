import "./styles/main-project-settings.css"
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import {useEffect, useRef} from "react";


interface Option {
    label: string,
    cb: (val: string) => void
    def: string
    type: "input" | "area" | "select" | "list"
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
    const setProject = projectSettingsStore(state => state.set_project)

    const options: Option[] = [
        {
            cb: (_) => {
            },
            def: `${project?.name}`,
            label: "Project Name",
            type: "input",
            disabled: true,
            placeholder: "Project Name"
        },
        {
            cb: (_) => {
            },
            def: `${project?.path}`,
            label: "Project Path",
            type: "input",
            disabled: true,
            placeholder: "Project Path"
        },
        {
            cb: (val) => {
                if (project)
                    setProject({
                        ...project!, meta: {
                            ...project!.meta,
                            description: val
                        }
                    })
            },
            def: `${project?.meta.description}`,
            label: "Description",
            type: "area",
        },
        {
            cb: (val) => {
                if (project)
                    setProject({
                        ...project!, meta: {
                            ...project!.meta,
                            authors: val.split(":")
                        }
                    })
            },
            def: `${project?.meta.authors.join(":")}`,
            label: "Authors",
            type: "input",
            placeholder: "author1:author2"

        },
        {
            cb: (val) => {
                if (project)
                    setProject({
                        ...project!, meta: {
                            ...project!.meta,
                            license: val
                        }
                    })
            },
            def: `${project?.meta.license}`,
            label: "License",
            type: "select",
        },
        {
            cb: (val) => {
                if (project)
                    setProject({
                        ...project!, meta: {
                            ...project!.meta,
                            tags: val.split(":")
                        }
                    })
            },
            def: `${project?.meta.tags.join(":")}`,
            label: "Tags",
            type: "input",
            placeholder: "tag1:tag2"

        },


    ]

    return (
        <div id={"main-project-settings"}>
            {options.map((el, i) =>
                <ProjectOption obj={el} key={i}/>
            )}
        </div>
    )
}

type OptionProps = {
    obj: Option
}

function ProjectOption(props: OptionProps) {

    const ref = useRef<HTMLTextAreaElement>(null)

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
    return (
        <div className={"project-option"}>
            <p>{props.obj.label}</p>
            {props.obj.type == "input" &&
                <input
                    placeholder={props.obj.placeholder}
                    disabled={props.obj.disabled}
                    value={props.obj.def}
                    onInput={
                        (e) => props.obj.cb(e.currentTarget.value)}
                />
            }
            {
                props.obj.type == "area" &&
                <textarea ref={ref}
                          placeholder={props.obj.placeholder}

                          disabled={props.obj.disabled}
                          value={props.obj.def}
                          onInput={
                              (e) => props.obj.cb(e.currentTarget.value)}

                />
            }{
            props.obj.type == "select" &&
            <select value={props.obj.label} onChange={(e) => props.obj.cb(e.currentTarget.value)}>
                {licenses.map((el, key) =>
                    <option key={key} value={el}>{el}</option>
                )}
            </select>
        }
        </div>
    )
}