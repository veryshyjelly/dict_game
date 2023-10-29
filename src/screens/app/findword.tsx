import { Button, Grid, LoadingOverlay, Stack, Text } from "@mantine/core";
import { invoke } from "@tauri-apps/api/tauri";
import { useEffect, useState } from "react";

enum OPT {A, B, C, D, E, F};

const Option = ({text, active, state, onClick} : {text: string | undefined, active: boolean, state: number, onClick: React.MouseEventHandler<HTMLDivElement>}) => {
    return (
        <div className="option" style={{width: 280}} onClick={onClick} key={text}>
            <Button my={40} variant={active ? "light" : "outline" } 
                color={state == 2 ? "teal" : state == 3 ? "red" : "blue"} 
                fullWidth radius={"xl"} size="lg" fw={500}>{text}</Button>
        </div>
    )
}


const ChooseWord = ({score, setScore} : {score: number, setScore: React.Dispatch<React.SetStateAction<number>>}) => {
    const [selected, setSelected] = useState("");
    const [promt, setPrompt] = useState<{
        a: string, b: string, 
        c: string, d: string, 
        e: string, f: string, 
        answer: string, meaning: string}
    >();
    const [loading, setLoading] = useState(false);
    // 1 is normal 2 is correct 3 is incorrect
    const BTS = {a: 1, b: 1, c: 1, d: 1, e: 1, f: 1}
    const [buttonstates, setbuttonstates] = useState(BTS);
    const isactive = (opt: string) => opt === selected;

    const handlebuttonclick = (option: OPT | undefined) => {
        switch (option) {
            case OPT.A: return () => {setSelected("A"); onsubmit("A")} 
            case OPT.B: return () => {setSelected("B"); onsubmit("B")} 
            case OPT.C: return () => {setSelected("C"); onsubmit("C")} 
            case OPT.D: return () => {setSelected("D"); onsubmit("D")} 
            case OPT.E: return () => {setSelected("E"); onsubmit("E")} 
            case OPT.F: return () => {setSelected("F"); onsubmit("F")} 
            default: return () => setSelected("");
        } 
    }

    const fetch_prompt = async () => {
        setLoading(true);
        try {
            let res = await invoke('word_prompt');
            setPrompt(res as { a: string; b: string; c: string; d: string; e: string; f: string; answer: string; meaning: string; } | undefined);
            console.log(res);
        } catch (err) {
            console.log(err);
        }
        setSelected("");
        setbuttonstates(BTS);
        setLoading(false);
    }

    const onsubmit = (sub: string) => {
        if (sub == "") return;
        let setwet = promt?.answer == sub ? 2 : 3;
        let bts =  {...BTS};
        if (setwet == 2) score++;
        setScore(score);
        switch (sub) {
            case "A": bts.a = setwet; break;
            case "B": bts.b = setwet; break;
            case "C": bts.c = setwet; break;
            case "D": bts.d = setwet; break;
            case "E": bts.e = setwet; break;
            case "F": bts.f = setwet; break;
        }
        setbuttonstates(bts);
        setTimeout(() => {if (sub == promt?.answer) {fetch_prompt();}}, 150);
    }

    useEffect(() => {score = 0; fetch_prompt(); setScore(score);}, [])

    return (
        <Stack m={12}>
        <LoadingOverlay visible={loading} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} />
            <Text mx={"auto"} my={20} fw={900} fz={25}>SELECT WORD FOR</Text>

        <div className="prompt" style={{margin: "10px", border: "1px solid lightblue", borderRadius: "30px"}}
         onClick={handlebuttonclick(undefined)}>
            <Text my={20} fw={500} fz={20} mx={60} style={{userSelect: "none"}}>
                {promt?.meaning}
            </Text>
        </div>

        <Grid mx={"auto"} display={"flex"} justify="space-around">
            <Grid.Col span={4}>
                <Option text={promt?.a} active={isactive("A")} onClick={handlebuttonclick(OPT.A)} state={buttonstates.a}/>
                <Option text={promt?.c} active={isactive("C")} onClick={handlebuttonclick(OPT.C)} state={buttonstates.c}/>
            </Grid.Col>
            <Grid.Col span={4}>
                <Option text={promt?.b} active={isactive("B")} onClick={handlebuttonclick(OPT.B)} state={buttonstates.b}/>
                <Option text={promt?.d} active={isactive("D")} onClick={handlebuttonclick(OPT.D)} state={buttonstates.d}/>
            </Grid.Col>
            <Grid.Col span={4}>
                <Option text={promt?.e} active={isactive("E")} onClick={handlebuttonclick(OPT.E)} state={buttonstates.e}/>
                <Option text={promt?.f} active={isactive("F")} onClick={handlebuttonclick(OPT.F)} state={buttonstates.f}/>
            </Grid.Col>
            <Grid.Col display={"flex"}>
            </Grid.Col>
        </Grid>
        </Stack>
    )
}

export default ChooseWord;

