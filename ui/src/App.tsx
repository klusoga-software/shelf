import './App.css'
import {AppLayout, TopNavigation} from "@cloudscape-design/components";

function App() {

  return (
      <>
        <TopNavigation identity={{href: "/", title: "Shelf"}}/>
          <AppLayout contentType="dashboard"></AppLayout>
      </>
  )
}

export default App
