import { useEffect} from "react";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import TitleBar from "./components/common/TitleBar.tsx";
import pageStore from "./stores/page_store.ts";
import {Window} from "./stores/page_store.ts";
import MainPage from "./components/pages/main-page/MainPage.tsx";
import Blur from "./components/common/Blur.tsx";
import {createProjectStore} from "./stores/create_project.ts";
import CreateProject from "./components/pages/create-project/CreateProject.tsx";
import {cacheStore} from "./stores/cache_store.ts";
import {Group, mainPageStore} from "./stores/main_page_store.ts";



function App() {
    const current = pageStore(state => state.current);
    const createProjectOpened = createProjectStore(state => state.page_opened)

    async function move_to_cache() {
        try {
            let templates = await invoke<ITemplate[]>("read_templates");
            if (templates.length > 0) {
                cacheStore.getState().add_templates_to_cache(templates);
                let temp = templates[0];
                cacheStore.getState().set_current_template(temp);
            }
        } catch (e) {
            console.error("error while load templates: ", e)
        }
        try {
            let packages = await invoke<IPackage[]>("read_packages");
            cacheStore.getState().add_packages_to_cache(packages)
            // console.log(packages)
        } catch (e) {
            console.error("error while load packages: ", e)
        }

        try {
            let path = await invoke<string>("get_projects_dir")
            cacheStore.getState().set_projects_path(path);
        } catch (e) {
            console.error("error while load project path: ", e)

        }
        try {
            let groups = await invoke<string[]>("get_groups");
            let id = 0
            let n_groups: Group[] = groups.map(el => {
                return {
                    id: id++,
                    name: el
                }
            });
            mainPageStore.getState().set_groups(n_groups);
        } catch (e) {
            console.error("error while load tags: ", e)
        }

    }

    useEffect(() => {
        setTimeout(() => invoke("show_win").then(), 0)
        move_to_cache().then();
    }, [])

    return (
        <>
            <Blur/>
            <TitleBar/>
            <div id={"main"}>
                {
                    createProjectOpened &&
                        <CreateProject/>
                }
                {
                    current == Window.Main &&
                    <MainPage/>
                }
            </div>
        </>
    );
}

export default App;
