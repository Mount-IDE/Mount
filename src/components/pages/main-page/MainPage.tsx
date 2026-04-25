import "./styles/main-page.css"
import Button from "../../common/Button.tsx";
import Filters from "./Filters.tsx";
import logo from "../../../assets/logo.svg"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Project from "./Project.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import pageStore from "../../../stores/page_store.ts";
import {open_project} from "../../../services/create-project.ts";

export default function MainPage() {

    // const openCreateProject=  createProjectStore(state=>state.open);
    // const setBlur = pageStore(state=>state.setFilter);
    const buttons = [
        {
            title: "New Project",
            cb: () => {
                open_project();
            },
            is_main:true
        }, {
            title: "Open Project",
            cb: () => {}
        }, {
            title: "Import from VCS",
            cb: () => {}
        }, {
            title: "Connect to",
            cb: () => {}
        }
    ]
    const [recent, setRecent] = useState<IProject[]>([]);

    async function loadRecents(){
        try {
            let recent = await invoke<IRecentProject[]>("get_recent_projects");
            let res = await invoke<IProject[]>("read_recent_projects", {
                recent: recent
            });
            console.log("recent loaded")
            setRecent(res);
        } catch (e){
            console.log("not loaded", e)
        }
    }
    useEffect(()=>{
      loadRecents().then()
    },[])

    return (
        <div className={"page"} id={"main-page"}>
            <div id={"main-page-left"}>
                <div id={"main-page-logo"}>
                    <div id={"main-page-logo-logo"}>
                        <img src={logo}/>
                    </div>
                    <p>Welcome to<br/>Mount!</p>
                </div>
                <div id={"main-page-left-buttons"}>
                    {buttons.map((el, i)=>
                    <Button {...el} key={i}/>
                    )}
                </div>
            </div>
            <div id={"main-page-right"}>
                <div id={"main-page-right-dec"}>
                    <Filters/>
                    <div id={"main-page-groups"}>

                    </div>
                    <div id={"main-page-projects"}>
                        {recent.length>0&&
                            recent.map((el, i)=>
                                <Project project={el} key={i} />
                            )
                        }
                        {recent.length==0 && <p
                            style={{
                                color: "var(--subtitle)",
                                width: "100%",
                                height: "50%",
                                display:"flex",
                                flexDirection:"column",
                                alignItems:"center",
                                justifyContent:"center"
                            }}
                        >Not any recent projects</p>}
                    </div>
                </div>
            </div>
        </div>
    )
}