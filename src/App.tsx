import {useEffect} from "react";
// import reactLogo from "./assets/react.svg";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import TitleBar from "./components/common/TitleBar.tsx";
import pageStore from "./stores/page_store.ts";
import {Window} from "./stores/page_store.ts";
import MainPage from "./components/pages/main-page/MainPage.tsx";

function App() {
    const current = pageStore(state => state.current);

    useEffect(() => {
        setTimeout(() =>
            invoke("show_win").then(), 0)
    }, [])

    return (
        <>
            <TitleBar/>
            <div id={"main"}>
                {
                    current == Window.Main &&
                    <MainPage/>
                }
            </div>
        </>
    );
}

export default App;
