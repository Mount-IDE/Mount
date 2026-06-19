import "./styles/launch.css"
import launch from "../../assets/launch-term.svg"
import play from "../../assets/play.svg"
import arrow from "../../assets/arrow.svg"
import more from "../../assets/more.svg"
import ContextMenu, {IContextMenuButton} from "./ContextMenu.tsx";
import {useRef, useState} from "react";
import {launchStore} from "../../stores/launch_store.ts";

export default function Launch() {


    const [showContext, setShowContext] = useState(false)
    const [cords, setCords] = useState<[number, number]>([0, 0])

    const currentLaunch = launchStore(state => state.current_launch)

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
                id={"launch-launch"}>
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