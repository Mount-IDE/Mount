import {ReactElement} from "react"
import "./styles/common-parameters.css"
import more from "../../assets/more.svg"
import more2 from "../../assets/more2.svg"

type Props = {
    children: ReactElement | ReactElement[],
    limit?: number,
    maxWidth?: number,
    height?: string,
    width?: string,
    title?: string,
    flex?: boolean,
    gap?: string
    align?: string
}


export default function RestrictedList(props: Props) {


    let children = props.limit != undefined ?
        (Array.isArray(props.children) ?
            props.children.slice(0, props.limit) : props.children)
        :
        props.children


    return (
        <div
            style={{
                maxWidth: props.maxWidth != undefined ? `${props.maxWidth}px` : "none",
                width: props.width ? props.width : "auto",
                height: props.height ? props.height : "auto",
                display: props.title ? "block" : "flex",
                flexDirection: "column",
                justifyContent: "center",
                alignItems: "center"
            }}
            className={"restricted-list"}>
            {
                props.title &&
                <p
                    className={"restricted-list-p"}
                >{props.title}</p>
            }
            <div
                className={"restricted-list-in"}
                style={{
                    display: props.flex ? "flex" : "block",
                    gap: props.gap ? props.gap : "none",
                    alignItems: "center",
                    justifyContent: props.align ? props.align : "none",
                    height: props.title ? "auto" : "50%"
                }}>
                {children}
                {
                    props.limit != undefined && Array.isArray(props.children) && props.children.length > props.limit &&
                    <div
                        style={props.height ? {
                            height: props.height
                        } : (props.width ? {
                            width: props.width
                        } : {height: "100%"})}

                        className={"restricted-list-button"}>
                        <img src={props.flex ? more : more2}/>
                    </div>
                }
            </div>

        </div>
    )
}