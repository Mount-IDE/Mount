import {create} from "zustand";
import {createProjectStore} from "./create_project.ts";
import pageStore from "./page_store.ts";
import {invoke} from "@tauri-apps/api/core";
import {projectSettingsStore} from "./project_settings_store.ts";

interface Type {

    // file
    create_project(): void

    open_project(): void

    open_recent_project(): void

    close_project(): Promise<void>

    save_all(): void

    auto_save(): void

    project_settings(): void

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
        console.log("clicked")
        try {
            await invoke("unwatch_project");
            await invoke("close_window_terminals");
            pageStore.getState().openMain();
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