import "./styles/project-workspace.css"
import MiniAside from "./MiniAside.tsx";
import Aside from "./Aside.tsx";
import Central from "./Central.tsx";
import React from "react";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {asideStore} from "../../../stores/aside_store.ts";


export default function ProjectWorkSpace() {

    const left_top = asideButtonsStore(state => state.left_top_buttons);
    const left_bot = asideButtonsStore(state => state.left_bottom_buttons);
    const right_top = asideButtonsStore(state => state.right_top_buttons);


    const set_current_left = asideStore(state => state.set_current_left);
    const set_current_right = asideStore(state => state.set_current_right);
    const set_current_bot = asideStore(state => state.set_current_bottom);




    return (
        <div id={"project-workspace"}>
            <MiniAside is_left={true} set_top={set_current_left} set_bot={set_current_bot}
                        top={left_top} bottom={left_bot} max_top={3} max_bot={null}/>
            <Aside left={true} />
            <Central/>
            <Aside left={false} />
            <MiniAside is_left={false} set_top={set_current_right} set_bot={set_current_bot} top={right_top}
                       max_top={3} max_bot={null}/>
        </div>
    )
}