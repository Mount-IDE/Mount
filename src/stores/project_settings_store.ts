import {create} from "zustand";
import {launchStore} from "./launch_store.ts";

interface Type {
    new_project_data: IProject | null
    set_project: (proj: IProject | null) => void
    opened: boolean
    set_opened: (val: boolean) => void

}


export const projectSettingsStore = create<Type>((set, get) => ({
    new_project_data: null,
    set_project: (proj) => {
        set({new_project_data: proj})
        if (proj) {
            let cur_launch = proj.workspace.current_launch;
            if (cur_launch != null) {
                let found =
                    proj.workspace.launch_references.find(el => el.id == cur_launch)
                if (found !== undefined) {
                    launchStore.getState().set_current_launch(found!);
                }
            }
        }
    },
    opened: false,
    set_opened: (val: boolean) => set({opened: val}),



}))