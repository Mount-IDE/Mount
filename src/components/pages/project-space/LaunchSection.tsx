import "./styles/launch-section.css"
import LaunchOption from "./LaunchOption.tsx";

type Props = {
    obj: ISection
    functions: LaunchFunction[]
    cur_ref: number,
    project: IProject | null
}


export default function LaunchSection(props: Props) {
    const {obj} = props;
    return (
        <div
            style={{
                borderBottom: obj.label == null ? "1px solid var(--border2)" : "1px solid transparent",
                padding: "20px"
            }}
            className={"launch-section"}>
            {
                obj.label !== undefined && obj.label !== null &&
                <div className={"launch-section-head"}>
                    <p>{obj.label}</p>
                </div>
            }
            <div
                className={"launch-section-options"}>
                {
                    obj.params.map((el, i) =>
                        <LaunchOption {...props} functions={props.functions} section={obj.id} obj={el} key={i}/>
                    )
                }
            </div>
        </div>
    )
}