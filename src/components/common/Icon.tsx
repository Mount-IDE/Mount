import {MouseEvent as _MouseEvent} from "react"

type Props = {
    src: string
    size?: string
    in_size?: string
    onClick?: (e?: _MouseEvent) => void
}

export default function Icon(props: Props) {


    return (
        <div
            style={{
                height: props.size ? props.size : "80%"
            }}
            className={"aspect-square centered"}
            onClick={props.onClick}
        >
            <img

                style={{
                    width: props.in_size ? props.in_size : "80%",
                    height: props.in_size ? props.in_size : "80%",
                }}

                className={"block object-contain"}
                src={props.src}/>
        </div>
    )
}