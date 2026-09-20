import {useMemo} from "react"
import {IHeaderButton} from "./Header"
import "./styles/header-button.css"


type Props = {
    obj: IHeaderButton,
    write_signal: (signal: string, is_save: boolean) => void,
}

export default function HeaderButton(props: Props) {


    const img = useMemo(() => {
        if (props.obj.is_builtin) {
            return `/builtin/aside-icons/headers/${props.obj.icon}`
        } else {
            return ""
        }
    }, [props.obj])


    return (
        <div className="header-button" onClick={() => {
            props.write_signal(props.obj.signal_name, !!props.obj.is_save)
        }}>
            <img src={img} alt={props.obj.name}/>
        </div>
    )
}

