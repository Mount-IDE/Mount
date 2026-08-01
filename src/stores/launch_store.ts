import {create} from "zustand";
import {invoke} from "@tauri-apps/api/core";
import {projectStore} from "./project_store.ts";
//import {asideLaunchStore} from "./aside_launch_store.ts";
import {LOG} from "../utils/utils.ts";


interface Type {
    current_obj: LaunchObject | null,
    opened: boolean,

    current_launch: LaunchTemplateReference | null
    set_current_launch: (cur: LaunchTemplateReference | null) => void
    temp_results: LaunchTemplateResult | null,
    set_temp_results: (temp: LaunchTemplateResult) => void
    write_temp: (section: number, option: string, value: string, ref: number, project: IProject | null) => void
    find_temp: (section: number, option: string, ref: number, project: IProject | null) => string | null
    set_opened: (val: boolean) => void,
    current_template: LaunchTemplate | null,
    set_current_template: (obj: LaunchTemplate | null) => void

    set_current_temp_by_ref: (obj: LaunchTemplateReference, templates: LaunchTemplate[]) => void
    compile_to_obj: (ref: LaunchTemplateReference, results: LaunchTemplateResult, template: LaunchTemplate) => Promise<LaunchObject | null>,
    run_launch: (launch: LaunchObject) => void
    compile_to_ref: (temp: LaunchTemplate, results: LaunchTemplateResult) => Promise<LaunchTemplateReference | null>

    active_objects: Set<LaunchObject>,

    add_active_object: (obj: LaunchObject) => void
    remove_active_object: (obj: LaunchObject) => void
}


export const launchStore = create<Type>((set, get) => ({
    current_obj: null,
    current_template: null,
    opened: false,
    temp_results: null,
    current_launch: null,
    active_objects: new Set(),
    remove_active_object: (obj) => set(prev => {
        let other = new Set([...prev.active_objects])
        other.delete(obj)
        return {
            active_objects: other
        }
    }),
    add_active_object: (obj) => set(prev => {
        let other = new Set([...prev.active_objects])
        other.add(obj);
        return {
            active_objects: other
        }
    }),

    set_current_launch(cur: LaunchTemplateReference | null): void {
        set({current_launch: cur})
    },


    set_current_template(obj: LaunchTemplate | null): void {
        set({current_template: obj})
    },

    set_temp_results: (temp) => set({temp_results: temp}),
    find_temp(section: number, option: string, ref: number, project: IProject | null): string | null {
        if (!project) return null
        let temp = project.workspace.launch_references ?? [];
        if (temp.length == 0) {
            return null
        }
        let found = temp.find(el => el.id == ref);
        if (!found)
            return null
        let results = found.results;
        //   LOG(`RESULTS ${JSON.stringify(results)} :: SECTION ${section} :: OPTION ${option}`)
        //  LOG(`BOOL ${section.toString() in results} :: ${option in results[section.toString()]}`)
        if (`${section}` in results) {
            let sec = results[section];
            if (option in sec) {
                //    LOG("OPTION ", sec[option], JSON.stringify(sec))
                return sec[option]
            }
        }
        return null

    },
    write_temp(section: number, option: string, value: string, ref: number, project: IProject | null): void {
        LOG("WRITING ", section, option, value)
        if (!project) return

        let temp = project.workspace.launch_references;
        let found =
            temp.find(el => el.id == ref);
        LOG("found", found)
        if (!found) return
        let results = found.results;
        if (results[section]?.[option] === value) {
            return;
        }
        let res: LaunchTemplateReference = {
            ...found,
            results: {
                ...results,
                [section]: {
                    ...results[section],
                    [option]: value
                }
            }
        }
        let result = temp.map(el => {
            if (el.id == ref) {
                return res
            } else {
                return el
            }
        })
        const proj: IProject = {
            ...project,
            workspace: {
                ...project.workspace,
                launch_references: result
            }
        }
        LOG("RESULT AFTER WRITING", JSON.stringify(result))
        projectStore.getState().set_current_project(proj)

    },

    set_opened(val: boolean): void {
        set({opened: val})
    },
    run_launch(launch: LaunchObject): void {
        const project = projectStore.getState().current_project;
        const reference =
            project?.workspace.launch_references.find((ref) => ref.id === launch.launch_reference);
        // asideLaunchStore.getState().start_launch(launch, reference?.name ?? `Launch ${launch.launch_reference}`);
        set({current_obj: launch});
        let objs = [...get().active_objects];
        let found = objs.find(el => el.id == launch.id);
        console.log('BEBE', found, objs)
        if (!found) {
            objs.push(launch);
            set({active_objects: new Set(objs)})
        }


    },
    async compile_to_obj(ref: LaunchTemplateReference,
                         results: LaunchTemplateResult,
                         template: LaunchTemplate): Promise<LaunchObject | null> {
        const proj = projectStore.getState().current_project
        if (proj) {
            try {
                let res =
                    await invoke<[IProject, LaunchObject]>("create_object", {
                        reference: ref,
                        results: results,
                        template: template,
                        vars: proj.vars,
                        project: proj
                    });
                projectStore.getState().set_current_project(res[0])
                return res[1]
            } catch (e) {
                console.error(e)
            }
        }
        return null

    },

    async compile_to_ref(temp: LaunchTemplate, results: LaunchTemplateResult): Promise<LaunchTemplateReference | null> {
        const proj = projectStore.getState().current_project
        if (proj) {
            try {
                let res =
                    await invoke<[IProject, LaunchTemplateReference]>("create_ref", {
                        results: results,
                        template: proj.template,
                        l_template: temp,
                        project: proj
                    });
                projectStore.getState().set_current_project(res[0])
                return res[1]
            } catch (e) {
                console.error(e)
            }
        }
        return null
    },

    set_current_temp_by_ref(obj: LaunchTemplateReference, templates: LaunchTemplate[]): void {
        let id = obj.template[1];
        let found = templates.find(el => el.id == id)
        if (found) {
            set({
                current_template: found!
            })
        }
    }


}))
