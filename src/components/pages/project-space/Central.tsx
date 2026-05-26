import "./styles/central.css"
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import CodeSpace from "./CodeSpace.tsx";
import logo from "../../../assets/logo_black_white.svg"



export default function Central() {

    const code_spaces = codeSpaceStore(state=>state.spaces)

    return (
        <div id={"project-central"}>
            {code_spaces.map(el=>
                <CodeSpace key={el.id} obj={el}/>
            )}
            {
                code_spaces.length===0 &&
                <div
                    style={{
                        userSelect:"none",
                        width:"100%",
                        height:"100%",
                        display: "flex",
                        flexDirection:"column",
                        alignItems:"center",
                        justifyContent:"center"
                    }}
                >
                    <div
                        style={{
                            // position:"relative",
                            // left: "calc(50% - 100px)",
                            // top:"70px",
                            display:"flex",
                            width:"200px",
                            height:"200px",
                            flexDirection:"column",
                            alignItems:"center",
                            justifyContent:"center"
                        }}
                    >
                        <img
                            style={{
                                display: "block",
                                width:"150px",
                                height: "150px",
                                objectFit:"contain"
                            }}
                            src={logo}/>
                    </div>
                </div>
            }
        </div>
    )
}