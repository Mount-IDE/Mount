import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";
import ProjectTemplates from "./ProjectTemplates.tsx";
import ProjectMeta from "./ProjectMeta.tsx";
import ProjectPackages from "./ProjectPackages.tsx";


export default function CreateProject() {




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
                    <Button title={"Create Project"} cb={() => {
                    }}/>
                </div>
            </div>
        </div>
    )
}