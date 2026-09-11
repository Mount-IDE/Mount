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
import {projectSettingsStore} from "./stores/project_settings_store.ts";
import ProjectSettings from "./components/pages/project-space/ProjectSettings.tsx";
import {settingsStore} from "./stores/settings_store.ts";
import SettingsPage from "./components/pages/settings/SettingsPage.tsx";
import {themeStore} from "./stores/theme_store.ts";
import Notifications from "./components/common/Notifications.tsx";
import {Parser} from "web-tree-sitter";
import {highlightWorkerStore} from "./stores/highlight_worker_store.ts";
import {AnimatePresence} from "motion/react";


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

    useEffect(() => {
        highlightWorkerStore.getState().init()


        let cancelled = false;

        async function setupWindow() {
            // noinspection JSUnusedGlobalSymbols
            await Parser.init({

                locateFile: (path: string) => "/" + path
            })
            try {
                await invoke("close_window_terminals");
            } catch (e) {
                console.error(e);
            }

            if (cancelled) return;

            setWindowReady(true);
            setTimeout(() => invoke("show_win").then(), 0);
            cacheStore.getState().update_cache().then()
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
                "files-bg"
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


    const blur = pageStore(state => state.need_filter)
    /**
     * Setups cache and stores while project was selected
     */
    return (
        <>
            {blur && <Blur/>}
            <TitleBar/>
            <Notifications/>
            <div id={"main"}>
                <AnimatePresence>
                    {
                        windowReady && settingsFlag &&
                        <SettingsPage/>
                    }

                </AnimatePresence>

                <AnimatePresence>
                    {
                        windowReady && projectSettingsOpened &&
                        <ProjectSettings/>
                    }
                </AnimatePresence>
                <AnimatePresence>
                    {
                        windowReady && createProjectOpened &&
                        <CreateProject/>
                    }
                </AnimatePresence>
                <AnimatePresence>
                    {
                        windowReady && current == Window.Main &&
                        <MainPage/>
                    }
                </AnimatePresence>
                <AnimatePresence>
                    {
                        windowReady && current == Window.Project &&
                        <ProjectSpace/>
                    }
                </AnimatePresence>
            </div>

        </>
    );
}

export default App;
