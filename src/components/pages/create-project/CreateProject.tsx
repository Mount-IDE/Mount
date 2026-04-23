import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";


export default function CreateProject() {




    return (
        <div id={"create-project"}>
            <div id={"create-project-top"}></div>
            <div id={"create-project-main"}>
                <div id={"create-project-left"}>
                    <p id={"create-project-templates-p"}>Templates</p>
                    <div id={"create-project-templates"}>

                    </div>
                    <button id={"create-project-templates-manage"}></button>
                </div>
                <hr/>
                <div id={"create-project-meta"}>
                </div>
                <div id={"create-project-packages"}>
                    <p id={"create-project-packages-p"}>Packages</p>
                    <div id={"create-project-packages-list"}>
                        <div id={"create-project-packages-list-in"}></div>
                    </div>
                    <button id={"create-project-packages-manage"}></button>
                </div>
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