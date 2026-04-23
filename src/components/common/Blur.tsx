import "./styles/blur.css"
import pageStore from "../../stores/page_store.ts";





export default function Blur(){

    const need = pageStore(state=>state.need_filter);
    return (
        <div
            style={{
                opacity: need? "1":"0",
                pointerEvents: need? "all":"none"
            }}
            className={"blur"}></div>
    )
}