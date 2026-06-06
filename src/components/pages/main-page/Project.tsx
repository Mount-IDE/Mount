import "./styles/project.css"
import more from "../../../assets/more.svg"
import {invoke} from "@tauri-apps/api/core";
import {cacheStore} from "../../../stores/cache_store.ts";

type Props = {
    project: IRecentProject
    onClick: (path: string) => void
    onContext: (e: React.MouseEvent, path: string) => void
}

export default function Project(props: Props) {
    const {project} = props;

    async function loadProject(e: React.MouseEvent) {
        try {
            let path = await invoke<string>("make_path_command", {
                components: [project.path, project.name]
            })
            props.onClick(path);
        } catch (e) {
            console.error(e)
        }
    }

    return (
        <div className={"main-page-project"} onClick={loadProject}>
            <div className={"main-page-project-logo"}></div>
            <div className={"main-page-project-left"}>
                <p className={"main-page-project-name"}>{project.name}</p>
                <p className={"main-page-project-path"}>{project.path}</p>
            </div>
            <div className={"main-page-project-right-"}>
                <p className={"main-page-project-packages"}>{
                    project.packages.length > 0 ? project.packages.slice(3).join(", ") : "no projects"
                }</p>
                <p className={"main-page-project-tags"}>{
                    project.meta.tags.length > 0 ? project.meta.tags.slice(5).join(" ") : "no tags"
                }</p>
            </div>
            <div className={"main-page-project-button"} onClick={(e) => {
                e.stopPropagation()
                const path = cacheStore.getState().make_path([props.project.path, props.project.name]);
                props.onContext(e, path);
            }}>
                <img src={more}/>
            </div>
        </div>
    )
}