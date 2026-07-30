import {create} from "zustand";

interface Type {

    settings: Settings | null,
    set_settings: (s: Settings) => void


    show_settings: boolean;
    set_show_settings: (val: boolean) => void

    settings_results: Record<number, Record<number, Record<number, boolean | string | string[]>>>,
    write_results: (val: string | string[] | boolean, category: number, section: number, par: number) => void
    has: (category: number, section: number, par: number) => boolean,
    get_result: (category: number, section: number, par: number) => string | string[] | boolean | null,
    clear: () => void
}


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
    }


}))