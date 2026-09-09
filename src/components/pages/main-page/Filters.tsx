import "./styles/filters.css"
import search from "../../../assets/search.svg"
import filters from "../../../assets/filters.svg"
import {mainPageStore} from "../../../stores/main_page_store.ts";
import {filterStore} from "../../../stores/filter_store.ts";
import {AnimatePresence, motion} from "motion/react";


export default function Filters() {

    let str = mainPageStore(state => state.filter_string)

    const activated = filterStore(state => state.activated)
    return (
        <div id={"filters"}>
            <div id={"search-filter"}>
                <div>
                    <img src={search}/>
                </div>
                <input value={str}
                       onInput={(e) =>
                           mainPageStore
                               .getState()
                               .set_filter_string(e.currentTarget.value)}
                       placeholder={"Search projects"}
                />
            </div>
            <div id={"filters-filters"} onClick={() => filterStore.getState().set_opened()}>
                <img src={filters}/>
                <AnimatePresence>
                    {
                        activated &&
                        <motion.div
                            initial={{
                                opacity: 0
                            }}
                            exit={{
                                opacity: 0
                            }}
                            animate={{
                                opacity: 1
                            }}

                            transition={{
                                duration: 0.1
                            }}
                            style={{
                                width: "11%",
                                aspectRatio: "1/1",
                                borderRadius: "100%",
                                background: "rgb(43,117,255)",
                                position: "absolute",
                                top: "63%",
                                left: "63%"
                            }}
                        />
                    }
                </AnimatePresence>

            </div>
        </div>
    )
}
