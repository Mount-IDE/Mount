import "./styles/common-parameters.css"


type Props = {
    variants: string[]
    title: string
    value: string
    write: (val: string) => void
    show?: boolean
    incorrect?: string
    required?: boolean
}


export default function List(props: Props) {
    const label = props.title;
    /*
        let classes = props.def ? "project-parameter-value project-parameter-list" :
            "project-parameter-value project-parameter-list project-parameter-value-disabled"

    */
    return (
        <div
            className={"list"}
            style={
                props.show == false ? {
                    opacity: 0.5,
                    pointerEvents: "none"
                } : {}
            }
        >
            <select value={props.value}
                    onChange={(e) => props.write(e.currentTarget.value)}>
                {props.variants.map((el, i) =>
                    <option key={i}>{el}</option>
                )}
            </select>
            <p>{label}</p>
        </div>
    )
}