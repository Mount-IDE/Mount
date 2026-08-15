import "./styles/common-parameters.css"


type Props = {
    value: boolean,
    title: string
    write: (val: boolean) => void
    show?: boolean
    incorrect?: string
    required?: boolean
}

export default function Check(props: Props) {
    const label = props.title;


    // let classes = props.def ? "project-parameter-value project-parameter-check" :
    // "project-parameter-value project-parameter-check project-parameter-value-disabled"

    return (
        <div
            className={"check"}
            style={
                props.show == false ? {
                    opacity: 0.5,
                    pointerEvents: "none"
                } : {}
            }
        >
            <input type={"checkbox"}
                   checked={props.value}
                   onChange={(e) => props.write(e.currentTarget.checked)}/>
            <p>{label}</p>
        </div>
    )
}
