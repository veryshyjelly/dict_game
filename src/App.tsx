import { useEffect, useState } from "react";
import "./App.css";
import ChooseWord from "./screens/app/findword";
import { Container, Group, Modal, Stack, Switch, Text } from "@mantine/core";
import ChooseMeaning from "./screens/app/findmeaning";

const CountDown = (
  { minutes = 0, seconds = 0, miliseconds = 0, score, highscore, setScore, active, setActive } : 
  {minutes : number, seconds: number, miliseconds: number, score: number, highscore: number, setScore: React.Dispatch<React.SetStateAction<number>>,
    active: boolean, setActive: React.Dispatch<React.SetStateAction<boolean>>})  => {
  const [over, setOver] = useState(false);
  const [[min, sec, ms], setTime] = useState([minutes, seconds, miliseconds]);

  const tick = () => {
    if (over) return;
    if (min === 0 && sec === 0 && ms === 0) setOver(true);
    else if (sec === 0 && ms === 0) {
      setTime([min - 1, 59, 599]);
    } else if (ms == 0) {
      setTime([min, sec - 1, 599]);
    } else {
      setTime([min, sec, ms - 1]);
    }
  };  
  
  const reset = () => {
    setTime([minutes, seconds, miliseconds]);
    setOver(false);
    setScore(0);
  };

  useEffect(() => {
    const timerID = setInterval(() => tick(), 1);
    return () => clearInterval(timerID);
  });

  return (
    <>
    {over && 
      <Modal size={"xs"} opened={over} onClose={reset} centered withCloseButton={false} overlayProps={{backgroundOpacity: 0.55, blur: 3}}>
        <Stack display={"flex"} justify="center" align="center">

        <Group display={"flex"} justify="center">
          <Text fw={500}>Score: {score}</Text>
          <Text fw={500}>Highscore: {highscore}</Text>
        </Group>

        <Switch checked={active} onChange={(e) => setActive(e.currentTarget.checked)} 
           color="blue.4" size="xl" mt={10} onLabel="MEANING" offLabel="WORD"/>
        </Stack>
      </Modal>}
    {!over && 
      <Text pos={"absolute"} my={30} ml={800} fw={500} fz={22} style={{letterSpacing: "0.125em"}}>{`${min.toString().padStart(2, '0')}:${sec
        .toString()
        .padStart(2, '0')}:${(ms/100).toPrecision(1).toString().slice(0, 1)}`}</Text>}
    </>
  );
};

function App() {
  const [active, setActive] = useState(false);
  const [score, setScore] = useState(0);
  const [highscore, sethighScore] = useState(0);

  useEffect(() => {
    sethighScore(Math.max(highscore, score));
  }, [score]);

  return (
    <Container fluid style={{overflow: "hidden"}}>
      <CountDown minutes={0} seconds={45} miliseconds={1} score={score} highscore={highscore} setScore={setScore} active={active} setActive={setActive}/>
      <Switch checked={active} onChange={(e) => setActive(e.currentTarget.checked)} 
        pos={"absolute"} color="blue.4" size="xl" ml={20} mt={20} onLabel="MEANING" offLabel="WORD"/>
      {!active && <ChooseWord score={score} setScore={setScore}/> }     
      {active && <ChooseMeaning score={score} setScore={setScore}/> }     
    </Container>
  );
}

export default App;
