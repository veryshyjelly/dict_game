import { useEffect, useState } from "react";
import "./App.css";
import ChooseWord from "./screens/app/findword";
import { Container  } from "@mantine/core";
import ChooseMeaning from "./screens/app/findmeaning";

function App() {
  const [active, _] = useState(false);
  const [score, setScore] = useState(0);
  const [highscore, sethighScore] = useState(0);

  useEffect(() => {
    sethighScore(Math.max(highscore, score));
  }, [score]);

  return (
    <Container fluid style={{overflow: "hidden"}}>
      {!active && <ChooseWord score={score} setScore={setScore}/> }     
      {active && <ChooseMeaning score={score} setScore={setScore}/> }     
    </Container>
  );
}

export default App;
