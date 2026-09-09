import {create} from "zustand";
import {createProjectStore} from "./create_project.ts";
import pageStore from "./page_store.ts";
import {invoke} from "@tauri-apps/api/core";
import {projectSettingsStore} from "./project_settings_store.ts";
import {settingsStore} from "./settings_store.ts";
import {projectStore} from "./project_store.ts";
import {asideButtonsStore} from "./aside_buttons_store.ts";
import {asideStore} from "./aside_store.ts";
import {codeSpaceStore} from "./code_space_store.ts";
import {fileCacheStore} from "./file_cache_store.ts";
import {fsAsideTreeStore} from "./fs_aside_tree_store.ts";
import {languageStore} from "./language_store.ts";
import {launchStore} from "./launch_store.ts";
import {treeStore} from "./tree_store.ts";
import {menuStore} from "./menu_store.ts";

interface Type {

    // file
    create_project(): void

    open_project(): void

    open_recent_project(): void

    close_project(): Promise<void>

    save_all(): void

    auto_save(): void

    project_settings(): void

    global_settings(): void

    exit(): void


    //edit
    undo(): void

    redo(): void

    cut(): void

    copy(): void

    copy_path(): void

    paste(): void

    duplicate_line(): void

    //view


    //code
    inspect(): void

    analyze(): void

    reformat(): void

}


export const menuBarStore = create<Type>(() => ({
    create_project(): void {
        pageStore.getState().setFilter(true);
        createProjectStore.getState().open();
    },
    open_project(): void {
    },
    open_recent_project(): void {
    },
    async close_project(): Promise<void> {


        async function clear() {
            await invoke("unwatch_project");
            await invoke("close_window_terminals");
            await invoke("close_window_launches")

            projectStore.getState().close_project()
            asideButtonsStore.getState().clear()
            asideStore.getState().clear()
            codeSpaceStore.getState().clear()
            fileCacheStore.getState().clear()
            await fsAsideTreeStore.getState().unwatch()
            fsAsideTreeStore.getState().clear()
            languageStore.getState().clear()
            launchStore.getState().clear()
            treeStore.getState().clear()
            pageStore.getState().openMain();
        }


        function load_proc() {
            menuStore.getState().open_modal({
                typ: "confirm",
                title: "There some opened processes in project",
                buttons: [
                    {
                        typ: "cancel",
                        title: "Terminate",
                        cb: () => {

                            clear()
                        }
                    },

                ]
            })
        }
        try {

            let is_dirty = fileCacheStore.getState().has_one_dirty();
            let has = await invoke<[boolean, boolean]>("has_processes");
            if (is_dirty || has[0] || has[1]) {
                if (is_dirty) {
                    menuStore.getState().open_modal({
                        typ: "confirm",
                        title: "Some files aren`t save. Save it?",
                        buttons: [
                            {
                                typ: "cancel",
                                title: "Save",
                                cb: () => {
                                    fileCacheStore.getState().save_all()
                                    if (has[0] || has[1]) {
                                        load_proc()
                                    } else {
                                        clear()
                                    }
                                }
                            }
                        ],

                    })
                    return
                }
                if (has[0] || has[1]) {
                    load_proc()
                    return
                }
            }
            await clear()

        } catch (e) {
            console.error(e)
        }
    },
    save_all(): void {
    },
    auto_save(): void {
    },
    project_settings(): void {
        projectSettingsStore.getState().set_opened(true)
        pageStore.getState().setFilter(true)
    },

    global_settings() {
        pageStore.getState().setFilter(true)
        settingsStore.getState().set_show_settings(true)
    },
    exit(): void {
    },


    undo(): void {
    },
    redo(): void {
    },
    cut(): void {
    },
    copy(): void {
    },
    copy_path(): void {
    },
    paste(): void {
    },
    duplicate_line(): void {
    },
    inspect(): void {
    },
    analyze(): void {
    },
    reformat(): void {
    }

}))