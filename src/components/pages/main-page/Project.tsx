import "./styles/project.css"


type Props ={
    project: IProject
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
            <div className={"main-page-project-right"}>
                <p className={"main-page-project-packages"}>{project.packages}</p>
                <p className={"main-page-project-tags"}>{project.meta.tags}</p>
            </div>
            <div className={""}></div>
        </div>
    )
}