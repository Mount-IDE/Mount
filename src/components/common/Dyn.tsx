import "./styles/common-parameters.css"
import {noteStore, NotificationType} from "../../stores/note_store.ts";
import add from "../../assets/plus.svg";
import close from "../../assets/title-close.svg";
import List from "./List.tsx";
import Check from "./Check.tsx";


type Typ = "text" | "input" | "list" | "check" | "base"

export interface ManyVal {
    typ?: Typ,
    val?: IVal
    variants?: string[]
    title?: string
    required?: boolean
    readonly?: boolean
    consturct?: Omit<ManyVal, "construct">[]
}


export type ManyValType = ManyVal | string


type Props = {
    value: ManyValType[],
    write: (val: ManyValType[]) => void,
    title: string,
    show?: boolean
    required?: boolean
    dynamic?: boolean
    def?: ManyValType
    maxWidth?: string

    otherwise?: string
}


export default function Dyn(props: Props) {


    function add_() {
        if (props.def && props.dynamic && props.show) {
            let res = [...props.value]
            res.push(props.def)
            props.write(res)
        }
    }

    function remove(i: number) {
        if (props.show == false && props.dynamic == false) {
            return
        }
        if (props.required && props.value.length <= 1) {
            noteStore.getState().add_note({
                type: NotificationType.ERR,
                text: "Cannot delete last element"
            })
            return
        }
        let res = [...props.value]
        res.splice(i, 1)
        props.write(res)
    }

    function change(i: number, i2: number | null, val: IVal) {
        if (props.show == false) {
            return
        }
        let res = [...props.value]
        if (typeof res[i] == "string") {
            res[i] = val.toString()
        } else {
            let obj: ManyVal = res[i]
            if (!("construct" in obj)) {
                res[i].val = val
            } else {
                res[i].consturct![i2!]!.val = val
            }
        }
        props.write(res)
    }


    function parse(el: ManyVal, i: number, i2: number | null) {
        if (el.typ == "text") {
            return <p>{el.val}</p>
        }
        if (el.typ == "input") {
            return <input
                value={el.val?.toString() ?? ""}
                onInput={(e) => change(i, i2, e.currentTarget.value)}
            />
        }
        if (el.typ == "list") {
            return <List
                variants={el.variants ?? []}
                title={el.title ?? ""}
                value={el.val?.toString() ?? ""}
                write={(e) => change(i, i2, e)}
            />
        }
        if (el.typ == "check") {
            return <Check
                margin={"0"}
                value={!!el.val}
                title={el.title ?? ""}
                write={(e) => change(i, i2, e)}
            />
        }
    }


    return (
        <div className={"gen"}
             style={props.show == false ? {
                 display: props.dynamic ? "block" : "block",
                 opacity: 0.5,
                 pointerEvents: "none",
                 alignItems: props.dynamic ? "none" : "center",
                 maxWidth: props.maxWidth ?? "none"

             } : {
                 display: props.dynamic ? "block" : "flex",
                 alignItems: props.dynamic ? "none" : "center",
                 maxWidth: props.maxWidth ?? "none"
             }}
        >
            <p
                style={{
                    height: props.dynamic ? "auto" : "100%",
                    marginRight: props.dynamic ? "none" : "10px"
                }}
            >{props.title}</p>
            <div className={"parameter-gen"}
                 style={{
                     marginTop: props.dynamic ? "10px" : "none"
                 }}
            >
                {props.dynamic != false &&
                    <>
                        <button
                            className={"parameter-gen-bt"}
                            onClick={add_}
                        >
                            <img src={add}/>
                        </button>
                        <hr/>
                    </>
                }
                {
                    props.value.length > 0 && props.value.map((el, i) => (
                        <div key={i} className={"parameter-gen-el"}>
                            {
                                typeof el == "string" &&
                                <>
                                    <input
                                        value={el}
                                        onInput={(e) =>
                                            change(i, null, e.currentTarget.value)
                                        }
                                    />
                                </>
                            }
                            {
                                typeof el != "string" &&
                                <>
                                    {
                                        "construct" in el &&
                                        el.consturct!.map((el2, i2) => (
                                            parse(el2, i, i2)
                                        ))
                                    }
                                    {
                                        !("construct" in el) &&
                                        parse(el, i, null)
                                    }
                                </>
                            }
                            <button className={""}
                                    onClick={() => remove(i)}
                            >
                                <img src={close}/>
                            </button>
                        </div>
                    ))
                }
                {
                    props.value.length == 0 &&
                    <p>{props.otherwise}</p>
                }
            </div>
        </div>
    )

}