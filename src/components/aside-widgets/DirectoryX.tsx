import "./styles/directory.css"
import FileX from "./FileX.tsx";
type Props = {
   obj: FsDirectory
}

export default function DirectoryX(props: Props){

    const directories = props.obj.directories
    const files = props.obj.files



    return (
        <div className={"fs-dirx"}>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}></div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
            <div className={"dirx-body"}>
                {directories.map(el=>
                    <DirectoryX obj={el} key={`dir::${el.name}`}/>
                )}
                {files.map(el=>
                    <FileX obj={el} key={`file::${el.name}`}/>
                )}
            </div>
        </div>
    )
}