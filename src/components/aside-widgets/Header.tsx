import "./styles/header.css"
import minus from "../../assets/title-wrap.svg"
import {asideStore, comp} from "../../stores/aside_store.ts";
import {useEffect, useMemo, useRef, useState} from "react";
import FsAside from "./FsAside.tsx";
import more from "../../assets/more.svg"
import HeaderOptions from "./HeaderOptions.tsx";

type Props={
    label: string
    is_left: boolean;
}

export interface HeaderOption {
    id?: string
    title?: string
    typ: "check" | "list" | "group",
    def?: IVal,
    radio?: boolean
    inner?: HeaderOption[]
}

function parse(el: comp): HeaderOption[] {
    if (el === FsAside) {
        return [
            {
                title: "Appearance",
                typ: "list",
                inner: [
                    {
                        typ: "group",
                        title: "Show",
                        inner: [
                            {
                                id: "ap:members",
                                typ: "check",
                                title: "Members"
                            }, {
                                id: "ap:ex",
                                typ: "check",
                                title: "Excluded Files"
                            }, {
                                id: "ap:scr",
                                typ: "check",
                                title: "Scratches and Consoles"
                            }, {
                                id: "ap:det",
                                typ: "check",
                                title: "File Details"
                            }, {
                                id: "ap:hide",
                                typ: "check",
                                title: "DotFiles"
                            }
                        ]
                    },
                ]
            },
            {
                title: "Sort by",
                typ: "list",
                inner: [
                    {
                        typ: "group",
                        radio: true,
                        inner: [
                            {
                                typ: "check",
                                title: "Name",
                                id: "sort:name"
                            },
                            {
                                typ: "check",
                                title: "Type",
                                id: "sort:type"
                            }, {
                                typ: "check",
                                title: "Modification time (Newest First)",
                                id: "sort:nt"
                            }, {
                                typ: "check",
                                title: "Modification time (Oldest First)",
                                id: "sort:ot"
                            },
                        ]
                    },
                ]
            }
        ] satisfies  HeaderOption[]
    }

    return []
}

function parseId(el: comp): string {
    if (el == FsAside) {
        return "fs"
    }

    return ""
}


export default function Header(props: Props){

    const minus_ref = useRef<HTMLButtonElement>(null)

    const main_ref = useRef<HTMLDivElement>(null)

    useEffect(() => {
        const main = main_ref.current;
        const minus_ = minus_ref.current;
        if (!main || !minus_) return

        const obs = new ResizeObserver(e => {
            for (let entry of e) {
                if (entry.target == main) {
                    if (entry.contentRect.width > 130) {
                        minus_.style.display = "flex"
                    } else {
                        minus_.style.display = "none"
                    }
                }
            }
        });
        obs.observe(main!);

        return () => {
            obs.disconnect()
        }

    }, [props.is_left, props.label]);


    const current = asideStore(state => props.is_left ? state.current_left : state.current_right)


    const buttons = useMemo(() => parse(current), [current])
    const id = useMemo(() => parseId(current), [current])

    function click() {
        props.is_left ? asideStore.getState().toggle_left(_ => false) : asideStore.getState().toggle_right(_ => false)
    }

    const [opened, setOpened] = useState(false)

    return (
        <div ref={main_ref} className={"aside--header"}>
            <p className={"aside-header-p"}>{props.label}</p>
            <div className={"aside-header-opt"}
                 onMouseEnter={() => setOpened(true)}
                 onMouseLeave={() => setOpened(false)}
            >
                <div className={"aside-header-opt-bt"}>
                    <img src={more}/>
                </div>
                {opened &&
                    <HeaderOptions options={buttons} aside_id={id}/>
                }
            </div>
            <button ref={minus_ref} onClick={click} className={"aside-header-bt"}>
                <img src={minus}/>
            </button>
        </div>
    )
}