import "./styles/settings-page.css"
import Button from "../../common/Button.tsx";
import {settingsStore} from "../../../stores/settings_store.ts";
import pageStore from "../../../stores/page_store.ts";
import {useMemo, useState} from "react";
import SettingsSection from "./SettingsSection.tsx";
import {noteStore, NotificationType} from "../../../stores/note_store.ts";


function useSettings(settings: Settings | null) {
    return useMemo<SettingsElement[]>(() => [
        {
            id: 0,
            title: "General",
            sections: [
                {
                    id: 0,
                    title: "System Settings",
                    parameters: [
                        {
                            id: -1,
                            title: "version",
                            type: "input",
                            def: settings?.version ?? "v",
                            readonly: true
                        },
                        {
                            id: 0,
                            title: "Path to projects",
                            type: "dir",
                            def: settings?.general.path_to_projects ?? "none",
                        },
                        {
                            id: 1,
                            title: "Project groups",
                            type: "gen",
                            def: settings?.general.project_groups ?? [],
                            required: true
                        },
                    ],
                },
            ],
        },
        {
            id: 1,
            title: "Appearance",
            sections: [
                {
                    id: 0,
                    parameters: [
                        {
                            id: 0,
                            title: "Current Theme",
                            type: "list",
                        },
                    ],
                },
            ],
        },
    ], [settings]);
}

export default function SettingsPage() {


    const close_settings = settingsStore(state => state.set_show_settings)
    const close_blur = pageStore(state => state.setFilter)

    const settings = settingsStore(state => state.settings)


    const baseSettings = useSettings(settings);


    const [currentSettings, setCurrentSettings] = useState<number>(0)

    function close() {
        close_settings(false)
        close_blur(false)
    }

    async function ok() {

        let set = await settingsStore.getState().save_settings();
        if (set) {
            settingsStore.getState().set_settings(set)
            noteStore.getState().add_note({
                text: "Settings has been saved",
                type: NotificationType.NOTE
            })
            settingsStore.getState().update_from_settings()
            return 0
        } else {
            noteStore.getState().add_note({
                text: "Cannot save settings",
                type: NotificationType.ERR
            })
            return 1
        }
    }

    return (
        <div id={"settings-page"}>
            <div id={"settings-head"}>
                <p id={"settings-label"}>Settings</p>
            </div>
            <div id={"settings-main"}>
                <div id={"settings-aside"}>
                    {
                        baseSettings.map((el, i) =>
                            <div
                                className={["settings-group", i == currentSettings ? "selected" : ""].join(" ")} key={i}
                                onClick={() => setCurrentSettings(i)}
                                style={{
                                    color: i == currentSettings ? "var(--title) !important" : "none",
                                    borderBottom: i == currentSettings ? "1px solid var(--title)" : "1px solid transparent"
                                }}
                            >
                                <p>{el.title}</p>
                            </div>
                        )
                    }
                </div>
                <div id={"settings-content"}>
                    {
                        baseSettings[currentSettings].sections?.map((el, i) =>
                            <SettingsSection obj={el} key={i} i={i} cat={currentSettings}/>
                        )
                    }
                </div>
            </div>
            <div id={"settings-footer"}>
                <div id={"settings-buttons"}>
                    <Button title={"Close"} cb={close}/>
                    <Button title={"Ok"} cb={() => {
                        ok().then()
                    }}/>
                    <Button title={"Apply"} cb={async () => {
                        let k = await ok()
                        if (k == 0) {
                            close_settings(false)
                            close_blur(false)
                        }
                    }}/>
                </div>
            </div>
        </div>
    )
}