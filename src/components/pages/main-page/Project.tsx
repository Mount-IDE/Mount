import "./styles/project.css"
import more from "../../../assets/more.svg"

type Props ={
    project: IRecentProject
}

export default function Project(props: Props) {
    const {project} = props;

    return (
        <div className={"main-page-project"}>
            <div className={"main-page-project-logo"}></div>
            <div className={"main-page-project-left"}>
                <p className={"main-page-project-name"}>{project.name}</p>
                <p className={"main-page-project-path"}>{project.path}</p>
            </div>
            <div className={"main-page-project-right-"}>
                <p className={"main-page-project-packages"}>{
                    project.packages.length>0?project.packages.slice(3).join(", "): "no projects"
                }</p>
                <p className={"main-page-project-tags"}>{
                    project.meta.tags.length>0?project.meta.tags.slice(5).join(" "):"no tags"
                }</p>
            </div>
            <div className={"main-page-project-button"}>
                <img src={more}/>
            </div>
        </div>
    )
}