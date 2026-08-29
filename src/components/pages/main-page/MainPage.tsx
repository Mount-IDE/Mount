import "./styles/main-page.css"
import Button from "../../common/Button.tsx";
import Filters from "./Filters.tsx";
import logo from "../../../assets/icon.svg"
import React, {useEffect, useMemo, useRef, useState} from "react";
import {invoke} from "@tauri-apps/api/core";
import Project from "./Project.tsx";
import {open_project} from "../../../services/create-project.ts";
import {mainPageStore} from "../../../stores/main_page_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {cacheStore} from "../../../stores/cache_store.ts";
import ContextMenu, {IContextMenuButton} from "../../common/ContextMenu.tsx";
import Modal from "../../common/Modal.tsx";
import {menuStore} from "../../../stores/menu_store.ts";
import {computeBP, themeStore} from "../../../stores/theme_store.ts";
import {noteStore, NotificationType} from "../../../stores/note_store.ts";


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


/**
 * A component of main page that contains list of created projects and buttons for itself creation
 * @constructor
 */
export default function MainPage() {


    // const [recent, setRecent] = useState<IRecentProject[]>([]);

    const recents = cacheStore(state => state.recent_projects);

    const groups = mainPageStore(state => state.groups);
    const set_current_group = mainPageStore(state => state.set_current_group)
    const current_group = mainPageStore(state => state.current_group);
    const filter_string = mainPageStore(state => state.filter_string);

    const [recent, setRecent] = useState<IRecentProject[]>([])

    useEffect(() => {

        let res = filter_string.length > 0 ? recents.filter(el =>
                el.name.includes(filter_string)
            )
            : recents
        setRecent(res)

    }, [recents, filter_string]);

    /**
     * Open project when user selects a project from list
     * @param current_path path to selected project
     */
    async function setup_project(current_path: string) {
        if (current_path.length > 0) {
            try {
                let res = await invoke<IProject>("read_project", {
                    path: current_path
                }) as unknown
                if (!res) {
                    noteStore.getState().add_note({
                        type: NotificationType.WARN,
                        text: "Project not found"
                    })
                    return
                }
                await projectStore.getState().open_project(res as IProject)
            } catch (e) {
                console.error(e)
            }
        } else {
            console.warn("path is empty")
        }
    }


    const recentByGroup = recent.filter(el =>
        el.meta.group == (groups.find(el => el.id == current_group)?.name ?? ""))


    const [currentPath, setCurrentPath] = useState("");

    const [showContext, setShowContext] = useState(false)
    const context_buttons: IContextMenuButton[] = useMemo(() => ([
        {
            cb: async () => {
            },
            hotkeys: "",
            title: "Open",
            icon: "dir.svg"

        }, {
            cb: async () => {
            },
            hotkeys: "",
            title: "Edit",
            icon: "edit.svg"

        }, {
            cb: async () => {

                openModal({
                    buttons: [{
                        cb: async (_: string | undefined) => {
                            try {
                                await invoke("remove_project", {path: currentPath});
                                cacheStore.getState().remove_from_recents(currentPath);
                                menuStore.getState().close_modal()
                            } catch (e) {
                                console.error(e)
                            }
                        },
                        title: "Remove",
                        typ: "cancel"

                    }],
                    title: `Delete project at ${currentPath}?`,
                    typ: "confirm"

                })

            },
            hotkeys: "",
            title: "Delete",
            icon: "remove.svg"

        }
    ]), [currentPath])
    const [cords, setCords] = useState<[number, number]>([0, 0]);
    const ref = useRef<HTMLDivElement>(null)

    const modal = menuStore(state => state.modal);
    const openModal = menuStore(state => state.open_modal);
    const modalSettings = menuStore(state => state.modal_settings);


    function show_context(e: React.MouseEvent, path: string) {
        if (showContext) {
            setShowContext(false);
            return;
        }
        setCords([e.clientX - 150, e.clientY + 20]);
        setShowContext(true);
        setCurrentPath(path);

    }


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

    const theme = themeStore(state => state.current_theme?.elements?.mainpage)

    return (
        <div className={"page"} id={"main-page"}>
            <div
                style={{
                    background: theme?.left?.background,
                    borderRadius: theme?.left?.rounded,
                    ...computeBP(theme?.left?.border, "border"),
                    ...computeBP(theme?.left?.padding, "padding"),
                }}
                id={"main-page-left"}>
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
            <div id={"main-page-right"}
                 style={{
                     background: theme?.right?.background,
                     borderRadius: theme?.right?.rounded,
                     ...computeBP(theme?.right?.border, "border"),
                     ...computeBP(theme?.right?.padding, "padding"),
                 }}
            >
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
                        {recentByGroup.length > 0 &&
                            recentByGroup.map((el, i) =>
                                <Project onContext={show_context} onClick={setup_project} project={el} key={i}/>
                            )
                        }
                        {recentByGroup.length == 0 && <p
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
                        {
                            modal
                            &&
                            <Modal {...modalSettings!} />
                        }
                        <ContextMenu auto={true} ref={ref} buttons={context_buttons} x={cords[0]} y={cords[1]}
                                     show={showContext}/>
                    </div>
                </div>
            </div>
        </div>
    )
}
