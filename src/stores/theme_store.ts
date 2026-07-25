import {create} from "zustand";


interface Type {
    themes: Theme[]
    current_theme: Theme | null;
    load_themes: (t: Theme[]) => void;
    set_theme: (theme: Theme) => void
}


/**
 *
 */
export const themeStore = create<Type>((set, get) => ({
    current_theme: null,
    themes: [],
    load_themes: (t: Theme[]) => set({themes: t}),
    set_theme: (theme: Theme) => set({current_theme: theme})

}))


