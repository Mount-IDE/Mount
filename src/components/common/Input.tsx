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

    /* useEffect(() => {
         if (!props.incorrect) {
             if (props.typ == "input") {
                 let cur = ref2.current!;
                 cur.setCustomValidity("")
                 cur.reportValidity()
             } else {
                 let cur = ref.current!;
                 cur.setCustomValidity("")
                 cur.reportValidity()
             }
             return
         }
         if (props.typ == "input") {
             let cur = ref2.current!;
             cur.setCustomValidity(props.incorrect)
             cur.reportValidity()
         } else {
             let cur = ref.current!;
             cur.setCustomValidity(props.incorrect)
             cur.reportValidity()
         }
     }, [props.incorrect]);
 */
    return (
        <div className={"input"}
             style={
                 props.show == false ? {
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
                       readOnly={!props.show}
                />
            }
            {
                typ == "area" &&
                <textarea ref={ref}
                          readOnly={!props.show}
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