import "./styles/create-project.css"
import Button from "../../common/Button.tsx";
import {close_project} from "../../../services/create-project.ts";
import ProjectTemplates from "./ProjectTemplates.tsx";
import ProjectMeta from "./ProjectMeta.tsx";
import ProjectPackages from "./ProjectPackages.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import FsAside from "../../aside-widgets/FsAside.tsx";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
// import {invoke} from "@tauri-apps/api/core";

export default function CreateProject() {

    const create_project = createProjectStore(state => state.create_project)
    const current_template = cacheStore(state => state.currentTemplate)
    const set_current_path = projectStore(state=>state.set_path_to_current_project);

    async function create_project_() {
        console.log(createProjectStore.getState().results)
        if (current_template) {
            let res = await create_project(current_template!);

            if (res[0] == 0) {
                set_current_path(res[1]);
                createProjectStore.getState().close();
                const buttons = res[2]!.workspace.buttons;
                let left_top = buttons.filter(el => el.pos == "LeftTop")
                let left_bot = buttons.filter(el => el.pos == "LeftBottom")
                let right_top = buttons.filter(el => el.pos == "RightTop")

                let left_top_2 =
                    left_top.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <FsAside></FsAside>,
                        icon: el.icon, keys: el.keys
                    }));

                let left_bot_2 =
                    left_bot.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <></>,
                        icon: el.icon, keys: el.keys
                    }));

                let right_top_2 =
                    right_top.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <></>,
                        icon: el.icon, keys: el.keys
                    }));
                asideButtonsStore.getState().load_left(left_top_2);
                asideButtonsStore.getState().load_bottom(left_bot_2);
                asideButtonsStore.getState().load_right(right_top_2);



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