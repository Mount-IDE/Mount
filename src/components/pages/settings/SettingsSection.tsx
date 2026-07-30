import "./styles/settings-section.css"
import SettingsParameter from "./SettingsParameter.tsx";


type Props = {
    obj: ISettingsSection
    i: number
    cat: number
}

export default function SettingsSection(props: Props) {

    return (
        <div className={"settings-section"}>
            {
                props.obj.title != undefined
                &&
                <p className={"settings-section-p"}>{props.obj.title}</p>
            }
            {
                props.obj.parameters.map((el, i) =>
                    <SettingsParameter key={i} obj={el} i={props.obj.id} cat={props.cat}/>
                )
            }
            <hr/>
        </div>
    )
}