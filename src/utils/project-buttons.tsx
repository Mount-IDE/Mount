import FsAside from "../components/aside-widgets/FsAside.tsx";
import Terminal from "../components/pages/project-space/Terminal.tsx";

function getComponentType(button: IAsideButtonExtended): "Light" | "Heavy" {
    if (button.component_type) return button.component_type;
    return button.widget === "TerminalAside" ? "Heavy" : "Light";
}

function renderButtonComponent(widget: string, active?: boolean) {
    switch (widget) {
        case "FsAside":
            return <FsAside/>;
        case "TerminalAside":
            return <Terminal active={active}/>;
        default:
            return <></>;
    }
}

export function mapProjectButton(button: IAsideButtonExtended): IAsideButton {
    return {
        id: button.order,
        alt: button.alt,
        widget: button.widget,
        component_type: getComponentType(button),
        component: ({active} = {}) => renderButtonComponent(button.widget, active),
        icon: button.icon,
        keys: button.keys,
    };
}

