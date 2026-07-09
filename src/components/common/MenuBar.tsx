import "./styles/menu-bar.css"
import logo from "../../assets/icon.svg"
import {useEffect, useRef, useState} from "react";
import list from "../../assets/list.svg"
import MenuBarSection, {IMenuBarSection} from "./MenuBarSection.tsx";
import {cacheStore} from "../../stores/cache_store.ts";
import {projectStore} from "../../stores/project_store.ts";
import {menuBarStore} from "../../stores/menubar_store.ts";


interface IMenuButton {
    label: string,
    options: IMenuButtonOption[]
}

interface IMenuButtonOption {
    label: string,
    hotkeys?: string,
    cb: () => void
}


export default function MenuBar() {


    const buttons: IMenuButton[] = [
        {
            label: "File",
            options: [
                {
                    label: "New",
                    hotkeys: "sl;k",
                    cb: menuBarStore.getState().create_project
                }, {
                    label: "Open",
                    hotkeys: "",
                    cb: menuBarStore.getState().open_project
                }, {
                    label: "Open Recent",
                    hotkeys: "",
                    cb: menuBarStore.getState().open_recent_project
                }, {
                    label: "Close Project",
                    hotkeys: "",
                    cb: menuBarStore.getState().close_project
                },
                {
                    label: "Save Project",
                    hotkeys: "",
                    cb: menuBarStore.getState().save_all
                }, {
                    label: "Auto-save",
                    hotkeys: "",
                    cb: menuBarStore.getState().auto_save
                }, {
                    label: "Project Settings",
                    hotkeys: "",
                    cb: menuBarStore.getState().project_settings
                }, {
                    label: "Global Settings",
                    hotkeys: "",
                    cb: menuBarStore.getState().global_settings
                }, {
                    label: "Exit",
                    hotkeys: "",
                    cb: menuBarStore.getState().exit
                },
            ]
        }, {
            label: "Edit",
            options: [
                {
                    label: "Undo",
                    hotkeys: "",
                    cb: menuBarStore.getState().undo
                }, {
                    label: "Redo",
                    hotkeys: "",
                    cb: menuBarStore.getState().redo
                }, {
                    label: "Cut",
                    hotkeys: "",
                    cb: menuBarStore.getState().cut
                }, {
                    label: "Copy",
                    hotkeys: "",
                    cb: menuBarStore.getState().copy
                }, {
                    label: "Copy Path",
                    hotkeys: "",
                    cb: menuBarStore.getState().copy_path
                }, {
                    label: "Paste",
                    hotkeys: "",
                    cb: menuBarStore.getState().paste
                }, {
                    label: "Duplicate line",
                    hotkeys: "",
                    cb: menuBarStore.getState().duplicate_line
                },
            ]
        }, {
            label: "View",
            options: []
        }, {
            label: "Code",
            options: []
        }, {
            label: "Help",
            options: []
        }, {
            label: "Window",
            options: []
        },
    ]

    const [current, setCurrent] = useState(-1);

    const [showButtons, setShowButtons] = useState(false)

    const ref = useRef<HTMLDivElement>(null)

    useEffect(() => {
            if (!showButtons) {
                setCurrent(-1)
            }
            const cur = ref.current
            if (!cur) return
            if (showButtons) {
                cur.style.width = cur.scrollWidth + "px";
                cur.style.maxWidth = "100%"
                cur.style.overflow = "visible"
            } else {
                cur.style.width = "0";
                cur.style.maxWidth = "0";
                cur.style.overflow = "hidden"
            }
        },
        [showButtons])

    const recent_projects = cacheStore(state => state.recent_projects);
    const current_project = projectStore(state => state.current_project);

    const projects: IMenuBarSection = {
        label: `${current_project?.name ?? ""}`,
        buttons: [
            {
                cb: () => {
                },
                icon: "plus.svg",
                label: "New Project",
                searchable: false
            }, {
                cb: () => {
                },
                icon: "dir.svg",
                label: "Open",
                searchable: false
            },
        ],
        lists: [
            {
                label: "Open Projects",
                searchable: true,
                elems: [
                    {
                        label: `${current_project?.name ?? ""}`,
                        cb: () => {
                        }
                    }
                ],
            },
            {
                label: "Recent",
                searchable: true,
                elems:
                    recent_projects.map((el_) => ({
                        label: `${el_.name}`,
                        cb: () => {
                        }
                    }))

            }
        ]

    }


    return (
        <div id={"menu-bar"}>
            <div id={"menu-bar-logo"}>
                <img src={logo}/>
            </div>
            <div id={"menu-bar-buttons"}>
                <div id={"menu-bar-show-buttons"} onClick={() => setShowButtons(prev => !prev)}>
                    <img src={list}/>
                </div>
                <div ref={ref} id={"menu-bar-list"}>
                    {
                        buttons.map((el, i) =>
                            <MenuBarButton index={i} obj={el} key={i}
                                           cb={(i) => setCurrent(i)} current={i == current}
                            />
                        )
                    }
                </div>
            </div>
            <MenuBarSection obj={projects}/>
        </div>
    )
}


type Props = {
    obj: IMenuButton
    cb: (i: number) => void;
    index: number;
    current: boolean
}

function MenuBarButton(props: Props) {
    const {obj} = props;

    return (
        <div onMouseEnter={() => props.cb(props.index)} className={"menu-bar-button"}>
            <p className={"menu-bar-button-p"}>
                {obj.label}
            </p>
            {props.current && <div className={"menu-bar-menu"}>
                {obj.options.map((el, i) =>
                    <div className={"menu-bar-menu-option"} onClick={el.cb} key={i}>
                        <p className={"menu-bar-option-name"}>{el.label}</p>
                        <p className={"menu-bar-option-keys"}>{el.hotkeys}</p>
                    </div>
                )}
            </div>}
        </div>
    )
}