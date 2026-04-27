import "./styles/section.css"
import React, {useEffect, useLayoutEffect, useRef, useState} from "react";
import Parameter from "./Parameter.tsx";
import {createProjectStore} from "../../../stores/create_project.ts";
import {cacheStore} from "../../../stores/cache_store.ts";

type Props = {
    section: ISection
}


export default function Section(props: Props) {

    const collapsible = props.section.list[0];
    const defaultOpened = !collapsible ? true : props.section.list[1];
    const [isOpened, setIsOpened] = useState<boolean>(defaultOpened);
    const current_template = cacheStore(state=>state.currentTemplate!);
    const set_value = createProjectStore(state => state.add_result);


    const body_ref = useRef<HTMLDivElement>(null)

    function toggle(_: React.MouseEvent<HTMLDivElement>) {
        const body = body_ref.current!;
        if (!collapsible) return
        if (body.classList.contains("project-section-in-open")) {
            body.style.maxHeight = body.scrollHeight + "px";

            requestAnimationFrame(() => {
                body.style.maxHeight = "0px";
            });

            body.classList.remove("project-section-in-open");
            setIsOpened(false);
        } else {
            body.classList.add("project-section-in-open");
            body.style.maxHeight = body.scrollHeight + "px";
            setIsOpened(true);
        }

    }

    useEffect(() => {
        if (!collapsible){
            const body = body_ref.current!;
            body.style.maxHeight = body.scrollHeight + "px";
        }else {

        }
    }, [props.section]);

    useLayoutEffect(() => {
        const body = body_ref.current;
        if (!body) return;

        if (isOpened) {
            body.style.maxHeight = body.scrollHeight + "px";
        } else {
            body.style.maxHeight = "0px";
        }
    }, [isOpened]);

    useEffect(() => {
        if (!collapsible) {
            setIsOpened(true);
        }
    }, [collapsible]);


    return (
        <div className={"project-section"}>
            <div className={"project-section-head"}
                 onClick={(e) => {
                     if (collapsible) {
                         toggle(e)
                     }
                 }}
            >
                {collapsible && <div
                    style={{
                        transform: isOpened ? "rotate(0deg)" : "rotate(-90deg)"
                    }}
                    className={"project-section-head-arrow"}>
                    <svg width="19" height="10" viewBox="0 0 19 10" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M0.421718 0.412777C-0.140573 0.956418 -0.140573 1.83792 0.421718 2.38154L7.46557 9.18545C8.59035 10.2719 10.4129 10.2714 11.5371 9.18461L18.5782 2.37653C19.1406 1.8329 19.1406 0.951407 18.5782 0.407752C18.016 -0.135917 17.1043 -0.135917 16.542 0.407752L10.5155 6.23469C9.95328 6.77845 9.04159 6.77831 8.47934 6.23469L2.45792 0.412777C1.89564 -0.130892 0.983995 -0.130892 0.421718 0.412777Z"
                            fill="#A4A4A4"/>
                    </svg>

                </div>}
                <p
                    style={{
                        color: isOpened ? "var(--title)" : "var(--subtitle)"
                    }}
                    className={"project-section-head-p"}>{props.section.label}</p>
            </div>
            <div ref={body_ref}
                 className={"project-section-in"}

                 >
                {props.section.params.map(el =>
                    <Parameter
                        set={(val) => {
                            set_value(current_template.id, props.section.id, el.out, val);
                        }}
                        param={el}
                        key={`${props.section.id}${el.out}`}
                        section={props.section.id}
                        allParams={props.section.params}
                    />)}
            </div>
        </div>
    )
}