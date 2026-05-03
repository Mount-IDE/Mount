import "./styles/main-page.css"
import Button from "../../common/Button.tsx";
import Filters from "./Filters.tsx";
import logo from "../../../assets/logo.svg"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Project from "./Project.tsx";
import {open_project} from "../../../services/create-project.ts";
import {mainPageStore} from "../../../stores/main_page_store.ts";



export default function MainPage() {

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
    const [recent, setRecent] = useState<IRecentProject[]>([]);

    const groups = mainPageStore(state=>state.groups);
    
    async function loadRecents(){
        try {
            let recent = await invoke<IRecentProject[]>("get_recent_projects");
            let res = recent.sort((a,b)=>b.last_opened-a.last_opened)
            setRecent(res);
        } catch (e){
            console.warn("not loaded", e)
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
                        {groups.map((el)=>
                            <button key={el.id} className={"main-page-group"}>{el.name}</button>
                        )}
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