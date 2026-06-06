import "./styles/menu-bar-section.css"
import search from "../../assets/search.svg"
import {useState} from "react";
import arrow from "../../assets/arrow.svg"

export interface IMenuBarSection {
    label: string;
    buttons: IMenuBarSectionButton[]
    lists: IMenuBarSectionList[]
    icon?: string
}

interface IMenuBarSectionButton {
    label: string
    searchable: boolean
    cb: () => void,
    icon?: string
}

interface IMenuBarSectionList {
    label: string;
    searchable: boolean;
    elems: IMenuBarSectionListElement[];
}

export interface IMenuBarSectionListElement {
    label: string;
    subtitle?: string;
    icon?: string;
    cb: () => void;
}

type Props = {
    obj: IMenuBarSection
}


export default function MenuBarSection(props: Props) {
    const [searchResult, setSearchResult] = useState("")
    const [opened, setOpened] = useState(false);


    const buttons = props.obj.buttons.filter(el => {
        if (searchResult.length === 0) return true;

        return !el.searchable || el.label.includes(searchResult);
    });
    const lists = props.obj.lists.map(list => ({
        ...list,
        elems: list.searchable && searchResult.length > 0
            ? list.elems.filter(el =>
                el.label.includes(searchResult)
            )
            : list.elems
    }));

    return (
        <div className={"menu-bar-section"}>
            <div className={"menu-bar-section-header"} onClick={() => setOpened(prev => !prev)}>
                {props.obj.icon &&
                    <div className={"menu-bar-section-logo"}>
                        <img src={props.obj.icon!}/>
                    </div>
                }
                {
                    !props.obj.icon &&
                    <div className={"menu-bar-section-cover"}>
                        <p>{props.obj.label.slice(0, 1).toUpperCase()}</p>
                    </div>
                }
                <p className={"menu-bar-section-p"}>{props.obj.label}</p>
                <div className={"menu-bar-section-logo"}>
                    <img src={arrow}/>
                </div>
            </div>


            {opened && <div className={"menu-bar-section-in"}>
                <div className={"menu-bar-section-search"}>
                    <div className={"menu-bar-section-search-img"}>
                        <img src={search}/>
                    </div>
                    <input value={searchResult}
                           onInput={(e) => setSearchResult(e.currentTarget.value)}
                    />
                </div>
                <div className={"menu-bar-section-buttons"}>
                    {buttons.map((el, i) =>
                        <button className={"menu-bar-section-elem"}
                                onClick={el.cb} key={i}
                        >
                            {el.icon &&
                                <div className={"menu-bar-section-elem-img"}>
                                    <img src={`/builtin/context-icons/${el.icon}`}/>
                                </div>
                            }
                            <p>{el.label}</p>
                        </button>
                    )}
                </div>
                <div className={"menu-bar-section-lists"}>
                    {lists.map((el, i) =>
                        <div className={"menu-bar-section-list"} key={i}>
                            <p className={"menu-bar-section-list-p"}>{el.label}</p>
                            {el.elems.map((el_, i_) =>
                                <button className={"menu-bar-section-elem"}
                                        onClick={el_.cb} key={i_}
                                >
                                    {el_.icon &&
                                        <div className={"menu-bar-section-elem-img"}>
                                            <img src={`/builtin/context-icons/${el_.icon}`}/>
                                        </div>
                                    }
                                    {
                                        !el_.icon &&
                                        <div className={"menu-bar-section-elem-cover"}>
                                            <p>{el_.label.slice(0, 1).toUpperCase()}</p>
                                        </div>
                                    }
                                    <p>{el_.label}</p>
                                </button>
                            )}
                        </div>
                    )}
                </div>
            </div>
            }
        </div>
    )
}