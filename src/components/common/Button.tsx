import "./styles/button.css"


type Props={
    title: string;
    cb: ()=>void;
    width?:number;
    height?:number;
}
export default function Button(props: Props){


    return (
        <button
            style={{
                width: props.width,
                height: props.height
            }}
            onClick={props.cb}
            className={"button"}>
            {props.title}
        </button>
    )
}