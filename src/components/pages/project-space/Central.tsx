import "./styles/central.css"
import {codeSpaceStore} from "../../../stores/code_space_store.ts";
import CodeSpace from "./CodeSpace.tsx";
import logo from "../../../assets/logo_black_white.svg"
import {computeBP, themeStore} from "../../../stores/theme_store.ts";


export default function Central() {

    const code_spaces = codeSpaceStore(state=>state.spaces)

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.center);

    const current = theme?.this;


    return (
        <div id={"project-central"}
             style={{
                 background: theme?.this?.background,
                 borderRadius: current?.rounded,
                 ...computeBP(current?.border, "border"),
                 ...computeBP(current?.padding, "padding"),
             }}>
            {code_spaces.map(el=>
                <CodeSpace key={el.id} obj={el} amount={code_spaces.length}/>
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