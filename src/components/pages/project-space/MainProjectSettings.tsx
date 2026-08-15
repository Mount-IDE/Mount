import "./styles/main-project-settings.css"
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import {useEffect, useRef} from "react";
import Gen from "../../common/Gen.tsx";
import Input from "../../common/Input.tsx";
import List from "../../common/List.tsx";
import {settingsStore} from "../../../stores/settings_store.ts";


interface Option {
    label: string,
    def: IVal
    type: "input" | "area" | "select" | "list" | "gen"
    variants?: string[]
    disabled?: boolean
    placeholder?: string
    def_?: string
}


export default function MainProjectSettings() {
    const project = projectSettingsStore(state => state.new_project_data)
    const settings = settingsStore(state => state.settings);
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
            def: `${project?.meta.license}`,
            label: "License",
            variants: [
                "MIT",
                "Apache 2.0",
                "BSD",
                "LGPL-3.0",
                "GPL-3.0",
                "AGPL-3.0",
                "CC0",
                "Nonlicense",
            ],
            type: "select",
        },
        {
            type: "select",
            def: project?.meta.group ?? "",
            label: "Group",
            variants: settings?.general.project_groups ?? ["general"]
        },
        {
            def: project?.meta.authors ?? [],
            label: "Authors",
            type: "gen",
            placeholder: "author1:author2",
            def_: "author"
        },
        {
            def: project?.meta.tags ?? [],
            label: "Tags",
            type: "gen",
            placeholder: "tag1:tag2",
            def_: "tag"
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
        <>
            {
                (props.obj.type == "input" || props.obj.type == "area") &&
                <Input
                    placeholder={props.obj.placeholder ?? ""}
                    show={!props.obj.disabled}
                    value={val.toString()}
                    write={write}
                    typ={props.obj.type}
                    title={props.obj.label}

                />
            }
            {
                props.obj.type == "select" &&
                <List
                    value={val.toString()}
                    write={write}
                    title={props.obj.label}
                    variants={props.obj.variants ?? []}
                />
            }
            {
                props.obj.type == "gen" &&
                <Gen
                    def={props.obj.def_ ?? ""}
                    title={props.obj.label}
                    value={val as string[]}
                    write={write}
                />
            }
        </>
    )
}