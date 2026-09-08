import "./styles/project-settings-packages.css"
import {Fragment} from "react";
import {projectStore} from "../../../stores/project_store.ts";
import ProjectSettingsPackage from "./ProjectSettingsPackage.tsx";


export default function ProjectSettingsPackages() {

    const packages = projectStore(state => state.selected_packages)

    let packs = [...packages.values()].map(el => el.main)


    return (
        <div id={"project-settings-packages"}>
            {
                packs.map(el =>
                    <Fragment key={el.id}>
                        <ProjectSettingsPackage pack={el}/>
                        <hr style={{
                            border: "1px solid var(--border2)",
                            marginBottom: "20px",
                            width: "90%",
                            marginLeft: "5%"
                        }}/>
                    </Fragment>
                )
            }
        </div>
    )
}