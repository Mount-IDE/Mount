import {create} from "zustand";


interface Type {
    themes: Theme[]
    current_theme: Theme | null;
    load_themes: (t: Theme[], settings: Settings) => void;
    set_theme: (theme: Theme) => void
}


/**
 *
 */
export const themeStore = create<Type>((set, get) => ({
    current_theme: null,
    themes: [],
    load_themes: (t: Theme[], settings) => set(prev => {

        let needed = t.find(el => el.id == settings.appearance.theme)
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
    set_theme: (theme: Theme) => set({current_theme: theme})

}))


