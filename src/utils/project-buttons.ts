import FsAside from "../components/aside-widgets/FsAside.tsx";
import Terminal from "../components/pages/project-space/Terminal.tsx";
import AsideLaunch from "../components/pages/project-space/AsideLaunch.tsx";
import React from "react";


const REGISTRY = {
    FsAside: FsAside,
    TerminalAside: Terminal,
    AsideLaunch: AsideLaunch
} satisfies Record<string, React.ComponentType<{ active?: boolean }>>

type WIDGET = keyof typeof REGISTRY

/*function getComponentType(button: IAsideButtonExtended) {
    return button.component_type
}*/


export function mapProjectButton(button: IAsideButtonExtended): IAsideButton {
    const Component = REGISTRY[button.widget as WIDGET] ?? (() => null)
    return {
        id: button.order,
        alt: button.alt,
        widget: button.widget,
        component_type: button.component_type ?? "Light",
        component: Component,
        icon: button.icon,
        keys: button.keys
    }
}