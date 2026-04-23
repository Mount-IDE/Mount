import {useEffect} from "react";
// import reactLogo from "./assets/react.svg";
import {invoke} from "@tauri-apps/api/core";
import "./App.css";
import TitleBar from "./components/common/TitleBar.tsx";
import pageStore from "./stores/page_store.ts";
import {Window} from "./stores/page_store.ts";
import MainPage from "./components/pages/main-page/MainPage.tsx";
import Blur from "./components/common/Blur.tsx";
import {createProjectStore} from "./stores/create_project.ts";
import CreateProject from "./components/pages/create-project/CreateProject.tsx";

function App() {
    const current = pageStore(state => state.current);
    const createProjectOpened = createProjectStore(state=>state.page_opened)
    useEffect(() => {
        setTimeout(() =>
            invoke("show_win").then(), 0)
    }, [])

    return (
        <>
            <Blur/>
            <TitleBar/>
            <div id={"main"}>
                {
                    createProjectOpened&&
                    <CreateProject/>
                }
                {
                    current == Window.Main &&
                    <MainPage/>
                }
            </div>
        </>
    );
}

export default App;
