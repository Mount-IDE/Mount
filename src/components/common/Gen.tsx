import "./styles/common-parameters.css"
import {noteStore, NotificationType} from "../../stores/note_store.ts";
import add from "../../assets/plus.svg";
import close from "../../assets/title-close.svg";


type Props = {
    title: string;
    value: string[];
    write: (val: string[]) => void;
    required?: boolean
    show?: boolean;
    def?: string
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


    return (
        <div className={"gen"}
             style={props.show == false ? {
                 opacity: 0.5,
                 pointerEvents: "none"
             } : {}}
        >
            <p>{props.title}</p>
            <div className={"parameter-gen"}>
                <button className={"parameter-gen-bt"} onClick={add_}>
                    <img src={add}/>
                </button>
                <hr/>
                {
                    list?.map((el, i) =>
                        <div key={i} className={"parameter-gen-el"}>
                            <input value={el}
                                   onInput={
                                       (e) =>
                                           change(
                                               i,
                                               (e.target as HTMLInputElement).value
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