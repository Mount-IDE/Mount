import {ReactElement} from "react";
import close from "../../assets/title-close.svg"
import Icon from "./Icon.tsx";

export interface PageButton {
    text: string
    onClick: (e?: React.MouseEvent) => void
}


export type Props = {
    title: string
    buttons: PageButton[]
    children: ReactElement | ReactElement[]

    can_moved?: boolean
}

export default function Page(props: Props) {


    return (
        <div
            className={"fixed w-75/100 h-75/100 top-25/200 left-25/200 bg-(--bg-t) p-3 rounded-[20px] border border-(--border3)"}>
            <div className={"w-full h-[40px] flex items-center justify-end overflow-hidden"}>
                <p className={""}>{props.title}</p>
                <Icon src={close} onClick={() => {
                }}/>
            </div>
            <div className={"w-full h-8/10"}>
                {props.children}
            </div>
            <div className={"h-[calc(80% - 40px)] w-full flex items-center justify-end overflow-hidden"}
            >

            </div>
        </div>
    )
}