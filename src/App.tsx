import {useEffect} from "react";
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
import ProjectSpace from "./components/pages/project-space/ProjectSpace.tsx";
import {projectStore} from "./stores/project_store.ts";


function App() {
    const current = pageStore(state => state.current);
    const createProjectOpened = createProjectStore(state => state.page_opened)
    const current_path = projectStore(state => state.path_to_current_project);

    const set_current_project = ()=>projectStore.getState().set_current_project
    const openProject = ()=>pageStore.getState().openProject
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

    async function setup_project() {
        if (current_path.length > 0) {
            try {
                let res = await invoke<IProject>("read_project", {
                    path: current_path
                })
                const set = set_current_project();
                set(res);
                const open = openProject()
                open();


            } catch (e) {
                console.error(e)
            }
        } else {
            console.log("path is empty")
        }
    }

    useEffect(() => {
        setup_project().then()

    }, [current_path]);


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
                {
                    current == Window.Project &&
                    <ProjectSpace/>
                }
            </div>
        </>
    );
}

export default App;
