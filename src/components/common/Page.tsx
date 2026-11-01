import {ReactElement, useEffect} from "react";
import close from "../../assets/title-close.svg"
import minus from "../../assets/title-wrap.svg"
import Icon from "./Icon.tsx";
import {
    ModalApiExtended,
    ModalApiShorted,
    ModalPageState,
    modalPageStore,
    RemoveEvent,
    WriteEvent
} from "../../stores/modal_page_store.ts";

import {modalPageExtendedService, modalPageService} from "../../services/page_service.ts";
import Button from "./Button.tsx";

export interface PageButton {
    text: string
    onClick: (api: ModalApiExtended, e?: React.MouseEvent) => void
}


export type Props = {
    title: string
    buttons: PageButton[]
    children?: ReactElement | ReactElement[]
    component?: React.ComponentType<{ api: ModalApiShorted }>
    can_moved?: boolean
}


const modalPageServiceShortedService: ModalApiShorted = {
    get_saved(field: string): IValExtended | null {
        const current = modalPageStore.getState().current;
        if (!current) return null
        return modalPageService.get_saved(current, field)
    },

    get_unsaved(field: string): IValExtended | null {
        const current = modalPageStore.getState().current;
        if (!current) return null
        return modalPageService.get_unsaved(current, field)
    },

    async save(): Promise<WriteEvent> {
        const current = modalPageStore.getState().current;
        if (!current) return WriteEvent.NotFound
        return modalPageService.save(current)

    },

    write_unsaved(field: string, value: IValExtended): boolean {
        const current = modalPageStore.getState().current;
        if (!current) return false
        return modalPageService.write_unsaved(current, field, value)
    },

    write_unsaved_obj(obj: Record<string, IValExtended>): boolean {
        const current = modalPageStore.getState().current;
        if (!current) return false
        return modalPageService.write_unsaved_obj(current, obj)
    },

    get_obj: () => {
        let current = modalPageStore.getState().current;
        if (!current) return null
        let res = modalPageStore.getState().pages[current]

        function intersection(obj1: any, obj2: any) {
            let obj = obj1;

            for (let i of Object.keys(obj)) {
                if (typeof obj2 == "object") {
                    if (!Object.keys(obj2).includes(i)) continue
                }
                let obj2_ = typeof obj2 == "object" ? obj2[i] : obj2
                let obj1_ = typeof obj == "object" ? obj[i] : obj
                if (typeof obj1_ == "object") {
                    obj = {...intersection(obj1_, obj2_)}
                }
                obj[i] = obj2_
            }
            return {...obj}
        }

        let obj = intersection(res.default_data, res.unsaved_data)

        return obj
    }

}

export default function Page() {

    const pages = modalPageStore(state => state.pages)
    const currentPageId = modalPageStore(state => state.current)
    const currentPage = currentPageId ? pages[currentPageId] as ModalPageState | undefined : undefined
    const props = currentPage?.props

    let Cmp = currentPage?.props?.component

    useEffect(() => {
        console.log("RERENDER(")
    }, [])

    return (
        <div
            className={"absolute w-75/100 z-101 h-75/100 top-25/200 left-25/200 bg-(--bg-t) rounded-[20px] border border-(--border3)"}
            style={{
                padding: "15px"
            }}
        >
            <div className={"w-full h-1/10 flex items-center justify-end gap-2 overflow-hidden p-10 relative"}>
                <p className={"text-(--subtitle) text-[10pt] mr-10"}>{props?.title}</p>
                <Icon src={minus} size={"30%"} styles={"opacity-50"} onClick={() => {
                    modalPageExtendedService.hide()
                }}/>
                <Icon src={close} size={"20%"} styles={"opacity-50"} onClick={() => {
                    let removed = modalPageExtendedService.remove_current_save()
                    if (removed == RemoveEvent.Unsaved) {
                        // open modal
                    }
                }}/>
            </div>
            <div className={"w-full h-8/10"}>
                {
                    Cmp &&
                    <Cmp api={modalPageServiceShortedService}/>
                }
                {
                    props?.children && !props?.component &&
                    props?.children
                }

            </div>
            <div className={"h-1/10 w-full flex items-center justify-end overflow-hidden"}
            >
                {
                    props?.buttons.slice(0, 4).map((el, i) =>
                        <Button title={el.text} cb={() => el.onClick(modalPageExtendedService)} key={i}/>
                    )
                }
            </div>
        </div>
    )
}