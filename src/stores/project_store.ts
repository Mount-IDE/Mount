import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";


interface Type {
    path_to_current_project: string
    current_project: IProject | null,
    add_launch_reference: (temp: LaunchTemplate, temp2: ITemplate) => number
    update_launch_references: (refs: LaunchTemplateReference[]) => Promise<void>
    update_launch_objects: (template: LaunchTemplate) => Promise<boolean>

    set_path_to_current_project(path: string): void,

    set_current_project(proj: IProject | null): void,

    save_project(proj: IProject): void

    set_current_launch: (id: number) => void

}


export const projectStore = create<Type>((set, get) => ({
    async update_launch_objects(template: LaunchTemplate): Promise<boolean> {
        let project = get().current_project;
        if (project) {
            let refs = project.workspace.launch_references;
            try {
                let res = await invoke<[IProject, LaunchObject[]]>("create_objects", {
                    references: refs,
                    template: template,
                    vars: project.vars,
                    project: project
                });
                set({
                    current_project: res[0]
                })
                return true
            } catch (e) {
                console.error(e)
            }
        }
        return false
    },
    path_to_current_project: "",
    current_project: null,
    set_path_to_current_project: (path: string) => set({
        path_to_current_project: path
    }),
    set_current_project(proj: IProject | null): void {
        set({
            current_project: proj
        })
    }, save_project(proj: IProject): void {
        set({current_project: proj})
        invoke("save_project", {project: proj}).then()
    },
    add_launch_reference(temp: LaunchTemplate, temp2: ITemplate): number {
        let proj = get().current_project;
        if (proj) {
            let last_id = 0
            let len = proj.workspace.launch_references.length
            if (len > 0) {
                last_id = proj.workspace.launch_references[len - 1].id + 1
            }
            set({
                current_project: {
                    ...proj,
                    workspace: {
                        ...proj?.workspace,
                        launch_references: [...proj?.workspace.launch_references,
                            {
                                id: last_id,
                                name: "Unnamed",
                                results: {},
                                scheme: 0,
                                template: [temp2.id, temp.id]

                            }]
                    }
                }
            })
            return last_id
        }
        return -1
    },
    async update_launch_references(refs: LaunchTemplateReference[]): Promise<void> {
        let project = get().current_project
        if (project) {
            let proj = structuredClone(project!)
            proj.workspace.launch_references = refs;
            for (let i = 0; i < proj.workspace.launch_references.length; i++) {
                if (Object.keys(proj.workspace.launch_references[i].results).includes("-1")) {
                    let found = proj.workspace.launch_references[i].results[-1]
                    if (Object.keys(found).includes("name")) {
                        proj.workspace.launch_references[i].name = found["name"];
                    }
                }

            }
            try {
                await invoke("save_project", {project: proj});
                set({current_project: proj})
            } catch (e) {
                console.error(e)
            }
        }
    },
    set_current_launch(id: number): void {
        let proj = get().current_project;
        if (proj) {

            set({
                current_project: {
                    ...proj,
                    workspace: {
                        ...proj.workspace,
                        current_launch: id
                    }
                }

            })
        }
    }


}))