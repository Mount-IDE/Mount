import "./styles/common-parameters.css"
import {noteStore, NotificationType} from "../../stores/note_store.ts";
import add from "../../assets/plus.svg";
import close from "../../assets/title-close.svg";
import Icon from "./Icon.tsx";


type Props = {
    title?: string;
    value: string[];
    write: (val: string[]) => void;
    required?: boolean
    show?: boolean;
    def?: string
    height?: string
}

export default function Gen(props: Props) {

    const list = props.value

    function add_() {
        let res = [...list]
        res.push(props.def ?? "")
        props.write(res)

    }

    function remove(i: number) {
        if (props.required && list.length <= 1) {
            noteStore.getState().add_note({
                text: "Cannot delete last element",
                type: NotificationType.WARN
            }, 2_000)
            return;
        }
        let res = [...list]
        res.splice(i, 1)
        props.write(res);
    }

    function change(i: number, val: string) {

        let res = [...list];
        res[i] = val;
        props.write(res)

        //props.write(list)
    }


    if (typeof props.title == "string") {
        return (
            <div className={"gen"} style={props.show == false ? {
                opacity: 0.5,
                pointerEvents: "none",
                height: props.height
            } : {height: props.height}}>
                <p>{props.title}</p>
                <div className={"parameter-gen"} style={{
                    height: props.height
                }}>
                    <Icon onClick={add_} src={add} size={"70%"}/>
                    <hr/>
                    {
                        list?.map((el, i) =>
                            <div key={i} className={"parameter-gen-el"}>
                                <input value={el}
                                       onInput={
                                           (e) =>
                                               change(
                                                   i,
                                                   (e.target as HTMLInputElement)
                                                       .value
                                               )
                                       }
                                />
                                <button className={""}
                                        onClick={() => remove(i)}
                                >
                                    <img src={close}/>
                                </button>
                            </div>
                        )
                    }
                </div>
            </div>
        )
    }

    return (
        <div className={"parameter-gen"} style={{
            height: props.height
        }}>
            <Icon onClick={add_} src={add} size={"70%"}/>
            <hr/>
            {
                list?.map((el, i) =>
                    <div key={i} className={"parameter-gen-el"}>
                        <input value={el}
                               onInput={
                                   (e) =>
                                       change(
                                           i,
                                           (e.target as HTMLInputElement)
                                               .value
                                       )
                               }
                        />
                        <button className={""}
                                onClick={() => remove(i)}
                        >
                            <img src={close}/>
                        </button>
                    </div>
                )
            }
        </div>
    )
}