import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";


interface Type{
    path_to_current_project: string
    current_project: IProject| null,
    set_path_to_current_project(path: string):void,
    set_current_project(proj: IProject|null):void

    save_project(proj: IProject): void
}


export const projectStore = create<Type>((set, _) => ({
    path_to_current_project: "",
    current_project: null,
    set_path_to_current_project:(path: string)=>set({
        path_to_current_project:path
    }),
    set_current_project(proj: IProject | null): void {
        set({
            current_project:proj
        })
    }, save_project(proj: IProject): void {
        set({current_project: proj})
        invoke("save_project", {project: proj})
    }


}))