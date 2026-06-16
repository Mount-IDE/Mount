import {create} from "zustand";

interface Type {
    new_project_data: IProject | null
    set_project: (proj: IProject | null) => void
    opened: boolean
    set_opened: (val: boolean) => void
}


export const projectSettingsStore = create<Type>((set, get) => ({
    new_project_data: null,
    set_project: (proj) => set({new_project_data: proj}),
    opened: false,
    set_opened: (val: boolean) => set({opened: val})

}))