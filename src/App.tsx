import { useState } from "react";
import "./App.css";
import ChooseWord from "./screens/app/findword";
import { Switch } from "@mantine/core";
import ChooseMeaning from "./screens/app/findmeaning";

function App() {
  const [active, setActive] = useState(false);

  return (
    <div className="container" style={{overflow: "hidden"}}>
      <Switch checked={active} onChange={(e) => setActive(e.currentTarget.checked)} 
        pos={"absolute"} color="blue.4" size="xl" ml={20} mt={20} onLabel="MEANING" offLabel="WORD"/>
      {!active && <ChooseWord/> }     
      {active && <ChooseMeaning/> }     
    </div>
  );
}

export default App;
