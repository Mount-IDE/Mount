import {createProjectStore} from "../../../stores/create_project.ts";
import "./styles/section.css"
import React, {Fragment, useEffect, useLayoutEffect, useRef, useState} from "react";
import {packageStore} from "../../../stores/package_store.ts";
import PackageParameter from "./PackageParameter.tsx";

export default function PackageSection() {

    const set_value = createProjectStore.getState().add_pack_result;
    const [isOpened, setIsOpened] = useState<boolean>(true);

    const all_packages = packageStore(state => state.packages);
    const selected_packages = createProjectStore(state => state.packages);

    const body_ref = useRef<HTMLDivElement>(null)
    useLayoutEffect(() => {
        const body = body_ref.current;
        if (!body) return;

        if (!isOpened) {
            body.style.maxHeight = "0px";
            return;
        }

        body.style.maxHeight = `${body.scrollHeight}px`;
    }, [isOpened]);
    useEffect(() => {
        const body = body_ref.current;
        if (!body) return;

        const observer = new ResizeObserver(() => {
            if (isOpened) {
                body.style.maxHeight = `${body.scrollHeight}px`;
            }
        });

        observer.observe(body);

        return () => observer.disconnect();
    }, [isOpened]);


    const [packages, setPackages] = useState<IPackage[]>([])

    useEffect(() => {
        setPackages(_ => {
            let res = all_packages.filter(el => selected_packages.has(el.id))
            console.log(res)

            return res
        })
        console.log(selected_packages)
    }, [all_packages, selected_packages])


    useEffect(() => {
        console.log(packages.length)
    }, [packages]);

    function toggle() {
        setIsOpened(prev => !prev);
    }

    function write(val: string | boolean | string[], par: string, pack: string) {
        set_value(pack, par, val);
    }


    useLayoutEffect(() => {
        const body = body_ref.current;
        if (!body) return;

        if (!isOpened) {
            body.style.maxHeight = "0px";
            return;
        }

        body.style.maxHeight = `${body.scrollHeight}px`;
    }, [isOpened, packages]);

    return (
        <div className={"project-section"}>
            <div className={"project-section-head"} onClick={toggle}>
                <div
                    style={{
                        transform: isOpened ? "rotate(0deg)" : "rotate(-90deg)"
                    }}
                    className={"project-section-head-arrow"}>
                    <svg width="19" height="10" viewBox="0 0 19 10" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M0.421718 0.412777C-0.140573 0.956418 -0.140573 1.83792 0.421718 2.38154L7.46557 9.18545C8.59035 10.2719 10.4129 10.2714 11.5371 9.18461L18.5782 2.37653C19.1406 1.8329 19.1406 0.951407 18.5782 0.407752C18.016 -0.135917 17.1043 -0.135917 16.542 0.407752L10.5155 6.23469C9.95328 6.77845 9.04159 6.77831 8.47934 6.23469L2.45792 0.412777C1.89564 -0.130892 0.983995 -0.130892 0.421718 0.412777Z"
                            fill="#A4A4A4"/>
                    </svg>

                </div>
                <p className={"project-section-head-p"} style={{
                    color: isOpened ? "var(--title)" : "var(--subtitle)"
                }}>Package Options</p>
            </div>

            <div ref={body_ref}
                 className={"project-section-in"}
            >
                {
                    isOpened && packages.map(el => (

                        <Fragment key={el.id}>
                            <p style={{color: "var(--subtitle)"}}>{el.name}({el.id})</p>
                            {
                                (el.startup?.options?.length ?? 0) > 0 &&
                                el.startup?.options?.map(el_ =>
                                    <PackageParameter pack={el.id} key={`${el.id}:${el_.id}`} param={el_}
                                                      set={(v) => write(v, el_.id, el.id)}
                                                      allParams={el.startup.options!}
                                    />
                                )
                            }
                            {
                                !el.startup.options &&
                                <p style={{color: "var(--subtitle)"}}>Not Found</p>
                            }
                        </Fragment>
                    ))
                }
            </div>
        </div>
    )
}