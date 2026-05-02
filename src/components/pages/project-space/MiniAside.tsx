import "./styles/mini-aside.css"
import more2 from "../../../assets/more2.svg"
import {useEffect, useRef} from "react";

type Props ={
    top: IAsideButton[],
    bottom?: IAsideButton[]
}


export default function MiniAside(props: Props) {

    let key = props.bottom? "borderRight" : "borderLeft"

    let top = props.top.length>3? props.top.slice(3):props.top;
    let bottom = (props.bottom && props.bottom.length>3)? props.bottom.slice(3):props.bottom;
    let home_dir = useRef("")
    useEffect(() => {
        async function load(){
            try {

            }catch(e){}
        }
    }, []);
    return (
        <div
           style={{
               [key]: " 1px solid var(--border2)"

           }}
            className={"project-mini-aside"}>
            <div className={"project-mini-aside-top"}>
                <div className={"project-mini-aside-buttons"}>
                    {top.map((el,i)=>
                            <button className={"project-mini-aside-button"} >
                                <img alt={el.alt}
                                    src={
                                        Array.isArray(el.icon)? ( el.icon[0]=="main"? `../../../assets/${el.icon[1]}` : ``) :``
                                    }
                                />
                            </button>
                        )}
                </div>
                <button className={"project-mini-aside-other-buttons"}>
                    <img src={more2}/>
                </button>
            </div>
            {
                props.bottom &&
                <div className={"project-mini-aside-bottom"}>
                    <button className={"project-mini-aside-other-buttons"}>
                        <img src={more2}/>
                    </button>
                    <div className={"project-mini-aside-buttons"}>

                    </div>
                </div>
            }
        </div>
    )
}