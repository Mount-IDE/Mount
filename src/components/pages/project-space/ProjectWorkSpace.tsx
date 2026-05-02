import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";
import React from "react";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";




export default function ProjectWorkSpace() {

    const left_top = asideButtonsStore(state=>state.left_top_buttons);
    const left_bot = asideButtonsStore(state=>state.left_bottom_buttons);
    const right_top = asideButtonsStore(state=>state.right_top_buttons);

    return (
        <div id={"project-workspace"}>
            <MiniAside top={left_top} bottom={left_bot} max_top={3}/>
            <Aside/>
            <Central/>
            <Aside/>
            <MiniAside top={right_top} max_top={3}/>
        </div>
    )
}