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
                props.obj.parameters.map((el, i) =>
                    <SettingsParameter key={i} obj={el} i={i} cat={props.cat}/>
                )
            }
            <hr/>
        </div>
    )
}