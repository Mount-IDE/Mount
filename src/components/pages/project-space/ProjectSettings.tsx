import "./styles/project-settings.css"
import {ReactElement, useState} from "react";
import MainProjectSettings from "./MainProjectSettings.tsx";
import Button from "../../common/Button.tsx";
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import pageStore from "../../../stores/page_store.ts";
import {projectStore} from "../../../stores/project_store.ts";


interface ProjectOption {
    label: string
    component: () => ReactElement
}


export default function ProjectSettings() {


    const proj = projectSettingsStore(state => state.new_project_data)
    const [current, setCurrent] = useState(0);
    const labels: ProjectOption[] = [
        {
            label: "General",
            component: MainProjectSettings
        },
        {
            label: "Variables",
            component: () => <></>
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
                    <Button title={"Confirm"} cb={() => {
                        if (proj)
                            projectStore.getState().save_project(
                                proj
                            )
                        projectSettingsStore.getState().set_opened(false)
                        pageStore.getState().setFilter(false)

                    }}/>
                </div>
            </div>
        </div>
    )
}