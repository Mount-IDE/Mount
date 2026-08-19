import React, {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import TitleBar from "./components/common/TitleBar.tsx";
import pageStore, {Window} from "./stores/page_store.ts";
import MainPage from "./components/pages/main-page/MainPage.tsx";
import Blur from "./components/common/Blur.tsx";
import {createProjectStore} from "./stores/create_project.ts";
import CreateProject from "./components/pages/create-project/CreateProject.tsx";
import ProjectSpace from "./components/pages/project-space/ProjectSpace.tsx";
import {cacheStore} from "./stores/cache_store.ts";
import {Group, mainPageStore} from "./stores/main_page_store.ts";
import {fsExtStore} from "./stores/fs_ext_store.ts";
import {projectSettingsStore} from "./stores/project_settings_store.ts";
import ProjectSettings from "./components/pages/project-space/ProjectSettings.tsx";
import {settingsStore} from "./stores/settings_store.ts";
import SettingsPage from "./components/pages/settings/SettingsPage.tsx";
import {themeStore} from "./stores/theme_store.ts";
import Notifications from "./components/common/Notifications.tsx";
import {packageStore} from "./stores/package_store.ts";


/**
 * Main component
 * @returns
 */
function App() {

    const current = pageStore(state => state.current);
    const createProjectOpened = createProjectStore(state => state.page_opened)
    const [windowReady, setWindowReady] = useState(false);

    const projectSettingsOpened = projectSettingsStore(state => state.opened)

    const settingsFlag = settingsStore(state => state.show_settings)

    /**
     * Caching many data while app is opening
     */
    async function move_to_cache() {
        try {
            const cache = await invoke<Cache>("get_cache");
            cacheStore.getState().set_data_dir(cache.data_dir_path);
            cacheStore.getState().set_file_templates(cache.file_templates);
            cacheStore.getState().set_os(cache.os);
            cacheStore.getState().set_projects_path(cache.projects_dir);
            cacheStore.getState().set_recent_projects(cache.recent_projects)
            mainPageStore.getState().set_groups(cache.groups.map((el, i): Group => ({
                id: i, name: el
            })));
            fsExtStore.getState().set_icons(cache.file_icons);
            packageStore.getState().set_package(cache.packages);
            cacheStore.getState().add_templates_to_cache(cache.templates);
            if (cache.templates.length > 0) {
                cacheStore.getState().set_current_template(cache.templates[0])
            }
            cacheStore.getState().set_shells(cache.shells);
            settingsStore.getState().set_settings(cache.settings)
            themeStore.getState().load_themes(cache.themes.map(e => JSON.parse(e) as ITheme), cache.settings)
        } catch (e) {
            console.warn(e)
        }


    }

    useEffect(() => {
        let cancelled = false;

        async function setupWindow() {
            try {
                await invoke("close_window_terminals");
            } catch (e) {
                console.error(e);
            }

            if (cancelled) return;

            setWindowReady(true);
            setTimeout(() => invoke("show_win").then(), 0);
            move_to_cache().then();
        }

        setupWindow().then();

        return () => {
            cancelled = true;
            invoke("close_window_terminals").catch((e) => console.error(e));
        };
    }, [])


    let currentTheme = themeStore(state => state.current_theme)


    useEffect(() => {
        //console.log("THEME", currentTheme)
        if (currentTheme) {
            let vars = [
                "bg",
                "bg1",
                "bg2",
                "title",
                "subtitle",
                "border",
                "border2",
                "border3",
                "proj-hover",
                "bg-t",
                "input",
                "placeholder",
            ];
            let root = document.querySelector(":root") as HTMLElement;
            if (root) {
                for (let i of vars) {
                    root.style.removeProperty(`--${i}`)
                }
                if (currentTheme.colors) {

                    for (let i of currentTheme.colors) {
                        //console.log("\ttheme", i.name)
                        if (vars.includes(i.name)) {
                            root.style.setProperty(`--${i.name}`, i.value)
                        }
                    }
                }
            }
            // location.reload()
        }


    }, [currentTheme]);
    /**
     * Setups cache and stores while project was selected
     */
    return (
        <>
            <Blur/>
            <TitleBar/>
            <Notifications/>
            <div id={"main"}>
                {
                    windowReady && settingsFlag &&
                    <SettingsPage/>
                }
                {
                    windowReady && projectSettingsOpened &&
                    <ProjectSettings/>
                }
                {
                    windowReady && createProjectOpened &&
                    <CreateProject/>
                }
                {
                    windowReady && current == Window.Main &&
                    <MainPage/>
                }
                {
                    windowReady && current == Window.Project &&
                    <ProjectSpace/>
                }
            </div>

        </>
    );
}

export default App;
