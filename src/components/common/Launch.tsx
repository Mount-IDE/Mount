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
import pageStore from "../../stores/page_store.ts";
import Icon from "./Icon.tsx";

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
                pageStore.getState().setFilter(true)
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

    const refs = launchStore(state => state.references)


    const ref = useRef<HTMLDivElement>(null)

    const [clicked, setClicked] = useState(false)

    console.log(refs)
    return (
        <div id={"launch"}>

            <div id={"launch-list"}>

            </div>
            <div
                className={clicked ? "launch-current-click" : ""}
                style={{
                    opacity: currentLaunch ? "1" : "0.5",
                }}
                id={"launch-current"}
                onClick={() => setClicked(p => !p)}
            >
                <Icon src={launch} size={"18px"}/>
                <p id={"launch-current-p"}
                   className={"select-none"}>{currentLaunch ? currentLaunch.name : "Nothing"}</p>

                <Icon src={arrow} size={"13px"}/>
                {clicked && <div
                    onClick={(e) => e.stopPropagation()}
                    style={{
                        padding: "10px"
                    }}
                    className={"absolute min-w-full top-full flex flex-col min-h-10 max-h-50 overflow-auto bg-(--bg) border border-(--border3) overflow-x-hidden"}>
                    {refs.map((el, i) =>
                        <div className={"w-full h-5 flex items-center hover:bg-(--hover) text-(--subtitle)"} key={i}
                             onClick={() => launchStore.getState().set_current_launch(el)}
                        >
                            <p className={"whitespace-nowrap text-[8pt] font-[300]"}>{el.name}</p>
                        </div>
                    )}
                </div>}
            </div>
            <Icon src={play} in_size={"65%"} styles={"hover:bg-(--hover) rounded-[5px]"} disabled={!currentLaunch}
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
                  }}/>

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
