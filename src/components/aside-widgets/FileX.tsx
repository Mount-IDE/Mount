import "./styles/file.css"
import {fsExtStore} from "../../stores/fs_ext_store.tsx";

type Props={
    obj: FsFile
}


export default function FileX(props: Props){
    const name = props.obj.name;
    const last_point = name.lastIndexOf(".");
    const ext = name.slice(last_point);
    const ico = fsExtStore.getState().get_file_by_ext(ext);
    const path = `/builtin/fs-icons/${ico[1]}`
    return (
        <div className={"fs-filex"}>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}>
                    {ico[0] &&
                        <img src={path}/>
                    }
                </div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
        </div>
    )
}