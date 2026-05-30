import "./styles/main-page.css"
import Button from "../../common/Button.tsx";
import Filters from "./Filters.tsx";
import logo from "../../../assets/icon.svg"
import {useEffect, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Project from "./Project.tsx";
import {open_project} from "../../../services/create-project.ts";
import {mainPageStore} from "../../../stores/main_page_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import pageStore from "../../../stores/page_store.ts";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {mapProjectButton} from "../../../utils/project-buttons.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";
import ContextMenu, {IContextMenuButton} from "../../common/ContextMenu.tsx";

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
    const set_current_group = mainPageStore(state => state.set_current_group)
    const current_group = mainPageStore(state => state.current_group);
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
            cacheStore.getState().set_recent_projects(res);
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
                    left_top.map<IAsideButton>(mapProjectButton);

                let left_bot_2 =
                    left_bot.map<IAsideButton>(mapProjectButton);

                let right_top_2 =
                    right_top.map<IAsideButton>(mapProjectButton);
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


    const recent_by_group = recent.filter(el =>
        el.meta.group == (groups.find(el => el.id == current_group)?.name ?? ""))

    const [currentPath, setCurrentPath] = useState("");

    const [showContext, setShowContext] = useState(false)
    const context_buttons: IContextMenuButton[] = [
        {
            cb: async () => {
                try {
                    await invoke("remove_project", {path: currentPath});
                } catch (e) {
                    console.error(e)
                }
            },
            hotkeys: "",
            title: "Delete"

        }
    ]
    const [cords, setCords] = useState<[number, number]>([0, 0]);

    function show_context(e: React.MouseEvent, path: string) {
        setCords([e.clientX, e.clientY]);
        setShowContext(true);
        setCurrentPath(path);

    }

    const ref = useRef<HTMLDivElement>(null)

    useEffect(() => {
        function handler(e: MouseEvent) {
            const tg = ref.current;
            if (!tg) return;
            const elem = e.target as Element
            if (!(tg.contains(elem) || tg == elem)) {
                setShowContext(false)
            }

        }

        window.addEventListener("click", handler)
        return () => {
            window.removeEventListener("click", handler)
        }
    }, [cords]);

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
                        <div id={"main-page-groups-list"}>
                            {groups.map((el) =>
                                <div key={el.id} className={"main-page-group"}
                                     onClick={(_) => set_current_group(el.id)}
                                     style={{
                                         color: current_group == el.id ? "var(--title)" : "var(--subtitle)"
                                     }}>
                                    <p style={{userSelect: "none"}}>{el.name}</p>
                                </div>
                            )}
                        </div>
                    </div>
                    <div id={"main-page-projects"}>
                        {recent.length > 0 &&
                            recent_by_group.map((el, i) =>
                                <Project onContext={show_context} onClick={setup_project} project={el} key={i}/>
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
                        <ContextMenu ref={ref} buttons={context_buttons} x={cords[0]} y={cords[1]} show={showContext}/>
                    </div>
                </div>
            </div>
        </div>
    )
}
