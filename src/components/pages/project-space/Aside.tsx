import "./styles/aside.css"


type Props ={
    left?:boolean
}

export default function Aside(props: Props) {

    let key=props.left? "borderRight":"borderLeft"
    return (
        <div
            style={{
                [key]: "1px solid var(--border2)"
            }}
            className={"project-aside"}>
        </div>
    )
}