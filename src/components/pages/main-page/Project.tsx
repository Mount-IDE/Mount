import "./styles/project.css"
import more from "../../../assets/more.svg"
import {cacheStore} from "../../../stores/cache_store.ts";
import {computeBP, themeStore} from "../../../stores/theme_store.ts";
import React from "react";

type Props = {
    project: IRecentProject
    onClick: (path: string) => void
    onContext: (e: React.MouseEvent, path: string) => void
}

export default function Project(props: Props) {
    const {project} = props;

    async function loadProject() {
        try {
            let path = cacheStore.getState().make_path([project.path, project.name])
            props.onClick(path);
        } catch (e) {
            console.error(e)
        }
    }


    const theme = themeStore(state => state.current_theme?.elements?.mainpage?.project)


    const color = (() => {
        if (project.meta.icon?.includes("#")) {
            let col = project.meta.icon!.replace("#", "")
            let r = parseInt(col.substring(0, 2), 16);
            let g = parseInt(col.substring(2, 4), 16);
            let b = parseInt(col.substring(4, 6), 16);
            const brightness =
                (r * 299 + g * 587 + b * 114) / 1000;
            return brightness > 128 ? "#333" : "#fff"
        }
        return ""
    })()

    let date = new Date(project.last_opened);

    return (
        <div className={"main-page-project"} onClick={loadProject}

             style={{
                 background: theme?.this?.background,
                 ...computeBP(theme?.this?.padding, "padding"),
                 borderRadius: theme?.this?.rounded
             }}
        >
            <div className={"main-page-project-logo"}
                 style={{
                     borderRadius: theme?.icon?.rounded,
                     ...computeBP(theme?.icon?.border, "border")
                 }}
            >
                {
                    !project.meta.icon &&
                    <div style={{
                        width: "100%",
                        height: "100%",
                        background: project.meta.icon,
                        font: "20pt 'Jetbrains Mono Medium'",
                        display: "flex",
                        flexDirection: "column",
                        alignItems: "center",
                        justifyContent: "center",
                        color: color,
                    }}>{project.name[0].toUpperCase()}</div>
                }

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
                        color: color,
                        // mixBlendMode: "difference"
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
                <p className={"main-page-project-name"}
                   style={{
                       color: theme?.name?.color,
                       textDecoration: theme?.name?.underscore
                   }}
                >{project.name}</p>
                <p className={"main-page-project-path"}
                   style={{
                       color: theme?.path?.color,
                       textDecoration: theme?.path?.underscore
                   }}
                >{project.path}</p>
            </div>
            <div className={"main-page-project-center"}>
                <p className={"main-page-project-p p"}>
                    {project.meta.authors.join(" ")}
                </p>
                <div className={"main-page-project-center-2"}>
                    <p className={"main-page-project-p"}>{date.getFullYear()}.{date.getMonth() + 1}.{date.getDate()} at {date.getHours()}:{date.getMinutes()}</p>
                    <p className={"main-page-project-p"}>{project.meta.license}</p>
                </div>
            </div>
            <div className={"main-page-project-right"}>
                <p className={"main-page-project-packages"}
                   style={{
                       color: theme?.path?.color,
                       textDecoration: theme?.path?.underscore
                   }}
                >{
                    project.packages.length > 0 ? project.packages.slice(0, 3).join(", ") : "no packages"
                }</p>
                <p className={"main-page-project-tags"}
                   style={{
                       color: theme?.path?.color,
                       textDecoration: theme?.path?.underscore
                   }}
                >{
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