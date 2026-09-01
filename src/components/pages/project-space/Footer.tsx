import "./styles/footer.css"
import {computeBP, themeStore} from "../../../stores/theme_store.ts";
import {projectStore} from "../../../stores/project_store.ts";
import {codeSpaceStore} from "../../../stores/code_space_store.ts";


export default function Footer() {

    const theme = themeStore(state => state.current_theme?.elements?.project_space?.footer)

    const codespace_num = codeSpaceStore(state => state.current);
    const codespace = codeSpaceStore(state => state.get_space(codespace_num))
    let file = codespace?.opened_files?.[codespace?.current_file ?? 0]
    let pack = projectStore(state => state.get_pack_by_file(file?.name ?? null))

    return (
        <div id={"project-footer"}
             style={{
                 background: theme?.background,
                 borderRadius: theme?.rounded,
                 ...computeBP(theme?.border, "border"),
                 ...computeBP(theme?.padding, "padding"),
             }}
        >
            <div id={"project-footer-pack"}>
                {
                    pack &&
                    <p>
                        package: {pack.main.id}
                    </p>
                }
            </div>
        </div>
    )
}