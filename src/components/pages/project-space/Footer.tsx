import "./styles/footer.css"
import {computeBP, themeStore} from "../../../stores/theme_store.ts";


export default function Footer() {

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.footer)

    //console.log("FOOTER", theme)
    return (
        <div id={"project-footer"}
             style={{
                 background: theme?.background,
                 borderRadius: theme?.rounded,
                 ...computeBP(theme?.border, "border"),
                 ...computeBP(theme?.padding, "padding"),
             }}
        >

        </div>
    )
}