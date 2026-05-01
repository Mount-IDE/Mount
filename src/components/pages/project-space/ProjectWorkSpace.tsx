import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";




export default function ProjectWorkSpace() {
    return (
        <div id={"project-workspace"}>
            <MiniAside is_left/>
            <Aside/>
            <Central/>
            <Aside/>
            <MiniAside/>
        </div>
    )
}