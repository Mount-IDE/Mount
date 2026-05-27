import "./styles/main-page.css"
import Button from "../../common/Button.tsx";
import Filters from "./Filters.tsx";
import logo from "../../../assets/logo.svg"
import {useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Project from "./Project.tsx";
import {open_project} from "../../../services/create-project.ts";
import {mainPageStore} from "../../../stores/main_page_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import pageStore from "../../../stores/page_store.ts";
import FsAside from "../../aside-widgets/FsAside.tsx";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";


/**
 * A component of main page that contains list of created projects and buttons for itself creation
 * @constructor
 */
export default function MainPage() {

    /**
     * buttons for creating project
     * @var buttons
     */
    const buttons = [
        {
            title: "New Project",
            cb: () => {
                open_project();
            },
            is_main: true
        }, {
            title: "Open Project",
            cb: () => {
            }
        }, {
            title: "Import from VCS",
            cb: () => {
            }
        }, {
            title: "Connect to",
            cb: () => {
            }
        }
    ]
    const [recent, setRecent] = useState<IRecentProject[]>([]);

    const groups = mainPageStore(state => state.groups);

    const set_current_project = () => projectStore.getState().set_current_project
    const openProject = () => pageStore.getState().openProject

    /**
     * load recent projects
     */
    async function loadRecents() {
        try {
            let recent = await invoke<IRecentProject[]>("get_recent_projects");
            let res = recent.sort((a, b) => b.last_opened - a.last_opened)
            setRecent(res);
        } catch (e) {
            console.warn("not loaded", e)
        }
    }

    useEffect(() => {
        loadRecents().then()
    }, [])


    /**
     * Open project when user selects a project from list
     * @param current_path path to selected project
     */
    async function setup_project(current_path: string) {
        if (current_path.length > 0) {
            try {
                let res = await invoke<IProject>("read_project", {
                    path: current_path
                })
                const set = set_current_project();
                set(res);
                const open = openProject()
                open();
                const buttons = res.workspace.buttons;
                let left_top = buttons.filter(el => el.pos == "LeftTop")
                let left_bot = buttons.filter(el => el.pos == "LeftBottom")
                let right_top = buttons.filter(el => el.pos == "RightTop")

                let left_top_2 =
                    left_top.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <FsAside></FsAside>,
                        icon: el.icon, keys: el.keys
                    }));

                let left_bot_2 =
                    left_bot.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <></>,
                        icon: el.icon, keys: el.keys
                    }));

                let right_top_2 =
                    right_top.map<IAsideButton>(el => ({
                        id: el.order,
                        alt: el.alt,
                        component: () => <></>,
                        icon: el.icon, keys: el.keys
                    }));
                asideButtonsStore.getState().load_left(left_top_2);
                asideButtonsStore.getState().load_bottom(left_bot_2);
                asideButtonsStore.getState().load_right(right_top_2);


                projectStore.getState().set_path_to_current_project(current_path);
            } catch (e) {
                console.error(e)
            }
        } else {
            console.warn("path is empty")
        }
    }

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
                    {buttons.map((el, i) =>
                        <Button {...el} key={i}/>
                    )}
                </div>
            </div>
            <div id={"main-page-right"}>
                <div id={"main-page-right-dec"}>
                    <Filters/>
                    <div id={"main-page-groups"}>
                        {groups.map((el) =>
                            <button key={el.id} className={"main-page-group"}>{el.name}</button>
                        )}
                    </div>
                    <div id={"main-page-projects"}>
                        {recent.length > 0 &&
                            recent.map((el, i) =>
                                <Project onClick={setup_project} project={el} key={i}/>
                            )
                        }
                        {recent.length == 0 && <p
                            style={{
                                color: "var(--subtitle)",
                                width: "100%",
                                height: "50%",
                                display: "flex",
                                flexDirection: "column",
                                alignItems: "center",
                                justifyContent: "center"
                            }}
                        >Not any recent projects</p>}
                    </div>
                </div>
            </div>
        </div>
    )
}