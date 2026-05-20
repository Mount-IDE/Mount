import "./styles/directory.css"
import FileX from "./FileX.tsx";
import arrow from "../../assets/arrow.svg"
import {useLayoutEffect, useRef, useState} from "react";

type Props = {
   obj: FsDirectory
}

export default function DirectoryX(props: Props){

    const directories = props.obj.directories
    const files = props.obj.files

    const [opened, setOpened]=useState(false);

    const ref=useRef<HTMLDivElement>(null);
    //
    // useLayoutEffect(() => {
    //
    //     const div = ref.current;
    //     if (!div)return;
    //     requestAnimationFrame(() => {
    //         div.style.maxHeight = opened
    //             ? `${div.scrollHeight}px`
    //             : "0px";
    //     });
    //
    //
    // }, [opened, directories.length, files.length]);


    return (
        <div className={"fs-dirx"}>
            <div className={"fs-aside-head"}>
                <div className={"fs-aside-icon"}
                     style={{
                     transform: opened? "rotate(0deg)":"rotate(-90deg)"
                     }}
                     onClick={()=>setOpened(prev=>!prev)}
                >
                    <img src={arrow}/>
                </div>
                <div className={"fs-aside-icon"}></div>
                <div className={"fs-aside-name"}>{props.obj.name}</div>
            </div>
            {opened && <div ref={ref} className={"dirx-body"}>
                {directories.map(el=>
                    <DirectoryX obj={el} key={`dir::${el.name}`}/>
                )}
                {files.map(el=>
                    <FileX obj={el} key={`file::${el.name}`}/>
                )}
            </div>}
        </div>
    )
}