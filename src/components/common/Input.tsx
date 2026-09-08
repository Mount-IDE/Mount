import "./styles/common-parameters.css"
import {useEffect, useRef} from "react";

type Props = {
    typ: "input" | "area"
    value: string,
    title: string
    write: (val: string) => void
    placeholder: string
    show?: boolean
    incorrect?: string
    required?: boolean
}

export default function Input(props: Props) {
    const typ = props.typ;
    const label = props.title;
    const ref = useRef<HTMLTextAreaElement>(null)
    const ref2 = useRef<HTMLInputElement>(null)

    useEffect(() => {
        if (props.typ == "area") {
            const cur = ref.current;
            if (!cur) return

            let tg = cur!
            let height = tg.clientHeight;
            let scroll = tg.scrollHeight;
            if (scroll > height) {
                tg.style.height = `${scroll}px`
            }


        }

    }, [props.typ, props.value]);

    return (
        <div className={"input"}
             style={
                 props.show == false && props.show != undefined ? {
                     opacity: 0.5,
                     pointerEvents: "none"
                 } : {}
             }
        >
            <p className={"project-parameter-input-p"}>{label}</p>
            {typ == "input" &&
                <input ref={ref2}
                       placeholder={props.placeholder}
                       value={props.value}
                       onInput={(e) => props.write(e.currentTarget.value)}
                       required={props.required}
                       readOnly={!props.show && props.show != undefined}
                />
            }
            {
                typ == "area" &&
                <textarea ref={ref}
                          readOnly={!props.show && props.show != undefined}
                          placeholder={props.placeholder}
                          value={props.value}
                          onInput={(e) => props.write(e.currentTarget.value)}
                          required={props.required}
                />
            }
            {(props.incorrect != undefined || (props.value?.length == 0 && props.required)) &&
                <span className={"input-error"}>
                    {props.incorrect}
                </span>
            }
        </div>
    )
}