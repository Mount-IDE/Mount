import "./styles/mini-aside.css"


type Props ={
    is_left?: boolean
}


export default function MiniAside(props: Props) {

    let key = props.is_left? "borderRight" : "borderLeft"

    return (
        <div
           style={{
               [key]: " 1px solid var(--border2)"

           }}
            className={"project-mini-aside"}>
            <div className={"project-mini-aside-top"}>

            </div>
            {
                props.is_left &&
                <div className={"project-mini-aside-bottom"}></div>
            }
        </div>
    )
}