import "./styles/project-space.css"
import ProjectWorkSpace from "./ProjectWorkSpace.tsx";
import Footer from "./Footer.tsx";
import CreateEntity from "./CreateEntity.tsx";
import {menuStore} from "../../../stores/menu_store.ts";
import Modal from "../../common/Modal.tsx";
import {useEffect} from "react";
import {projectSettingsStore} from "../../../stores/project_settings_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {launchStore} from "../../../stores/launch_store.ts";
import LaunchPage from "./LaunchPage.tsx";


export default function ProjectSpace() {
    const show_file_creation_menu = menuStore(state => state.file_create_menu);
    const show_modal = menuStore(state=>state.modal);
    const modal_settings = menuStore(state=>state.modal_settings);
    const project = projectStore(state => state.current_project)
    const set_project = projectSettingsStore(state => state.set_project)
    const launch_opened = launchStore(state => state.opened)

    useEffect(() => {
        set_project(project)
    }, [project])
    return (
        <div id={"project-space"}>
            {
                show_modal && modal_settings!=null &&
                <Modal {...modal_settings}/>

            }
            {
                launch_opened &&
                <LaunchPage/>
            }
            {
                show_file_creation_menu &&
                <CreateEntity/>
            }
            <ProjectWorkSpace/>
            <Footer/>
        </div>
    )
}