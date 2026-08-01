import "./styles/bottom.css"
import {asideStore} from "../../../stores/aside_store.ts";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";
import {useEffect, useMemo, useRef} from "react";


type Props = {
   // state: boolean
}


export default function Bottom(_: Props) {

    const state = asideStore(state=>state.bottom);

    const current = asideButtonsStore(state=>state.current_bottom)
    const bottomButtons = asideButtonsStore(state => state.bottom_buttons);
    const heavyButtons = useMemo(
        () => bottomButtons.filter(button => button.component_type === "Heavy"),
        [bottomButtons]
    );

    const ref = useRef<HTMLDivElement>(null)
    const ref_ = useRef<HTMLHRElement>(null)
    const base_pos = useRef(0);
    const base_height = useRef(10)
    const is_moving = useRef(false)

    useEffect(() => {
        let current_ = ref.current
        let hr_ = ref_.current
        if (!hr_) return
        if (!current_) return
        let current = current_!;
        let hr = hr_!;

        let frame: number | null=null;
        function start_move(e: MouseEvent) {
            is_moving.current = true;
            base_pos.current = e.clientY;
            base_height.current = current.getBoundingClientRect().height;
            document.body.style.userSelect = "none";
            // current.style.transition = "none";

        }

        function move(e: MouseEvent) {
            if (!is_moving.current) return

            if (frame!=null)return

            frame = requestAnimationFrame(() => {
                frame=null;
                const delta = e.clientY - base_pos.current;
                const next = base_height.current - delta;
                const MIN = 50;
                const MAX = window.innerHeight * 0.8;

                const clamped = Math.max(MIN, Math.min(MAX, next));
                current.style.height = `${clamped}px`
            })
        }


        function stop_move() {
            if (!is_moving.current) return
            is_moving.current = false
            document.body.style.userSelect = "";
            // current.style.transition = "all 0.2s";
        }

        hr.addEventListener("mousedown", start_move);
        window.addEventListener("mousemove", move);
        window.addEventListener("mouseup", stop_move);

        return () => {
            hr.removeEventListener("mousedown", start_move);
            window.removeEventListener("mousemove", move);
            window.removeEventListener("mouseup", stop_move);
        }

    }, [ref_, state]);

    /* const lightComponent =
         current?.component_type === "Light" && state ? current.component({active: true}) : null;
 */
    const Light = current?.component_type === "Light" && state ? current.component : null;

    return (
        <>
            <hr ref={ref_} style={{cursor: "s-resize"}} className={"project-hr"}/>
            <div
                style={{
                    // position: "absolute",
                    // bottom:0,
                }}
                ref={ref} id={"project-bottom"} className={state?"project-bottom":"project-bottom-dis"}>
                {/*<BottomHeader ref={head_ref} title={current?.alt??"Not Found"}/>*/}
                <div id={"project-bottom-in"}>
                    {heavyButtons.map(button => {
                        const active = state && current?.id === button.id;
                        const Component = button.component;
                        return (
                            <div
                                key={`${button.widget}-${button.id}`}
                                className={active ? "project-bottom-panel" : "project-bottom-panel project-bottom-panel-hidden"}
                            >
                                {
                                    //  button.component({active})
                                    Component && <Component active={active}/>
                                }
                            </div>
                        );
                    })}
                    {
                        Light &&
                        <Light active={true}/>
                    }
                </div>
            </div>
        </>
    )
}
