import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";
import ProjectTemplates from "./ProjectTemplates.tsx";
import ProjectMeta from "./ProjectMeta.tsx";
import ProjectPackages from "./ProjectPackages.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";
import pageStore from "../../../stores/page_store.ts";

export default function CreateProject() {

    const create_project = createProjectStore(state => state.create_project)
    const current_template = cacheStore(state => state.currentTemplate)
    const open_project = pageStore(state=>state.openProject)
    function create_project_() {
        if (current_template) {
            create_project(current_template!, open_project).catch(console.error).then()
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
                    <Button title={"Create Project"} cb={create_project_}/>
                </div>
            </div>
        </div>
    )
}