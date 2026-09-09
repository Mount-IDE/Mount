import {motion} from "motion/react"
import "./styles/blur.css"


export default function Blur(){

    // const need = pageStore(state=>state.need_filter);
    return (
        <motion.div
            initial={{opacity: 0}}
            animate={{opacity: 1}}
            exit={{opacity: 0}}

            transition={{duration: 0.2}}
            /* style={{
                 opacity: need? "1":"0",
                 pointerEvents: need? "all":"none"
             }}*/
            className={"blur"}></motion.div>
    )
}