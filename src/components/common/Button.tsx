import "./styles/button.css"


type Props={
    title: string;
    cb: ()=>void;
    width?:number;
    height?:number;
    is_main?:boolean;
}
export default function Button(props: Props){


    return (
        <button
            style={{
                width: props.width,
                height: props.height
            }}
            onClick={props.cb}
            className={props.is_main?"button-main button":"button"}>
            {props.title}
        </button>
    )
}