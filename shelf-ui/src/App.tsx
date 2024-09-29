import {
  AppLayout,
  TopNavigation,
} from "@cloudscape-design/components";
import {Route, Routes} from "react-router-dom";
import ReposPage from "./pages/ReposPage.tsx";
import Sidenav from "./components/Sidenav.tsx";

function App() {
  return (
    <>
      <TopNavigation identity={{ href: "ui", title: "Shelf" }}></TopNavigation>
      <AppLayout
          contentType={"dashboard"}
        navigation={
            <Sidenav/>
        }
        content={
            <Routes>
                <Route path="/repos" element={<ReposPage/>}/>
            </Routes>
        }
      ></AppLayout>
    </>
  );
}

export default App;
