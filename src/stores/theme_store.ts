import {create} from "zustand";

import {Window} from "./page_store"

export const enum ElementType {
    MAINPAGE,
    CREATE_PROJECT,
    PROJECT_SPACE,
    LAUNCH,
    SETTINGS,
    ENTITIES,
    INPUT,
    CHECK,
    LIST,
    GEN,
    BUTTON,
    MAIN_BUTTON,
    ICON,
    TITLEBAR
}


//type key = ITheme["elements"]

interface Type {
    themes: ITheme[]
    current_theme: ITheme | null;
    load_themes: (t: ITheme[], settings: Settings) => void;
    set_theme: (theme: ITheme) => void

    get_needed: (typ: ElementType) => any

    get_titlebar(page: Window): IThemeTitleBar | undefined;
}


/**
 *
 */
export const themeStore = create<Type>((set, get) => ({
    current_theme: null,
    themes: [],
    load_themes: (t, settings) => set(_ => {

        let needed = t.find(el => el.name == settings.appearance.theme)
        if (needed) {
            return {
                themes: t,
                current_theme: needed
            }
        } else {
            return {
                themes: t
            }
        }
    }),
    set_theme: (theme) => set({current_theme: theme}),
    get_needed: (typ) => {
        let theme = get().current_theme;
        if (!theme) {
            return null
        }
        switch (typ) {
            case ElementType.ICON:
                return theme.elements?.common?.icons;
            case ElementType.TITLEBAR:
                return theme.elements?.common?.title_bar;
            case ElementType.INPUT:
                return theme.elements?.common?.input;
            case ElementType.CHECK:
                return theme.elements?.common?.check;
            case ElementType.LIST:
                return theme.elements?.common?.list
            case ElementType.GEN:
                return theme.elements?.common?.gen
            case ElementType.BUTTON:
                return theme.elements?.common?.button
            case ElementType.MAIN_BUTTON:
                return theme.elements?.common?.main_button
            case ElementType.MAINPAGE:
                return theme.elements?.mainpage
            case ElementType.CREATE_PROJECT:
                return theme.elements?.create_project
            case ElementType.PROJECT_SPACE:
                return theme.elements?.project_space
            case ElementType.LAUNCH:
                return theme.elements?.launch
            case ElementType.SETTINGS:
                return theme.elements?.settings
            case ElementType.ENTITIES:
                return theme.elements?.create_entities


        }
    },


    get_titlebar(page: Window): IThemeTitleBar | undefined {
        let theme = get().current_theme;
        let common = theme?.elements?.common?.title_bar
        //  console.log(page)
        if (page == Window.Project) {
            let proj = theme?.elements?.project_space?.title_bar
            //   console.log(proj ? Object.keys(proj) : undefined)
            return proj ?? common
        }
        let main = theme?.elements?.mainpage?.title_bar;
        // console.log(main ? Object.keys(main) : undefined)
        return main ?? common
    }


}))


export type BP = string | {
    this?: string,
    left?: string,
    right?: string,
    top?: string,
    bottom?: string
}


export function computeBP(val: BP | undefined, kw: "padding" | "border") {
    return typeof val == "object" ? {
        [`${kw}Left`]: val?.left,
        [`${kw}Right`]: val?.right,
        [`${kw}Top`]: val?.top,
        [`${kw}Bottom`]: val?.bottom,
    } : {[kw]: val}
}

