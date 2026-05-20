import "./styles/code-files.css"

type Props={
    files: Opened[]
}


export default function CodeFiles(props: Props){




    return (
        <div className={"code-space-files"}>
            {props.files.map((el)=>
                <div key={el.id} className={"code-space-file"}>{el.name}</div>)
            }
        </div>
    )
}