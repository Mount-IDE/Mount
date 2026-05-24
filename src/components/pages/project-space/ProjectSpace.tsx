import "./styles/project-space.css"
import ProjectWorkSpace from "./ProjectWorkSpace.tsx";
import Footer from "./Footer.tsx";
import CreateEntity from "./CreateEntity.tsx";
import {menuStore} from "../../../stores/menu_store.ts";
import Modal from "../../common/Modal.tsx";


export default function ProjectSpace() {
    const show_file_creation_menu = menuStore(state => state.file_create_menu);
    const show_modal = menuStore(state=>state.modal);
    const modal_settings = menuStore(state=>state.modal_settings);
    return (
        <div id={"project-space"}>
            {
                show_modal && modal_settings!=null &&
                <Modal {...modal_settings}/>

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