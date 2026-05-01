import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";
import ProjectTemplates from "./ProjectTemplates.tsx";
import ProjectMeta from "./ProjectMeta.tsx";
import ProjectPackages from "./ProjectPackages.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {invoke} from "@tauri-apps/api/core";

export default function CreateProject() {

    const create_project = createProjectStore(state => state.create_project)
    const current_template = cacheStore(state => state.currentTemplate)
    const set_current_path = projectStore(state=>state.set_path_to_current_project);
    const getPath = () =>
        createProjectStore.getState()
            .results?.["__meta__"]?.[-4]?.["project_path"];

    const getName = () =>
        createProjectStore.getState()
            .results?.["__meta__"]?.[-4]?.["project_name"];

    async function create_project_() {
        const path = getPath();
        const name = getName();

        if (current_template) {
            let res = await create_project(current_template!);

            if (res == 0) {
                let path__ = await invoke<string>("make_path_command", {
                    components: [path, name]
                })
                set_current_path(path__);
            }
        }
    }

    return (
        <div
            id={"create-project"}>
            <div id={"create-project-top"}>
                <div id={"create-project-top-label"}>Create Project</div>
            </div>
            <div id={"create-project-main"}>
                <ProjectTemplates/>
                <hr id={"create-project-hr"}/>
                <ProjectMeta/>
                <ProjectPackages/>
            </div>
            <div id={"create-project-bottom"}>
                <div id={"create-project-buttons"}>
                    <Button title={"Close"} cb={() => close_project()}/>
                    <Button title={"Create Project"} cb={()=>create_project_()}/>
                </div>
            </div>
        </div>
    )
}