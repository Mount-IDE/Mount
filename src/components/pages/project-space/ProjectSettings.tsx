import "./styles/project-settings.css"
import {ReactElement, useEffect, useState} from "react";
import MainProjectSettings from "./MainProjectSettings.tsx";
import Button from "../../common/Button.tsx";
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import pageStore from "../../../stores/page_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import ProjectSettingsVariables from "./ProjectSettingsVariables.tsx";


interface ProjectOption {
    label: string
    component: () => ReactElement
}


export default function ProjectSettings() {


    const project = projectStore(state => state.current_project)

    useEffect(() => {
        if (project) {
            projectSettingsStore.getState().set_variables(project.vars)
        }
    }, [project])
    const [current, setCurrent] = useState(0);
    const labels: ProjectOption[] = [
        {
            label: "General",
            component: MainProjectSettings
        },
        {
            label: "Variables",
            component: ProjectSettingsVariables
        },
        {
            label: "Tasks",
            component: () => <></>
        },
        {
            label: "Launch Configurations",
            component: () => <></>
        },
        {
            label: "Packages",
            component: () => <></>
        },
    ]


    function saveSettings() {
        if (project) {
            const main_data = projectSettingsStore.getState().main_results;
            const vars = projectSettingsStore.getState().variables;
            projectStore.getState().save_project({
                ...project,
                vars: vars,
                meta: {
                    ...project.meta,
                    description: main_data[2] as string ?? project.meta.description,
                    authors: main_data[5] as string[] ?? project.meta.authors,
                    license: main_data[3] as string ?? project.meta.license,
                    tags: main_data[6] as string[] ?? project.meta.tags,
                    group: main_data[4] as string ?? project.meta.group
                }
            })
        }
    }

    const _Widget = labels[current].component
    return (
        <div id={"project-settings"}>
            <div id={"project-settings-top"}>
                <p>Project settings</p>
            </div>
            <div id={"project-settings-main"}>
                <div id={"project-settings-left"}>
                    {labels.map((el, i) =>
                        <div
                            className={"project-settings-option"}
                            key={el.label}
                            onClick={() => setCurrent(i)}
                            style={
                                i == current ? {
                                        borderBottom: "1px solid var(--title)",
                                        color: "var(--title)"
                                    } :
                                    {
                                        borderBottom: "1px solid transparent",
                                    }
                            }
                        >
                            {el.label}
                        </div>
                    )}
                </div>
                <div id={"project-settings-right"}>
                    {<_Widget/>}
                </div>
            </div>
            <div id={"project-settings-bottom"}>
                <div id={"project-settings-buttons"}>
                    <Button title={"Cancel"} cb={() => {
                        projectSettingsStore.getState().set_opened(false)
                        pageStore.getState().setFilter(false)
                    }}/>
                    <Button title={"Ok"} cb={() => {
                        saveSettings();
                        projectSettingsStore.getState().set_opened(false)
                        pageStore.getState().setFilter(false)

                    }}/><Button title={"Apply"} cb={() => {
                    saveSettings()
                    }}/>
                </div>
            </div>
        </div>
    )
}