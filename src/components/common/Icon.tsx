import {MouseEvent as _MouseEvent, RefObject} from "react"

type Props = {
    src: string
    size?: string
    in_size?: string
    onClick?: (e?: _MouseEvent) => void
    modifier?: string
    rotate?: string
    styles?: string
    disabled?: boolean

    ref?: RefObject<HTMLDivElement | null>
}

export default function Icon(props: Props) {


    return (
        <div
            ref={props.ref}
            style={{
                [props.modifier ?? "height"]: props.size ? props.size : "80%"
            }}
            className={`aspect-square centered select-none  ${props.styles ?? ""}`}
            onClick={(e) => props.disabled != true && props.onClick?.(e)}
        >
            <img

                style={{
                    rotate: props.rotate,
                    width: props.in_size ? props.in_size : "80%",
                    height: props.in_size ? props.in_size : "80%",
                }}

                className={"block object-contain"}
                src={props.src}/>
        </div>
    )
}