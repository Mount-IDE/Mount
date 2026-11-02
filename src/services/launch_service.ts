import {projectStore} from "@stores/project_store.ts";
import {invoke} from "@tauri-apps/api/core";
import {launchStore} from "@stores/launch_store.ts";

export const enum LaunchResult {
    Done,
    ProjectNotFound,
    ReferenceNotFound,
    ReferenceAlreadyExists,
    CompileLaunchObjects,
    ReferenceIsSame,
    ReferenceNotSelected
}


interface Type {

    change_ref: (new_: LaunchTemplateReference) => LaunchResult
    add_ref: (ref: LaunchTemplateReference) => LaunchResult
    remove_ref: (id: number) => LaunchResult


    set_current_launch: (id: number | null) => LaunchResult


    update_launch_objects: () => Promise<[LaunchResult, IProject | null]>


    write_ref_result: (ref: number, sec: number, opt: string, value: IVal) => LaunchResult

    find_result: (ref: number, section: number, option: string) => IVal | undefined


    remove_active_object: (id: number) => LaunchResult


    run_launch: (launch: LaunchObject) => LaunchResult


    add_before_launch: (launch: LaunchReferenceBefore) => LaunchResult
    rem_before_launch_by_id: (id: number) => LaunchResult
    rem_before_launch_by_index: (index: number) => LaunchResult
}


export class LaunchService implements Type {
    add_before_launch(launch: LaunchReferenceBefore): LaunchResult {
        let proj = projectStore.getState().current_project
        if (!proj) return LaunchResult.ProjectNotFound
        let cur = proj.workspace.current_launch
        if (cur == null) return LaunchResult.ReferenceNotSelected
        let id = launch.typ == "custom" ? null : launch.id
        if (id != null && id == cur) return LaunchResult.ReferenceIsSame
        let references = proj.workspace.launch_references
        let found = references.findIndex(el => el.id == cur)
        if (found == -1) return LaunchResult.ReferenceNotFound
        if (references[found].before == undefined) {
            references[found].before = []
        }
        if (id == null) {
            references[found].before.push(launch)
        } else {
            let f = references[found].before.find(el => el.typ == "other" && el.id == id)
            if (f) {
                return LaunchResult.ReferenceAlreadyExists
            }
            references[found].before.push(launch)
        }
        projectStore.setState({
            current_project: {
                ...proj,
                workspace: {
                    ...proj.workspace,
                    launch_references: [...references]
                }
            }
        })
        return LaunchResult.Done;
    }

    rem_before_launch_by_id(id: number): LaunchResult {
        let proj = projectStore.getState().current_project
        if (!proj) return LaunchResult.ProjectNotFound

        let cur = proj.workspace.current_launch
        if (cur == null) return LaunchResult.ReferenceNotSelected

        let refs = proj.workspace.launch_references
        let ref = refs.findIndex(el => el.id == cur)
        if (ref == -1) return LaunchResult.ReferenceNotFound

        let i = refs[ref].before?.findIndex(el => el.typ == "other" && el.id == id)
        if (i == -1 || i == undefined) return LaunchResult.ReferenceNotFound

        refs[ref].before?.splice(i, 1);
        projectStore.setState({
            current_project: {
                ...proj,
                workspace: {
                    ...proj.workspace,
                    launch_references: [...refs]
                }
            }
        })
        return LaunchResult.Done;
    }

    rem_before_launch_by_index(index: number): LaunchResult {
        if (index == -1) {
            return LaunchResult.ReferenceNotFound
        }
        let proj = projectStore.getState().current_project
        if (!proj) return LaunchResult.ProjectNotFound

        let cur = proj.workspace.current_launch
        if (cur == null) return LaunchResult.ReferenceNotSelected


        let refs = proj.workspace.launch_references
        let ref = refs.findIndex(el => el.id == cur)
        if (ref == -1) return LaunchResult.ReferenceNotFound
        refs[ref].before?.splice(index, 1)
        projectStore.setState({
            current_project: {
                ...proj,
                workspace: {
                    ...proj.workspace,
                    launch_references: [...refs]
                }
            }
        })
        return LaunchResult.Done
    }



    run_launch(launch: LaunchObject): LaunchResult {

        launchStore.setState({
            current_obj: launch
        })
        let objs = [...launchStore.getState().active_objects]
        if (!objs.find(el => el.id == launch.id)) {
            objs.push(launch)
            launchStore.setState({
                active_objects: new Set(objs)
            })
        }
        return LaunchResult.Done;
    }


