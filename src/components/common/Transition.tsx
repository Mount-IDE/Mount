import {ReactNode} from "react";
import {
    AnimatePresence,
    LegacyAnimationControls,
    motion,
    TargetAndTransition,
    Transition as Tr,
    VariantLabels
} from "motion/react"

type Props = {
    children: ReactNode[] | ReactNode
    initial: TargetAndTransition | VariantLabels | undefined
    exit: TargetAndTransition | VariantLabels | undefined
    animate: TargetAndTransition | VariantLabels | LegacyAnimationControls | undefined
    transition: Tr | undefined
}

export default function Transition(props: Props) {


    return (
        <AnimatePresence>
            <motion.div {...props}
                        transition={{}}
            >
                {props.children}
            </motion.div>
        </AnimatePresence>
    )
}