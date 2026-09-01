import {create} from "zustand"
import {invoke} from "@tauri-apps/api/core";
import {loadPackages} from "../utils/utils.ts";
import {asideButtonsStore} from "./aside_buttons_store.ts";
import {mapProjectButton} from "../utils/project-buttons.ts";
import {cacheStore} from "./cache_store.ts";
import pageStore from "./page_store.ts";


interface Type {
    path_to_current_project: string
    current_project: IProject | null,
    add_launch_reference: (temp: LaunchTemplate, temp2: ITemplate) => number
    update_launch_references: (refs: LaunchTemplateReference[]) => Promise<void>
    update_launch_objects: (template: LaunchTemplate) => Promise<[boolean, IProject | null]>

    set_path_to_current_project(path: string): void,

    set_current_project(proj: IProject | null): void,

    save_project(proj: IProject): void

    set_current_launch: (id: number) => void

    package_configs: Map<string, PackageConfig>
    selected_packages: Map<string, PackageInner>

    set_selected_packages: (map: Map<string, PackageInner>) => Promise<void>


    open_project: (proj: IProject) => Promise<void>


    get_pack_by_file: (file: string | null) => PackageInner | null
}


export const projectStore = create<Type>((set, get) => ({
    selected_packages: new Map(),
    package_configs: new Map(),
    async set_selected_packages(map: Map<string, PackageInner>): Promise<void> {
        let configs = new Map<string, PackageConfig>()
        console.log(map)
        for (let i of map.entries()) {
            let blob = new Blob([i[1].config], {type: "text/javascript"})

            let url = URL.createObjectURL(blob)
            try {
                let module = await import(url) as PackageConfig;
                configs.set(i[0], module)
                console.debug(module)
            } finally {
                URL.revokeObjectURL(url)
            }

        }

        set({
            selected_packages: map,
            package_configs: configs
        })
    },
    async update_launch_objects(template: LaunchTemplate): Promise<[boolean, IProject | null]> {
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
                return [true, res[0]]
            } catch (e) {
                console.error(e)
            }
        }
        return [false, null]
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
    },


    open_project: async (proj) => {
        let packs = await loadPackages(proj.packages);
        const buttons = proj.workspace.buttons;
        let left_top = buttons.filter(el => el.pos == "LeftTop")
        let left_bot = buttons.filter(el => el.pos == "LeftBottom")
        let right_top = buttons.filter(el => el.pos == "RightTop")

        let left_top_2 =
            left_top.map<IAsideButton>(mapProjectButton);

        let left_bot_2 =
            left_bot.map<IAsideButton>(mapProjectButton);

        let right_top_2 =
            right_top.map<IAsideButton>(mapProjectButton);
        asideButtonsStore.getState().load_left(left_top_2);
        asideButtonsStore.getState().load_bottom(left_bot_2);
        asideButtonsStore.getState().load_right(right_top_2);
        try {
            await invoke("unwatch_project");
            await invoke("close_window_terminals");
        } catch (e) {
            console.error(e)
        }

        let rec = cacheStore.getState().recent_projects.find(el => el.path == proj.path);
        if (!rec) {
            cacheStore.getState().add_recent({
                last_opened: new Date().getTime(),
                meta: {...proj.meta},
                name: proj.name,
                packages: [...proj.packages],
                path: proj.path

            } satisfies IRecentProject)
        } else {
            rec.last_opened = new Date().getTime()
            cacheStore.getState().update_recent(rec)
        }
        pageStore.getState().openProject();
        set({
            selected_packages: packs ?? new Map(),
            current_project: proj,
            path_to_current_project: cacheStore.getState().make_path([proj.path, proj.name])
        })
    },
    get_pack_by_file(file: string | null): PackageInner | null {
        if (!file) return null
        let packs = [...get().selected_packages]
        let pack = packs.find(el => {
            for (let i of el[1].main.files.extentions) {
                if (file.endsWith(i)) return true
            }
            return false
        })
        if (!pack) {
            return null
        }
        return pack[1]
    }


}))