    add_ref(ref: LaunchTemplateReference) {
        let project = projectStore.getState().current_project
        if (!project) {
            return LaunchResult.ProjectNotFound
        }

        let refs = project!.workspace.launch_references
        if (refs.find(el => el.id == ref.id) != undefined) {
            return LaunchResult.ReferenceAlreadyExists
        }
        let res = [...refs, ref]
        projectStore.setState({
            current_project: {
                ...project,
                workspace: {
                    ...project.workspace,
                    launch_references: res
                }
            }
        })
        return LaunchResult.Done

    }

    change_ref(new_: LaunchTemplateReference) {
        let id = new_.id;
        let project = projectStore.getState().current_project;
        if (!project) {
            return LaunchResult.ProjectNotFound
        }
        let found = project.workspace.launch_references.findIndex(el => el.id == id)
        if (found == -1) {
            return LaunchResult.ReferenceNotFound
        }
        let refs = [...project.workspace.launch_references]
        refs[found] = new_
        projectStore.setState({
            current_project: {
                ...project,
                workspace: {
                    ...project.workspace,
                    launch_references: [...refs]
                }
            }
        })
        return LaunchResult.Done

    }

    remove_ref(id: number) {
        let project = projectStore.getState().current_project
        if (!project) {
            return LaunchResult.ProjectNotFound
        }
        let refs = project.workspace.launch_references
        let i = refs.findIndex(e => e.id == id)
        if (i == -1) {
            return LaunchResult.ReferenceNotFound
        }
        refs.splice(i, 1)
        projectStore.setState({
            current_project: {
                ...project,
                workspace: {
                    ...project.workspace,
                    launch_references: [...refs]
                }
            }
        })
        return LaunchResult.Done
    }

    set_current_launch(id: number | null): LaunchResult {
        let proj = projectStore.getState().current_project
        if (!proj) return LaunchResult.ProjectNotFound
        if (id == null) {
            projectStore.setState({
                current_project: {
                    ...proj,
                    workspace: {
                        ...proj.workspace,
                        current_launch: null
                    }
                }
            })
            return LaunchResult.Done
        }
        let refs = proj.workspace.launch_references
        let found = refs.find(el => el.id == id)
        if (!found) return LaunchResult.ReferenceNotFound
        projectStore.setState({
            current_project: {
                ...proj,
                workspace: {
                    ...proj.workspace,
                    current_launch: id
                }
            }
        })
        return LaunchResult.Done

    }

    async update_launch_objects(): Promise<[LaunchResult, IProject | null]> {
        let project = projectStore.getState().current_project
        if (!project) return [LaunchResult.ProjectNotFound, null]
        try {
            let res = await invoke<[IProject, LaunchObject[]]>(
                "create_objects", {
                    project
                })
            projectStore.setState({
                current_project: {
                    ...res[0],
                    workspace: {
                        ...res[0].workspace,
                        launch_objects: res[1]
                    }
                }
            })
            return [LaunchResult.Done, res[0]]
        } catch (e) {
            console.error(e)
            return [LaunchResult.CompileLaunchObjects, null]
        }


    }

    remove_active_object(id: number): LaunchResult {
        launchStore.setState(prev => {
            let obj = prev.active_objects
            let found = [...obj.values()].find(el => el.id == id)
            if (found) {
                obj.delete(found)
                return {
                    active_objects: new Set(obj)
                }
            }
            return prev
        })
        return LaunchResult.Done;
    }


    find_result(ref: number, section: number, option: string): IVal | undefined {
        let refs = launchStore.getState().copy_references
        let ref_ = refs.find(el => el.id == ref)
        if (!ref_) return undefined
        let opt = ref_.results[section]?.[option]
        return opt
    }

    write_ref_result(ref: number, sec: number, opt: string, value: IVal): LaunchResult {
        let refs = launchStore.getState().copy_references
        let ref_ = refs.findIndex(el => el.id == ref)
        if (ref_ == -1) return LaunchResult.ReferenceNotFound
        let r = refs[ref_]
        r.results = {
            ...r.results,
            [sec]: {
                ...r.results[sec],
                [opt]: value
            }
        }
        refs[ref_] = r
        let changed = launchStore.getState().changed_references
        changed.add(r.id)
        launchStore.setState({
            copy_references: [...refs],
            changed_references: new Set(changed)
        })
        return LaunchResult.Done
    }

}


const launchService = new LaunchService()
export default launchService