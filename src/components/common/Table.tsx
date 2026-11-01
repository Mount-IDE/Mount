import plus from "@/public/builtin/icons/plus.svg"
import minus from "@/public/builtin/icons/title-wrap.svg"
import arrow_line from "@/public/builtin/icons/arrow-line.svg"
import Icon from "@common/Icon.tsx";
import {Fragment, useState} from "react";

export interface ITableRow {
    data: (string | { type: "text" | "img", val: string })[]
}


type Props = {
    rows: ITableRow[]
    add_row?: () => void
    rem_row?: (i: number) => void
    write_row?: (rows: ITableRow[]) => void

    swap_rows?: (first: number, second: number) => void

    type?: "scroll" | "full"
    child_type?: "scroll" | "trunc"
    child_overflow?: "visible" | "hidden"

    child_size?: string

    style?: string
}

export default function Table(props: Props) {


    const [clicked, setClicked] = useState(-1)

    return (
        <div className={`w-full min-h-[100px] border border-(--border3) ${props.style}`}>
            <div className={"h-[40px] gap-2 w-full flex items-center"}>
                {props.add_row && <Icon src={plus} size={"50%"} onClick={props.add_row}/>}
                {props.rem_row && <Icon src={minus} size={"50%"} onClick={() => props.rem_row?.(clicked)}/>}
                {props.swap_rows && <>
                    <Icon src={arrow_line} size={"50%"} onClick={() => props.swap_rows?.(clicked - 1, clicked)}/>
                    <Icon src={arrow_line} styles={"rotate-180"} size={"50%"}
                          onClick={() => props.swap_rows?.(clicked, clicked + 1)}/>
                </>}
            </div>
            <div className={""} style={{padding: "10px"}}>
                {props.rows.map((el, i) =>
                    <TableRow row={el}
                              onClick={() => {
                                  setClicked(prev => {
                                      if (prev == i) {
                                          return -1
                                      }
                                      return i
                                  })
                              }}
                              key={i}
                              child_size={props.child_size ?? "auto"}
                              child_overflow={props.child_overflow ?? "hidden"}
                              child_type={props.child_type ?? "trunc"}
                              clicked={clicked == i}
                    />
                )}
            </div>
        </div>
    )
}


type RowProps = {
    row: ITableRow,
    onClick: () => void
    child_type: "scroll" | "trunc"
    child_overflow: "visible" | "hidden"
    clicked: boolean
    child_size: string
}

function TableRow(props: RowProps) {

    return (
        <div className={"flex w-full items-center gap-2"}
             style={{
                 background: props.clicked ? "var(--hover)" : "transparent",
                 overflowX: props.child_type == "trunc" ? "hidden" : "auto"
             }}
             onClick={() => props.onClick()}
        >
            {
                props.row.data.map((el2, i2) =>
                    <Fragment key={i2}>
                        {
                            typeof el2 == "object" &&
                            <>
                                {
                                    el2.type == "img" &&
                                    <Icon src={el2.val} size={"80%"}/>
                                }
                            </>
                        }
                        {
                            (typeof el2 != "object" || typeof el2 == "object" && el2.type == "text") &&
                            <p className={"whitespace-nowrap text-(--subtitle) text-[10pt]"}
                               style={{
                                   width: props.child_size,
                                   textOverflow: props.child_overflow == "hidden" ? "ellipsis" : "none",
                                   overflow: props.child_overflow == "hidden" ? "hidden" : "visible",

                               }}
                            >
                                {typeof el2 == "object" && el2.val}
                                {typeof el2 == "string" && el2}
                            </p>
                        }
                    </Fragment>
                )
            }
        </div>
    )
}