import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";


interface Type {
    packages: IPackage[]
    templates: ITemplate[]
    currentTemplate: ITemplate | null,
    groups: string[],
    data_dir: string,
    os: string,
    set_os: (val:string)=>void,
    add_template_to_cache: (t: ITemplate) => void
    add_templates_to_cache: (t: ITemplate[]) => void
    add_package_to_cache: (t: IPackage) => void
    add_packages_to_cache: (t: IPackage[]) => void
    clear_templates: () => void
    clear_packages: () => void
    clear_current_template: () => void
    set_current_template: (t: ITemplate) => void,
    projects_path: string
    set_projects_path: (path: string) => void
    load_groups: (groups: string[]) => void
    set_data_dir: (str: string) => void,
    file_templates: configFsTemplate[],
    load_file_templates: ()=> Promise<void>
}


export const cacheStore = create<Type>((set, _) => ({
    currentTemplate: null,
    packages: [],
    templates: [],
    file_templates: [],
    data_dir: "",
    projects_path: "",
    groups: [], 
    os: "",
    set_os(val: string): void {
        set({
            os: val
        })
    },
    add_package_to_cache: (t: IPackage) => set(prev => {
        const pack = prev.packages;
        if (!pack.map(el => el.id).includes(t.id)) {
            pack.push(t);
        }
        return {
            packages: pack
        }
    }),
    add_template_to_cache: (t: ITemplate) => set(prev => {
        const temp = prev.templates;
        if (!temp.map(el => el.id).includes(t.id)) {
            temp.push(t);
        }
        return {
            templates: temp
        }
    }),
    clear_current_template: () => set({
        currentTemplate: null
    }),
    clear_packages: () => set({
        packages: []
    }),
    clear_templates: () => set({
        templates: []
    }),
    add_packages_to_cache: (t: IPackage[]) => set(prev => {

        const packs = prev.packages;
        let id = packs.map(el => el.id)
        for (let i of t) {
            if (!id.includes(i.id)) {
                packs.push(i)
                id = packs.map(el => el.id)
            }
        }
        return {
            packages: packs
        }
    }),
    add_templates_to_cache: (t: ITemplate[]) => set(prev => {
        const temps = prev.templates;
        let id = temps.map(el => el.id)
        for (let i of t) {
            if (!id.includes(i.id)) {
                temps.push(i)
                id = temps.map(el => el.id)
            }
        }
        return {
            templates: temps
        }
    }),
    set_current_template: (t: ITemplate) => set({currentTemplate: t}),
    set_projects_path: (path: string) => set({projects_path: path}),
    load_groups: (groups: string[]) => set({
        groups
    }),
    set_data_dir(str: string): void {
        set({
            data_dir: str
        })
    },
    load_file_templates: async (): Promise<void>=> {
        try{
            const data = await invoke<configFsTemplate[]>("get_file_templates");
            set({
                file_templates: data
            })
        }catch (e){
            console.error(e)
        }
    }


}))
