import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";
import React from "react";




export default function ProjectWorkSpace() {


    const left_top_buttons: IAsideButton[] = [
        {
            id: 0,
            alt: "Project",
            component:()=><></>,
            icon: ["main", "dir.svg"],
            keys: ""

        }, {
            id: 1,
            alt: "Commit",
            component:()=><></>,
            icon: ["main", "commit.svg"],
            keys: ""

        },
    ]
    const left_bot_buttons: IAsideButton[] = [
        {
            id: 0,
            alt: "Debug",
            component:()=><></>,
            icon: ["main", "debug.svg"],
            keys: ""

        }, {
            id: 1,
            alt: "Launch",
            component:()=><></>,
            icon: ["main", "play.svg"],
            keys: ""

        },{
            id: 2,
            alt: "Terminal",
            component:()=><></>,
            icon: ["main", "terminal.svg"],
            keys: ""

        },{
            id: 3,
            alt: "Problems",
            component:()=><></>,
            icon: ["main", "problems.svg"],
            keys: ""

        },{
            id: 4,
            alt: "Git",
            component:()=><></>,
            icon: ["main", "git.svg"],
            keys: ""

        },
    ]
    const right_top_buttons: IAsideButton[] = [
        {
            id: 0,
            alt: "Settings",
            component:()=><></>,
            icon: ["main", "dir.svg"],
            keys: ""

        }, {
            id: 1,
            alt: "Database",
            component:()=><></>,
            icon: ["main", "commit.svg"],
            keys: ""

        }, {
            id: 1,
            alt: "Commit",
            component:()=><></>,
            icon: ["main", "commit.svg"],
            keys: ""

        },
    ]



    return (
        <div id={"project-workspace"}>
            <MiniAside is_left/>
            <Aside/>
            <Central/>
            <Aside/>
            <MiniAside/>
        </div>
    )
}