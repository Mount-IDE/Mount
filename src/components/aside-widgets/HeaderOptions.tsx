import {HeaderOption} from "./Header.tsx";
import {useEffect, useState} from "react";
import arrow from "../../assets/arrow.svg"
import check from "../../assets/check.svg"
import {asideStore} from "../../stores/aside_store.ts";
import "./styles/header-options.css"

type Props = {
    options: HeaderOption[]
    aside_id: string
}


export default function HeaderOptions(props: Props) {


    return (
        <div className={"header-options"}>
            {
                props.options.map((el, i) => {
                    let Cmp = parseOptions(el);
                    return <Cmp aside_id={props.aside_id} key={i} obj={el} other={el.radio ? el.inner : undefined}/>
                })
            }
        </div>
    )
}

function parseOptions(el: HeaderOption) {
    if (el.typ == "list") {
        return HeaderList
    }
    if (el.typ == "group") {
        return HeaderGroup
    }
    return HeaderCheck
}

type OptionsProps = {
    obj: HeaderOption,
    other?: HeaderOption[],
    aside_id: string
    i?: number
}


function HeaderList(props: OptionsProps) {

    const [opened, setOpened] = useState(false)


    return (
        <div className={"header-opt-list"}
             onMouseEnter={() => setOpened(true)}
             onMouseLeave={() => setOpened(false)}
        >
            <div className={"header-opt-h"}>
                {props.obj.title}
                <div className={"header-opt-h-img"}
                     style={{
                         transform: "rotate(-90deg)"

                     }}
                >
                    <img style={{
                        width: "50%",
                        height: "50%"
                    }} src={arrow}/>
                </div>
            </div>
            {
                opened &&
                <div className={"header-opt-list-in"}>
                    {props.obj.inner?.map((el, i) => {
                        let Comp = parseOptions(el);
                        return <Comp key={i} obj={el} aside_id={props.aside_id} i={i}
                                     other={props.obj.radio ? props.obj.inner : undefined}
                        />
                    })}
                </div>
            }
        </div>

    )
}


function HeaderCheck(props: OptionsProps) {

    const val = asideStore(state => state.widgets_results[props.aside_id ?? ""]?.[props.obj.id ?? ""]) ?? props.obj.def ?? false;

    function write() {
        if (props.other != undefined) {
            let needed = props.other.filter(el => el.id != props.obj.id).map(el => el.id ?? "")
            asideStore.getState().rewrite_results(props.aside_id, needed, false)
            asideStore.getState().write_results(props.aside_id, props.obj.id ?? "", true)
        } else {
            asideStore.getState().write_results(props.aside_id, props.obj.id ?? "", !val)

        }
    }


    useEffect(() => {
        if (props.other && props.i == 0) {
            write()
        }
    }, [props.other, props.i])



    return (
        <div className={"header-opt-check"} onClick={write}>

            <div className={"header-opt-h-img"}>
                {
                    val == true &&
                    <img style={{
                        width: "70%",
                        height: "70%"
                    }} src={check}/>
                }
            </div>
            <p>{props.obj.title}</p>
        </div>
    )
}


function HeaderGroup(props: OptionsProps) {


    return (
        <div className={"header-opt-group"}>
            {
                props.obj.title &&
                <p className={"header-opt-title"}>{props.obj.title}</p>
            }
            <div className={"header-opt-group-in"}>
                {props.obj.inner?.map((el, i) => {
                    let Comp = parseOptions(el);

                    return <Comp key={i} obj={el} aside_id={props.aside_id} i={i}
                                 other={props.obj.radio ? props.obj.inner : undefined}
                    />
                })}
            </div>
        </div>
    )
}