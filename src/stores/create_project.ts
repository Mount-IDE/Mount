import {create} from "zustand";
import {invoke} from "@tauri-apps/api/core";
import {projectStore} from "./project_store.ts";
import pageStore from "./page_store.ts";


interface Type {
    page_opened: boolean;
    open: () => void;
    close: () => void;
    results: Result,
    tags: { id: number, name: string }[]
    packages: Set<string>
    add_result: (tid: string, sid: number, pid: string, value: string | boolean | string[]) => void,
    // get_result:(tid: string, sid:number, pid: string)=>string | boolean | undefined,
    // has_result: (from:string)=>boolean,
    add_package: (id: string) => void
    add_packages: (pack: string[]) => void
    remove_package: (pack: string) => void
    add_tag: (name: string) => void
    remove_tag: (id: number) => void
    change_tag: (id: number, to: string) => void

    package_results: Record<string, Record<string, IVal>>

    add_pack_result: (pack: string, param: string, val: IVal) => void

    create_project(template: ITemplate): Promise<[number, string, IProject | null]>

}

export interface Result {
    [template: string]: {
        [section: number]: {
            [parameter: string]: string | boolean | string[]
        }
    }
}


export const createProjectStore = create<Type>((set, get) => ({
    void: undefined,
    page_opened: false,
    results: {},
    packages: new Set(),
    tags: [],
    package_results: {},
    add_pack_result(pack: string, param: string, val: IVal): void {
        let prev = get().package_results
        set({
            package_results: {
                ...prev,
                [pack]: {
                    ...prev[pack],
                    [param]: val
                }
            }
        })
    },


    close: () => set({page_opened: false}),
    open: () => set({page_opened: true}),
    add_result: (tid: string, sid: number, pid: string, value) => set(prev => {
        const results = prev.results;

        return {
            results: {
                ...results,
                [tid]: {
                    ...results[tid],
                    [sid]: {
                        ...results[tid]?.[sid],
                        [pid]: value
                    }
                }
            }
        }
    }),
    add_package: (id: string) => set(prev => {
        const newMap = new Set(prev.packages.values());
        newMap.add(id);
        return {packages: newMap};

    }),
    add_packages: (pack: string[]) => set(prev => {
        const newMap = new Set(prev.packages.values());
        for (let i of pack) {
            newMap.add(i);
        }
        return {
            packages: newMap
        }
    }),
    remove_package: (pack: string) => set(prev => {
        const newMap = new Set(prev.packages.values());
        newMap.delete(pack);
        return {packages: newMap};
    }),
    add_tag: (name: string) => set(prev => {
        const id = prev.tags.length > 0 ? prev.tags[prev.tags.length - 1].id : 0
        return {
            tags: [...prev.tags, {id: id + 1, name}]
        }
    }),
    remove_tag: (id: number) => set(prev => {
        return {
            tags: prev.tags.filter(el => el.id != id)
        }
    }),
    change_tag: (id: number, to: string) => set(prev => {
        return {
            tags: prev.tags.map(el => el.id == id ? {id, name: to} : el)
        }
    }),
    async create_project(template: ITemplate) {
        let results = get().results;
        let packages = get().packages;
        let tags = get().tags;
        if (!template) {
            return [1, "", null]; // undefined template
        }
        try {
            const project = await invoke<IProject>("create_project", {
                template, results, packages: [...packages.values()], tags, packResults: get().package_results
            })
            let name = get().results?.["__meta__"]?.[-4]?.["project-name"];
            let path = get().results?.["__meta__"]?.[-4]?.["project-path"];

            let unified = await invoke<string>("make_path_command", {
                components: [path, name]
            })
            projectStore.getState().set_current_project(project);
            pageStore.getState().openProject();
            pageStore.getState().setFilter(false);

            return [0, unified, project]
        } catch (e) {
            console.warn(e)
            return [2, "", null] //error while create project
        }


    }


}))
