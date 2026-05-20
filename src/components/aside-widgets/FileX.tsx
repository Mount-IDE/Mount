import "./styles/file.css"

type Props={
    obj: FsFile
}


export default function FileX(props: Props){
    return (
        <div>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}></div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
        </div>
    )
}