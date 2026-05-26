import "./styles/bottom-header.css"
import {ReactElement} from "react";



type Props={
    part?: ()=>ReactElement,
    title: string
}

export default function BottomHeader(props: Props){
    return (
        <div id={"bottom-header"}>
            <p id={"bottom-header-p"}>{props.title}</p>
        </div>
    )
}