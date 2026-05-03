import "./styles/header.css"


type Props={
    label: string
}



export default function Header(props: Props){
    return (
        <div className={"aside-header"}>
            <p className={"aside--header-p"}>{props.label}</p>
        </div>
    )
}