import {create} from "zustand";
import {invoke} from "@tauri-apps/api/core";
import {noteStore, NotificationType} from "./note_store.ts";
import {cacheStore} from "./cache_store.ts";
import {Group, mainPageStore} from "./main_page_store.ts";

interface Type {

    settings: Settings | null,
    set_settings: (s: Settings) => void


    show_settings: boolean;
    set_show_settings: (val: boolean) => void

    settings_results: SettingsResults,
    write_results: (val: string | string[] | boolean, category: number, section: number, par: number) => void
    has: (category: number, section: number, par: number) => boolean,
    get_result: (category: number, section: number, par: number) => string | string[] | boolean | null,
    clear: () => void

    save_settings: () => Promise<Settings | null>

    update_from_settings: () => void;
}

type SettingsFieldVal = boolean | string | string[]
type SettingsResults = Record<number, Record<number, Record<number, SettingsFieldVal>>>

type Handler = (s: Settings, res: SettingsResults) => void


const handlers: Handler[] = [
    (s, res) => {
        s.general.path_to_projects = res[0][0][0] as string
    },
    (s, res) => {
        s.general.project_groups = res[0][0][1] as string[]
    },
    (s, res) => {
        s.appearance.theme = res[1][0][0] as string
    }

]


export const settingsStore = create<Type>((set, get) => ({
    set_show_settings: (val) => set({show_settings: val}),
    show_settings: false,
    settings_results: {},

    settings: null,
    set_settings: (s) => set({settings: s}),
    write_results: (val, category, section, par) => {
        let res = get().settings_results
        set({
            settings_results: {
                ...res,
                [category]: {
                    ...res[category],
                    [section]: {
                        ...res[category]?.[section],
                        [par]: val
                    }
                }
            }
        })
    },
    has: (category, section, par) => {
        return !!(get().settings_results[category]?.[section]?.[par] ?? false)
    },
    get_result: (category, section, par) => {
        return get().settings_results[category]?.[section]?.[par] ?? null
    },
    clear: () => {
        set({settings_results: {}})
    },
    save_settings: async (): Promise<Settings | null> => {
        let results = get().settings_results;
        let settings = get().settings;
        if (!settings) {
            return settings;
        }
        for (let i of handlers) {
            i(settings, results)
        }
        try {
            await invoke("save_settings", {
                settings: settings
            })
        } catch (e) {
            noteStore.getState().add_note({
                text: `${e}`,
                type: NotificationType.ERR
            })
            console.error(e)
            return null;
        }
        set({settings: settings})
        return settings;

    },
    update_from_settings: () => {
        let settings = get().settings;
        if (settings) {
            cacheStore
                .getState()
                .set_projects_path(settings.general.path_to_projects)
            mainPageStore
                .getState()
                .set_groups(settings.general.project_groups.map<Group>((el, i) => ({id: i, name: el})))
        }
    }

}))