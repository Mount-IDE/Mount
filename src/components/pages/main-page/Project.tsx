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
            <div className={"main-page-project-logo"}>
                {
                    project.meta.icon && project.meta.icon?.includes("#") &&
                    <div style={{
                        width: "100%",
                        height: "100%",
                        background: project.meta.icon,
                        font: "20pt 'Jetbrains Mono Medium'",
                        display: "flex",
                        flexDirection: "column",
                        alignItems: "center",
                        justifyContent: "center",
                        color: "var(--title)",
                    }}>{project.name[0].toUpperCase()}</div>
                }
                {
                    project.meta.icon && !project.meta.icon.includes("#") &&
                    <div style={{
                        width: "100%",
                        height: "100%",
                        display: "flex",
                        flexDirection: "column",
                        alignItems: "center",
                        justifyContent: "center",
                    }}>
                        <img
                            style={{
                                display: "block",
                                objectFit: "contain",
                            }}
                            src={project.meta.icon}/>
                    </div>
                }
            </div>
            <div className={"main-page-project-left"}>
                <p className={"main-page-project-name"}>{project.name}</p>
                <p className={"main-page-project-path"}>{project.path}</p>
            </div>
            <div className={"main-page-project-right-"}>
                <p className={"main-page-project-packages"}>{
                    project.packages.length > 0 ? project.packages.slice(0, 3).join(", ") : "no packages"
                }</p>
                <p className={"main-page-project-tags"}>{
                    project.meta.tags.length > 0 ? project.meta.tags.slice(0, 5).join(" ") : "no tags"
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