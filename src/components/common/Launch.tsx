import "./styles/launch.css"
import launch from "../../assets/launch-term.svg"
import play from "../../assets/play.svg"
import arrow from "../../assets/arrow.svg"
import more from "../../assets/more.svg"
import ContextMenu, {IContextMenuButton} from "./ContextMenu.tsx";
import {useRef, useState} from "react";
import {launchStore} from "../../stores/launch_store.ts";
import {projectStore} from "../../stores/project_store.ts";
import {asideButtonsStore} from "../../stores/aside_buttons_store.ts";
import {asideStore} from "../../stores/aside_store.ts";

export default function Launch() {


    const [showContext, setShowContext] = useState(false)
    const [cords, setCords] = useState<[number, number]>([0, 0])

    const currentLaunch = launchStore(state => state.current_launch)
    const project = projectStore(state => state.current_project)
    const runLaunch = launchStore(state => state.run_launch)
    const compileToObj = launchStore(state => state.compile_to_obj)

    const open_launch = launchStore(state => state.set_opened);
    const more_buttons: IContextMenuButton[] = [
        {
            cb: () => {
                open_launch(true)
            },
            hotkeys: "",
            title: "Edit"
        }, {
            cb: () => {
            },
            hotkeys: "",
            title: "Delete"
        },
    ]

    const ref = useRef<HTMLDivElement>(null)

    return (
        <div id={"launch"}>

            <div id={"launch-list"}>

            </div>
            <div
                style={{
                    opacity: currentLaunch ? "1" : "0.5"

                }}
                id={"launch-current"}>
                <div className={"launch-current-img"}>
                    <img src={launch}/>
                </div>
                <p id={"launch-current-p"}>{currentLaunch ? currentLaunch.name : "Nothing"}</p>
                <div className={"launch-current-img2"}>
                    <img src={arrow}/>
                </div>
            </div>
            <button
                disabled={!currentLaunch}
                id={"launch-launch"}
                onClick={async () => {
                    if (!currentLaunch || !project) return;

                    let object =
                        project.workspace.launch_objects.find((obj) => obj.launch_reference === currentLaunch.id) ?? null;

                    if (!object) {
                        const template = project.workspace.launch_templates.find((template) => template.id === currentLaunch.template[1]);
                        if (!template) return;
                        object = await compileToObj(currentLaunch, currentLaunch.results, template);
                    }

                    if (!object) return;
                    runLaunch(object);

                    openAsideLaunch();
                }}
            >
                <img src={play}/>
            </button>
            <button id={"launch-actions"}
                    onClick={(e) => {
                        setShowContext(prev => !prev)
                        setCords([e.clientX, e.clientY])
                    }}
            >
                <img src={more}/>
                <ContextMenu ref={ref} buttons={more_buttons} x={cords[0]} y={cords[1]} show={showContext}/>
            </button>
        </div>
    )
}

function openAsideLaunch() {
    const buttons = asideButtonsStore.getState();
    const aside = asideStore.getState();
    const button =
        buttons.left_buttons.find((button) => button.widget === "AsideLaunch") ??
        buttons.right_buttons.find((button) => button.widget === "AsideLaunch") ??
        buttons.bottom_buttons.find((button) => button.widget === "AsideLaunch");

    if (!button) return;

    if (buttons.left_buttons.some((item) => item.id === button.id && item.widget === button.widget)) {
        buttons.set_current_left_button(button);
        aside.toggle_left(() => true);
        return;
    }

    if (buttons.right_buttons.some((item) => item.id === button.id && item.widget === button.widget)) {
        buttons.set_current_right_button(button);
        aside.toggle_right(() => true);
        return;
    }

    buttons.set_current_bottom_button(button);
    aside.toggle_bottom(() => true);
}
