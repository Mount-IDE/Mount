import "./styles/template.css"


type Props = {
    template: ITemplate,
    cb: () => void,
    current: boolean
}

export default function Template({template, cb, current}: Props) {
    return (
        <div className={current?"project-template-current": "project-template-p"} onClick={cb}>
            <div className={"project-template-icon"}>
                {template.meta?.icon && <img src={template.meta.icon}/>}
            </div>
            <div className={"project-template-p"}>
                {template.name}
            </div>
        </div>
    )
}