import {create} from "zustand";

interface Type {

    show_settings: boolean;


    set_show_settings: (val: boolean) => void
}


export const settingsStore = create<Type>((set, get) => ({
    set_show_settings: (val) => set({show_settings: val}),
    show_settings: false
}))