import "./styles/directory.css"
import FileX from "./FileX.tsx";
type Props = {
   obj: FsDirectory
}

export default function DirectoryX(props: Props){

    const directories = props.obj.children
        .filter(el=>el.typ=="dir") as FsDirectory[];
    const files = props.obj.children
        .filter(el=>el.typ=="file") as FsFile[];



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