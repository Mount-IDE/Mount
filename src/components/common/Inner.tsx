import "./styles/inner.css"
import React, {Fragment, ReactElement, useEffect, useLayoutEffect, useRef, useState} from "react";
import arrow from "../../assets/arrow.svg"
import Input from "./Input.tsx";
import FSContext from "./FSContext.tsx";
import Gen from "./Gen.tsx";
import List from "./List.tsx";
import Check from "./Check.tsx";

export type childVal = string | string[] | boolean | null | undefined

export interface Child {
    typ: "text" | "input" | "inner" | "check" | "area" | "list" | "file" | "dir" | "gen"
    value?: childVal
    setValue?: (val: childVal) => void
    meta?: Props
    other_meta?: {
        placeholder?: string
        title?: string
        readonly?: boolean
        required?: boolean
        disabled?: boolean
        variants?: string[]

    }
}


export type Props = {
    show_default?: boolean // true by default
    can_show?: boolean // true by default
    title: string
    children?: React.ReactElement[]
    api_children?: Child[]

}


function parseChild(child: Child): ReactElement {

    if (child.typ == "text") {
        return <p>{child.value}</p>
    }

    if (child.typ == "input" || child.typ == "area") {
        return <Input typ={child.typ}
                      value={child.value?.toString() ?? ""}
                      title={child.other_meta?.title ?? ""}
                      write={child.setValue ?? (() => {
                      })} placeholder={child.other_meta?.placeholder ?? ""}
                      required={child.other_meta?.required}
                      show={!child.other_meta?.readonly}
        />
    }
    if (child.typ == "file" || child.typ == "dir") {
        return <FSContext typ={child.typ}
                          title={child.other_meta?.title ?? ""}
                          value={child.value?.toString() ?? ""}
                          write={child.setValue ?? (() => {
                          })}
                          placeholder={child.other_meta?.placeholder ?? ""}
                          required={child.other_meta?.required}
                          show={!child.other_meta?.readonly}

        />
    }

    if (child.typ == "gen") {
        return <Gen title={child.other_meta?.title ?? ""}
                    value={Array.isArray(child.value) ? child.value : []}
                    write={child.setValue ?? (() => {
                    })}
                    required={child.other_meta?.required}
                    show={!child.other_meta?.readonly}

        />
    }
    if (child.typ == "list") {
        return <List variants={child.other_meta?.variants ?? []}
                     title={child.other_meta?.title ?? ""}
                     value={child.value?.toString() ?? ""}
                     write={child.setValue ?? (() => {
                     })}
                     required={child.other_meta?.required}
                     show={!child.other_meta?.readonly}
        />
    }
    if (child.typ == "check") {
        return <Check value={!!child.value}
                      title={child.other_meta?.title ?? ""}
                      write={child.setValue ?? (() => {
                      })}
                      required={child.other_meta?.required}
                      show={!child.other_meta?.readonly}
        />
    }
    return <Inner {...child.meta as Props}/>
}


export default function Inner(props: Props) {

    const [show, setShow] = useState((props.show_default ?? true))
    const collapsible = props.can_show ?? true;

    const contentRef = useRef<HTMLDivElement>(null) // измеряем этот
    const [maxHeight, setMaxHeight] = useState<string>(show ? "none" : "0px")

    const recalc = () => {
        const content = contentRef.current;
        if (!content) return;
        setMaxHeight(show ? `${content.scrollHeight}px` : "0px");
    }

    useLayoutEffect(() => {
        recalc()
    }, [show, props.api_children, props.children]);

    useEffect(() => {
        const content = contentRef.current;
        if (!content) return;

        const obs = new ResizeObserver(() => {
            if (show) {
                setMaxHeight(`${content.scrollHeight + 20}px`)
            }
        })

        obs.observe(content)

        return () => obs.disconnect()
    }, [show]);


    return (
        <div className={"list-inner"}>
            <div className={"list-inner-head"}
                 onClick={() => {
                     if (collapsible) {
                         setShow(prev => !prev)
                     }
                 }}
            >
                {
                    (props.can_show ?? true) &&
                    <div
                        style={{
                            transform: show ? "rotate(0deg" : "rotate(-90deg)"
                        }}
                        className={"list-inner-head-arrow"}

                    >
                        <img src={arrow}/>
                    </div>
                }
                <p>{props.title}</p>
            </div>
            <div className={"list-inner-body"}
                 style={{
                     maxHeight,
                     overflow: "hidden",
                     transition: "max-height 0.3s"
                 }}
            >
                <div className={"list-inner-body-content"}
                     ref={contentRef}
                >
                    {props.children && props.children}
                    {!props.children && props.api_children &&
                        props.api_children.map((el, i) => {
                            return <Fragment key={i}>{parseChild(el)}</Fragment>
                        })
                    }
                </div>
            </div>
        </div>
    )

}