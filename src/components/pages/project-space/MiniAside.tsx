import "./styles/mini-aside.css"
import more2 from "../../../assets/more2.svg"
import {useEffect} from "react";
import AsideButton from "./AsideButton.tsx";
import {cacheStore} from "../../../stores/cache_store.ts";

type Props = {
    top: IAsideButton[],
    bottom?: IAsideButton[],
    max_top?: number
    max_bot?: number
}


export default function MiniAside(props: Props) {

    let key = props.bottom ? "borderRight" : "borderLeft"

    let top = props.max_top !== undefined ?
        props.top.length > props.max_top ?
            props.top.slice(props.max_top)
            : props.top
        : props.top
    let bottom = props.max_bot !== undefined ?
        (props.bottom && props.bottom.length > props.max_bot) ? props.bottom.slice(props.max_bot)
            : props.bottom
        : props.bottom;
    let data_dir = cacheStore(state => state.data_dir)

    console.log(top, bottom, props.bottom)
    useEffect(() => {
        async function load() {
            try {

            } catch (e) {
            }
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
                    {top.map((el) =>
                        <AsideButton selected={false} bt={el} key={el.id} data_dir={data_dir}/>
                    )}
                </div>
                {
                    props.max_top !== undefined &&
                    <button className={"project-mini-aside-other-buttons"}>
                        <img src={more2}/>
                    </button>
                }
            </div>
            {
                props.bottom &&
                <div className={"project-mini-aside-bottom"}>
                    {
                        props.max_bot !== undefined &&
                        <button className={"project-mini-aside-other-buttons"}>
                            <img src={more2}/>
                        </button>}
                    <div className={"project-mini-aside-buttons"}>
                        {bottom!.map((el) =>
                            <AsideButton selected={false} bt={el} key={el.id} data_dir={data_dir}/>
                        )}
                    </div>
                </div>
            }
        </div>
    )
}