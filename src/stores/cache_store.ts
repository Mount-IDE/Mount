import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";
import {ERROR} from "../utils/utils.ts";
import {themeStore} from "./theme_store.ts";
import {Group, mainPageStore} from "./main_page_store.ts";
import {packageStore} from "./package_store.ts";
import {fsExtStore} from "./fs_ext_store.ts";


interface Type {
    packages: IPackage[]
    templates: ITemplate[]
    currentTemplate: ITemplate | null,
    groups: string[],
    data_dir: string,
    os: string,



    recent_projects: IRecentProject[]
    set_recent_projects: (rec: IRecentProject[]) => void

    set_os: (val: string) => void,

    make_path: (pieces: string[]) => string

    shells: string[]
    set_shells: (shells: string[]) => void
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
    set_file_templates: (temp: configFsTemplate[]) => void


    remove_from_recents: (path: string) => void


    add_recent: (rec: IRecentProject) => void
    update_recent: (rec: IRecentProject) => void


    update_cache: () => Promise<void>
}


export const cacheStore = create<Type>((set, get) => ({
    currentTemplate: null,
    packages: [],
    templates: [],
    file_templates: [],
    data_dir: "",
    projects_path: "",
    groups: [],


    os: "",
    update_recent(rec: IRecentProject): void {
        let path = rec.path;
        let recents = get().recent_projects;
        let path_ = get().make_path([path, rec.name])
        recents = recents.map(el => {
            if (get().make_path([el.path, el.name]) == path_) {
                return rec
            }
            return el
        })

        set({
            recent_projects: recents
        })

        invoke("update_recents", {projects: recents}).then()


    },

    add_recent(rec: IRecentProject): void {
        set({
            recent_projects: [rec, ...get().recent_projects]
        })
    },


    remove_from_recents(path): void {
        set({
            recent_projects: get().recent_projects.filter(el =>
                path != cacheStore.getState().make_path([el.path, el.name])
            )
        })
    },


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
    set_file_templates: async (temp: configFsTemplate[]) => {
        set({
            file_templates: temp
        })
    }, set_shells(shells: string[]): void {
        set({shells: shells})
    },
    shells: [],
    recent_projects: [],
    set_recent_projects(rec: IRecentProject[]): void {
        set({recent_projects: rec})
    },
    make_path(pieces: string[]): string {
        if (pieces.length == 0) {
            return ""
        }
        let res = "";
        let os = get().os;
        for (let i = 0; i < pieces.length - 1; i++) {
            res += `${pieces[i]}${os == "windows" ? "\\" : "/"}`
        }
        res += pieces[pieces.length - 1]
        return res;
    },
    async update_cache(): Promise<void> {
        try {
            let cache = await invoke<Cache>("get_cache");
            themeStore.getState().load_themes(
                cache.themes.map(el => JSON.parse(el) as ITheme),
                cache.settings)
            mainPageStore.getState().set_groups(
                cache.groups.map<Group>(
                    (el, i) => ({id: i, name: el})
                ))
            packageStore.getState().set_package(cache.packages)
            fsExtStore.getState().set_icons(cache.file_icons)

            set({
                data_dir: cache.data_dir_path,
                file_templates: cache.file_templates,
                os: cache.os,
                projects_path: cache.projects_dir,
                recent_projects: cache.recent_projects,
                templates: cache.templates,
                shells: cache.shells
            })
        } catch (e) {
            ERROR(e)
        }
    }


}))
