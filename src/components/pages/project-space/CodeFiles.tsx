import "./styles/code-files.css"
import {fsExtStore} from "../../../stores/fs_ext_store.ts";
import cross from "../../../assets/title-close.svg"
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import {fileCacheStore} from "../../../stores/file_cache_store.ts";
import ContextMenu, {IContextMenuButton} from "../../common/ContextMenu.tsx";
import React, {useEffect, useRef, useState} from "react";
import {computeBP, themeStore} from "../../../stores/theme_store.ts";

type Props = {
    files: Opened[]
    id: number;
    current: [number|null, number],
    setCurrent: (id: number, id2:number)=>void
}


export default function CodeFiles(props: Props) {

    const remove_file = codeSpaceStore(state=>state.remove_file_from_code_space)
    const add_code_space = codeSpaceStore(state => state.add_code_space)
    const add_to_space = codeSpaceStore(state => state.add_file_to_code_space)
    function cb(obj_: Opened){
        remove_file(props.id, obj_);
    }
    function onSelect(obj_: Opened){
        props.setCurrent(obj_.id, obj_.cache_id);
    }

    const theme =
        themeStore(state =>
            state
                .current_theme
                ?.elements
                ?.project_space
                ?.center
                ?.file_list
        )

    console.log(theme)
    const [buttons, setButtons] = useState<IContextMenuButton[]>([])
    const [cords, setCords] = useState<[number, number]>([0, 0])
    const [showContext, setShowContext] = useState(false)


    function onContext(e: React.MouseEvent, obj: Opened) {
        setCords([e.clientX, e.clientY]);
        setShowContext(true)
        setButtons([
            {
                cb: () => {
                    cb(obj)
                },
                hotkeys: "",
                icon: "remove.svg",
                title: "Close"
            }, {
                cb: () => {
                    const {cache_id} = obj;
                    const space_id = add_code_space();
                    add_to_space(space_id, cache_id, obj);
                },
                hotkeys: "",
                title: "Open in Right"
            },
        ])
    }

    const context_ref = useRef<HTMLDivElement>(null)

    useEffect(() => {
        const cur = context_ref.current;
        if (!cur) return

        function hide(e: MouseEvent) {
            if (e.target != cur) {
                setShowContext(false)
            }
        }

        window.addEventListener("click", hide)

        return () => window.removeEventListener("click", hide)
    }, [showContext]);

    let border = computeBP(theme?.this?.border, "border")
    console.log(border)
    return (
        <>
            <div className={"code-space-files"}
                 style={{
                     background: theme?.this?.background,
                     ...border
                 }}
            >
                {props.files.map(el =>
                    <CodeFile onContext={onContext} onSelect={onSelect} onRemove={cb} obj={el}
                              selected={props.current[0] == el.id} key={el.id}/>
                )}
            </div>
            <ContextMenu ref={context_ref} buttons={buttons} x={cords[0]} y={cords[1]} show={showContext}/>
        </>

    )
}


type FileProps = {
    obj: Opened
    selected: boolean
    onRemove: (obj: Opened)=>void
    onSelect: (obj: Opened)=>void
    onContext: (e: React.MouseEvent, obj: Opened) => void
}

function CodeFile(props: FileProps) {

    const name = props.obj.name;
    const get = fsExtStore.getState().get_file_by_name(name);
    const path_to = `/builtin/fs-icons/${get[1]}`
    const from_cache = fileCacheStore(state=>state.get_by_id(props.obj.cache_id));

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.center?.file_list?.element)

    const border = props.selected ? (
        theme?.focus?.border ?
            computeBP(theme.focus.border, "border") :
            {borderBottom: "1px solid var(--border)"}
    ) : {borderBottom: "1px solid transparent"}

    return (
        <div onClick={()=>props.onSelect(props.obj)}
            style={{
                ...border,
                borderRadius: theme?.rounded,
                background: theme?.background,

            }}
             className={"code-space-file"}
             onContextMenu={(e) => {
                 e.stopPropagation();
                 e.preventDefault();
                 props.onContext(e, props.obj)
             }}
        >
            <div className={"code-file-img"}>
                <img src={path_to}/>
            </div>
            {from_cache!=null && from_cache.is_dirty && <p>*</p>}
            <p>{name}</p>
            <button onClick={()=>props.onRemove(props.obj)}>
                <img src={cross}/>
            </button>
        </div>
    )
}