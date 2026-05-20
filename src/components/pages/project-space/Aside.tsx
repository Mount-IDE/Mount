import "./styles/aside.css"
import React, {useEffect, useRef} from "react";
import Header from "../../aside-widgets/Header.tsx";
import {asideButtonsStore} from "../../../stores/aside_buttons_store.ts";


type Props = {
    left?: boolean
    state: boolean
}

function Aside(props: Props) {

    const ref = useRef<HTMLDivElement>(null)
    const ref_ = useRef<HTMLHRElement>(null)
    const base_pos = useRef(0);
    const base_width = useRef(10)
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
            base_pos.current = e.clientX;
            base_width.current = current.getBoundingClientRect().width;
            document.body.style.userSelect = "none";
            // current.style.transition = "none";
        }

        function move(e: MouseEvent) {
            if (!is_moving.current) return

            if (frame!=null)return

            frame = requestAnimationFrame(() => {
                frame=null;
                const delta = e.clientX - base_pos.current;
                const next = !props.left ?
                    base_width.current - delta :
                    base_width.current + delta
                const MIN = 50;
                const MAX = window.innerWidth * 0.8;

                const clamped = Math.max(MIN, Math.min(MAX, next));
                current.style.width = `${clamped}px`
                console.log(clamped, current.style.width)
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

    }, [ref_]);


    const current_top = props.left ?
        asideButtonsStore(state => state.current_left) :
        asideButtonsStore(state => state.current_right)

    return (
        <>
            {
                !props.left &&
                <hr ref={ref_} className={"project-hr"}/>
            }
            <div
                style={{
                    width: base_width.current
                }}
                ref={ref} className={props.state ? "project-aside-selector project-aside" : "project-aside-selector project-aside-dis"}>
                    <Header label={current_top?.alt ?? ""}/>
                    <div className={"project-aside-body"}>
                        {current_top?.component()}
                    </div>
            </div>
            {props.left &&
                <hr ref={ref_} className={"project-hr"}/>}
        </>

    )
}
export default React.memo(Aside)