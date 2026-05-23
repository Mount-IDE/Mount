import "./styles/project-space.css"
import ProjectWorkSpace from "./ProjectWorkSpace.tsx";
import Footer from "./Footer.tsx";
import CreateEntity from "./CreateEntity.tsx";
import {menuStore} from "../../../stores/menu_store.ts";


export default function ProjectSpace() {
    const show_file_creation_menu = menuStore(state => state.file_create_menu);
    return (
        <div id={"project-space"}>
            {
                show_file_creation_menu &&
                <CreateEntity/>
            }
            <ProjectWorkSpace/>
            <Footer/>
        </div>
    )
}